use std::io;
use tinf::Desc;
use {terminal_mode, Color, Handle, LockableStream, Result, Stream,
     TerminalMode, DoStyle};

/// A styled stream using terminfo escape sequences.
///
/// Instances are lightweight, using only three escape sequences
/// (instead of copying entire terminfo descriptions).
pub struct TermStream<T> {
    cap_reset: Vec<u8>,
    cap_fg: Vec<u8>,
    cap_em: Vec<u8>,
    w: T,
}

impl TermStream<io::Stdout> {
    /// A `TermStream` that wraps `std::io::stdout()`, using the
    /// current terminal description.
    pub fn stdout(do_style: DoStyle) -> TermStream<io::Stdout> {
        let mode = terminal_mode(Handle::Stdout);
        TermStream::std(mode, io::stdout(), do_style)
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
    /// A `TermStream` that wraps `std::io::stderr()`, using the
    /// current terminal description.
    pub fn stderr(do_style: DoStyle) -> TermStream<io::Stderr> {
        let mode = terminal_mode(Handle::Stderr);
        TermStream::std(mode, io::stderr(), do_style)
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
    /// Create a `TermStream` that wraps a `std::io::Write`, using the
    /// terminfo description given by `desc`.
    ///
    /// If `do_style` is false, or `desc[sgr0]` is empty, the `Stream`
    /// methods (`reset`, `fg`, and `em`) will have no effect.
    pub fn new(w: T, desc: &Desc, do_style: bool) -> TermStream<T> {
        use tinf::cap;

        let do_style = do_style && !desc[cap::sgr0].is_empty();
        match do_style {
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

    pub(super) fn std(
        mode: TerminalMode,
        w: T,
        do_style: DoStyle,
    ) -> TermStream<T> {
        use self::TerminalMode::*;
        use self::DoStyle::*;

        match mode {
            #[cfg(windows)]
            None => TermStream::init(w),
            Redir => TermStream::new(w, Desc::current(), do_style == Always),
            Term => TermStream::new(w, Desc::current(), do_style != Never),
            #[cfg(windows)]
            Console => TermStream::init(w),
            #[cfg(windows)]
            Win10 => TermStream {
                cap_reset: b"\x1b[0m".to_vec(),
                cap_fg: b"\x1b[3%p1%dm".to_vec(),
                cap_em: b"\x1b[1m".to_vec(),
                w,
            }
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

/// The `Stream` methods of this implementation may return
/// an`Err(`[`Error`](struct.Error.html)`)` if there is an I/O error
/// writing to the wrapped stream, or if the `TermStream` object was
/// created from a broken terminfo description with invalid escape
/// sequences.
impl<T: io::Write> Stream for TermStream<T> {
    // Note that vars is not persistent between calls to tparm; for
    // the escape sequences we use, this is fine unless you are using
    // certain ancient terminals made by Data General or Wyse.

    /// Sends the `sgr0` escape sequence.
    fn reset(&mut self) -> Result<()> {
        ::tinf::tparm(
            &mut self.w,
            &self.cap_reset,
            &mut params!(),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }

    /// Sends the `setaf` escape sequence.
    fn fg(&mut self, fg: Color) -> Result<()> {
        ::tinf::tparm(
            &mut self.w,
            &self.cap_fg,
            &mut params!(fg.0),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }

    /// Sends the `bold`, `smul`, or `smso` escape sequence (depending
    /// on which is supported by the terminfo description).
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
