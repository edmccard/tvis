#![cfg(windows)]

use std::io::{self, Write};
use tvis_util::{Handle, ConsoleMode};
use tvis_util::size;
use win32;
use {Color, LockableStream, Result, CLIStream, Stream, DoStyle, WinSize};


/// A styled stream using the Windows Console API.
pub struct ConStream<T> {
    w: T,
    hndl: win32::Handle,
    orig_pair: CPair,
    defsz: WinSize,
    do_style: bool,
}

impl ConStream<io::Stdout> {
    /// A `ConStream` that wraps `std::io::stdout()`.
    pub fn stdout(do_style: DoStyle) -> ConStream<io::Stdout> {
        ConStream::init(io::stdout(), Handle::Stdout, do_style)
    }
}

impl ConStream<io::Stderr> {
    /// A `ConStream` that wraps `std::io::stderr()`.
    pub fn stderr(do_style: DoStyle) -> ConStream<io::Stderr> {
        ConStream::init(io::stderr(), Handle::Stderr, do_style)
    }
}

impl<T: Write> ConStream<T> {
    fn init(w: T, handle: Handle, do_style: DoStyle) -> ConStream<T> {
        let hndl = unsafe { win32::GetStdHandle(handle as u32) };
        let is_atty = handle.console_mode() != ConsoleMode::None;
        let do_style = is_atty && (do_style != DoStyle::Never);
        let orig_pair = match do_style {
            true => *ORIG_PAIR,
            false => (7, 0),
        };
        ConStream {
            w,
            hndl,
            orig_pair,
            defsz: size::get_default_console_size(),
            do_style,
        }
    }

    fn reset(&mut self) -> Result<()> {
        if self.do_style {
            self.flush()?;
            let orig = self.orig_pair;
            set_colors(self.hndl, orig);
        }
        Ok(())
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        if self.do_style {
            self.flush()?;
            let cur = get_colors(self.hndl);
            let em_mask = cur.0 & win32::FOREGROUND_INTENSITY;
            set_colors(self.hndl, (fg.1 | em_mask, cur.1));
        }
        Ok(())
    }

    fn em(&mut self) -> Result<()> {
        if self.do_style {
            self.flush()?;
            let cur = get_colors(self.hndl);
            let fg = cur.0 | win32::FOREGROUND_INTENSITY;
            set_colors(self.hndl, (fg, cur.1));
        }
        Ok(())
    }

    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        let mut csbi: win32::ConsoleScreenBufferInfo = Default::default();
        unsafe {
            win32::GetConsoleScreenBufferInfo(self.hndl, &mut csbi);
        }
        self.flush()?;
        let mut coord = csbi.cursor_position;
        if count > 0 {
            coord.x = 0;
        }
        if count > 1 {
            coord.y -= ::std::cmp::min(count - 1, coord.y as u16) as i16;
        }
        unsafe {
            win32::SetConsoleCursorPosition(self.hndl, coord);
        }
        Ok(())
    }

    fn get_size(&self, handle: Handle) -> WinSize {
        match size::get_size(handle) {
            Some(sz) => sz,
            None => self.defsz,
        }
    }
}

impl<T: Write> Write for ConStream<T> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.w.write(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}

impl Stream for ConStream<io::Stdout> {
    fn reset(&mut self) -> Result<()> {
        let _ = self.w.lock();
        self.reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        let _ = self.w.lock();
        self.fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        let _ = self.w.lock();
        self.em()
    }

    fn is_cli(&self) -> bool {
        true
    }
}

impl Stream for ConStream<io::Stderr> {
    fn reset(&mut self) -> Result<()> {
        let _ = self.w.lock();
        self.reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        let _ = self.w.lock();
        self.fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        let _ = self.w.lock();
        self.em()
    }

    fn is_cli(&self) -> bool {
        true
    }
}

impl<'a> Stream for ConStream<io::StdoutLock<'a>> {
    fn reset(&mut self) -> Result<()> {
        self.reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        self.fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        self.em()
    }

    fn is_cli(&self) -> bool {
        true
    }
}

impl<'a> Stream for ConStream<io::StderrLock<'a>> {
    fn reset(&mut self) -> Result<()> {
        self.reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        self.fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        self.em()
    }

    fn is_cli(&self) -> bool {
        true
    }
}

impl CLIStream for ConStream<io::Stdout> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        let _ = self.w.lock();
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        self.get_size(Handle::Stdout)
    }
}

impl CLIStream for ConStream<io::Stderr> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        let _ = self.w.lock();
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        self.get_size(Handle::Stderr)
    }
}

impl<'a> CLIStream for ConStream<io::StdoutLock<'a>> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        self.get_size(Handle::Stdout)
    }
}

impl<'a> CLIStream for ConStream<io::StderrLock<'a>> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        self.rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        self.get_size(Handle::Stderr)
    }
}

impl LockableStream for ConStream<io::Stdout> {
    fn lock<'a>(&'a self) -> Box<CLIStream + 'a> {
        let locked = ConStream {
            w: self.w.lock(),
            hndl: self.hndl,
            orig_pair: self.orig_pair,
            defsz: self.defsz,
            do_style: self.do_style,
        };
        Box::new(locked)
    }
}

impl LockableStream for ConStream<io::Stderr> {
    fn lock<'a>(&'a self) -> Box<CLIStream + 'a> {
        let locked = ConStream {
            w: self.w.lock(),
            hndl: self.hndl,
            orig_pair: self.orig_pair,
            defsz: self.defsz,
            do_style: self.do_style,
        };
        Box::new(locked)
    }
}


type CPair = (u16, u16);

// Assumes `hndl` is a console screen buffer.
fn get_colors(hndl: win32::Handle) -> CPair {
    let mut csbi: win32::ConsoleScreenBufferInfo = Default::default();
    unsafe {
        win32::GetConsoleScreenBufferInfo(hndl, &mut csbi);
    }
    (csbi.attributes & 0x7, (csbi.attributes & 0x70) >> 4)
}

// Assumes `hndl` is a console screen buffer.
fn set_colors(hndl: win32::Handle, clrs: CPair) {
    unsafe {
        win32::SetConsoleTextAttribute(hndl, clrs.0 | ((clrs.1) << 4));
    }
}

lazy_static! {
    static ref ORIG_PAIR: CPair = {
        let hndl = unsafe { win32::GetStdHandle(Handle::Stdout as u32) };
        match Handle::Stdout.console_mode() {
            ConsoleMode::None => (7, 0),
            _ => get_colors(hndl),
        }
    };
}
