use std::io::{self, Write};
use tinf::Desc;
use tvis_util::{size, Handle, TerminalMode};
use {CLIStream, Color, DoStyle, LockableStream, Result, Stream, WinSize};

/// A styled stream using terminfo escape sequences.
///
/// Instances are lightweight, using only four escape sequences
/// (instead of copying entire terminfo descriptions).
pub struct TermStream<T> {
    cap_reset: Vec<u8>,
    cap_fg: Vec<u8>,
    cap_em: Vec<u8>,
    cap_rewind: Vec<u8>,
    w: T,
    defsz: WinSize,
    mode: TerminalMode,
}

impl TermStream<io::Stdout> {
    /// A `TermStream` that wraps `std::io::stdout()`, using the
    /// current terminal description.
    pub fn stdout(do_style: DoStyle) -> TermStream<io::Stdout> {
        let mode = Handle::Stdout.terminal_mode();
        TermStream::std(mode, io::stdout(), do_style)
    }
}

impl TermStream<io::Stderr> {
    /// A `TermStream` that wraps `std::io::stderr()`, using the
    /// current terminal description.
    pub fn stderr(do_style: DoStyle) -> TermStream<io::Stderr> {
        let mode = Handle::Stderr.terminal_mode();
        TermStream::std(mode, io::stderr(), do_style)
    }
}

// Thanks to /u/cbreeden for help with lifetimes.
impl LockableStream for TermStream<io::Stdout> {
    fn lock<'a>(&'a self) -> Box<CLIStream + 'a> {
        let locked = TermStream {
            cap_reset: self.cap_reset.clone(),
            cap_fg: self.cap_fg.clone(),
            cap_em: self.cap_em.clone(),
            cap_rewind: self.cap_rewind.clone(),
            w: self.w.lock(),
            defsz: self.defsz,
            mode: self.mode,
        };
        Box::new(locked)
    }
}

impl LockableStream for TermStream<io::Stderr> {
    fn lock<'a>(&'a self) -> Box<CLIStream + 'a> {
        let locked = TermStream {
            cap_reset: self.cap_reset.clone(),
            cap_fg: self.cap_fg.clone(),
            cap_em: self.cap_em.clone(),
            cap_rewind: self.cap_rewind.clone(),
            w: self.w.lock(),
            defsz: self.defsz,
            mode: self.mode,
        };
        Box::new(locked)
    }
}

impl<T: io::Write> TermStream<T> {
    fn new(
        w: T,
        desc: &Desc,
        do_style: bool,
        mode: TerminalMode,
    ) -> TermStream<T> {
        use self::TerminalMode::*;
        use tinf::cap;

        let mut term_stream = TermStream {
            cap_reset: Vec::new(),
            cap_fg: Vec::new(),
            cap_em: Vec::new(),
            cap_rewind: if mode == Redir {
                Vec::new()
            } else {
                desc[cap::cuu].to_vec()
            },
            w,
            defsz: get_default_size(mode, desc),
            mode: mode,
        };
        let do_style = do_style && !desc[cap::sgr0].is_empty();
        if do_style {
            term_stream.cap_reset = desc[cap::sgr0].to_vec();
            term_stream.cap_fg = desc[cap::setaf].to_vec();
            term_stream.cap_em = get_em(desc);
        }
        term_stream
    }

    pub(super) fn std(
        mode: TerminalMode,
        w: T,
        do_style: DoStyle,
    ) -> TermStream<T> {
        use self::TerminalMode::*;
        use self::DoStyle::*;

        let use_style = match mode {
            Redir => do_style == Always,
            _ => do_style != Never,
        };

        match mode {
            #[cfg(windows)]
            Win10 => TermStream {
                cap_reset: b"\x1b[0m".to_vec(),
                cap_fg: b"\x1b[3%p1%dm".to_vec(),
                cap_em: b"\x1b[1m".to_vec(),
                cap_rewind: b"\x1b[%p1%dA".to_vec(),
                w,
                defsz: size::get_default_console_size(),
                mode: TerminalMode::Win10,
            },
            _ => TermStream::new(w, Desc::current(), use_style, mode),
        }
    }

    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        if self.mode == TerminalMode::Redir {
            panic!("cli method called on redirected stream");
        }
        self.flush()?;
        if count > 0 {
            self.write_all(&[b'\r'])?;
        }
        if count > 1 {
            ::tinf::tparm(
                &mut self.w,
                &self.cap_rewind,
                &mut params!(count - 1),
                &mut ::tinf::Vars::new(),
            )?;
        }
        Ok(())
    }

    fn get_size(&self, handle: Handle) -> WinSize {
        if self.mode == TerminalMode::Redir {
            panic!("cli method called on redirected stream");
        }
        match size::get_size(handle) {
            Some(sz) => sz,
            None => self.defsz,
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

    fn is_cli(&self) -> bool {
        self.mode != TerminalMode::Redir
    }
}

fn get_default_size(mode: TerminalMode, desc: &Desc) -> WinSize {
    #[cfg(windows)]
    use self::TerminalMode::*;
    use tinf::cap;

    match mode {
        #[cfg(windows)]
        Win10 | Console => size::get_default_console_size(),
        _ => {
            let cols = match desc[cap::cols] {
                0 | 0xffff => 80,
                v => i32::from(v),
            };
            let rows = match desc[cap::lines] {
                0 | 0xffff => 24,
                v => i32::from(v),
            };
            WinSize { cols, rows }
        }
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


impl CLIStream for TermStream<io::Stdout> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        // if cygwin, call with locked self.w else below
        self.get_size(Handle::Stdout)
    }
}

impl CLIStream for TermStream<io::Stderr> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        // if cygwin, call with locked self.w?
        self.get_size(Handle::Stderr)
    }
}

impl<'a> CLIStream for TermStream<io::StdoutLock<'a>> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        // if cygwin, call with self.w?
        self.get_size(Handle::Stdout)
    }
}

impl<'a> CLIStream for TermStream<io::StderrLock<'a>> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        // if cygwin, call with self.w?
        self.get_size(Handle::Stderr)
    }
}
