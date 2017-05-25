use std::io;
use tinf::Desc;
use {terminal_mode, Color, Handle, LockableStream, Result, Stream,
     TerminalMode, UseColor};

pub struct TermStream<T> {
    cap_reset: Vec<u8>,
    cap_fg: Vec<u8>,
    cap_em: Vec<u8>,
    w: T,
}

impl TermStream<io::Stdout> {
    pub fn stdout(use_color: UseColor) -> TermStream<io::Stdout> {
        TermStream::std(Handle::Stdout, io::stdout(), use_color)
    }
}

// Thanks to /u/cbreeden for help with lifetimes.
impl LockableStream for TermStream<io::Stdout> {
    fn lock<'a>(&'a self) -> Box<Stream + 'a> {
        let locked = TermStream {
            cap_reset: self.cap_reset.clone(),
            cap_fg: self.cap_fg.clone(),
            cap_em: self.cap_em.clone(),
            w: self.w.lock(),
        };
        Box::new(locked)
    }
}

impl TermStream<io::Stderr> {
    pub fn stderr(use_color: UseColor) -> TermStream<io::Stderr> {
        TermStream::std(Handle::Stderr, io::stderr(), use_color)
    }
}

impl LockableStream for TermStream<io::Stderr> {
    fn lock<'a>(&'a self) -> Box<Stream + 'a> {
        let locked = TermStream {
            cap_reset: self.cap_reset.clone(),
            cap_fg: self.cap_fg.clone(),
            cap_em: self.cap_em.clone(),
            w: self.w.lock(),
        };
        Box::new(locked)
    }
}

impl<T: io::Write> TermStream<T> {
    pub fn new(w: T, desc: &Desc, color: bool) -> TermStream<T> {
        use tinf::cap;

        let color = color && !desc[cap::sgr0].is_empty();
        match color {
            true => {
                TermStream {
                    cap_reset: desc[cap::sgr0].to_vec(),
                    cap_fg: desc[cap::setaf].to_vec(),
                    cap_em: get_em(desc),
                    w,
                }
            }
            false => TermStream::init(w),
        }
    }

    fn std(handle: Handle, w: T, use_color: UseColor) -> TermStream<T> {
        use self::TerminalMode::*;
        use self::UseColor::*;

        match terminal_mode(handle) {
            None => TermStream::init(w),
            Redir => TermStream::new(w, Desc::current(), use_color == Always),
            Term => TermStream::new(w, Desc::current(), use_color != Never),
            #[cfg(windows)]
            Win10 => unimplemented!(),
        }

    }

    fn init(w: T) -> TermStream<T> {
        TermStream {
            cap_reset: Vec::new(),
            cap_fg: Vec::new(),
            cap_em: Vec::new(),
            w,
        }
    }
}

impl<T: io::Write> io::Write for TermStream<T> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.w.write(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}

impl<T: io::Write> Stream for TermStream<T> {
    // Note that vars is not persistent between calls to tparm; for
    // the capabilities we use, this is fine unless you are using
    // certain ancient terminals made by Data General or Wyse.

    fn reset(&mut self) -> Result<()> {
        ::tinf::tparm(
            &mut self.w,
            &self.cap_reset,
            &mut params!(),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        ::tinf::tparm(
            &mut self.w,
            &self.cap_fg,
            &mut params!(fg.0),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }

    fn em(&mut self) -> Result<()> {
        ::tinf::tparm(
            &mut self.w,
            &self.cap_em,
            &mut params!(),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }
}

fn get_em(desc: &Desc) -> Vec<u8> {
    use tinf::cap;

    let ncv = match desc[cap::ncv] {
        0xffff => 0,
        n => n,
    };
    let color = desc[cap::colors] != 0xffff && desc[cap::colors] != 0;
    if !desc[cap::bold].is_empty() && !(color && (ncv & 0x20 != 0)) {
        return desc[cap::bold].to_vec();
    }
    if !desc[cap::smul].is_empty() && !(color && (ncv & 0x02 != 0)) {
        return desc[cap::smul].to_vec();
    }
    if !desc[cap::smso].is_empty() && !(color && (ncv & 0x01 != 0)) {
        return desc[cap::smso].to_vec();
    }

    Vec::new()
}