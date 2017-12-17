#![cfg(windows)]

use std::io::{self, Write};
use winapi;
use kernel32;
use tvis_util::{ConsoleMode, Handle};
use tvis_util::{color, size};
use {CLIStream, Color, DoStyle, LockableStream, Result, Stream, WinSize};

/// A styled stream using the Windows Console API.
pub struct ConStream<T> {
    w: T,
    hndl: winapi::HANDLE,
    orig_pair: color::CPair,
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
        let hndl =
            unsafe { kernel32::GetStdHandle(handle as winapi::DWORD) };
        let is_atty = handle.console_mode() != ConsoleMode::None;
        let do_style = is_atty && (do_style != DoStyle::Never);
        let orig_pair = match do_style {
            true => color::default_colors(),
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
            color::set_colors(self.hndl, orig);
        }
        Ok(())
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        if self.do_style {
            self.flush()?;
            let cur = color::get_colors(self.hndl);
            let em_mask = cur.0 & (winapi::FOREGROUND_INTENSITY as u16);
            color::set_colors(self.hndl, (fg.1 | em_mask, cur.1));
        }
        Ok(())
    }

    fn em(&mut self) -> Result<()> {
        if self.do_style {
            self.flush()?;
            let cur = color::get_colors(self.hndl);
            let fg = cur.0 | (winapi::FOREGROUND_INTENSITY as u16);
            color::set_colors(self.hndl, (fg, cur.1));
        }
        Ok(())
    }

    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        use std::mem;

        let mut csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO =
            unsafe { mem::uninitialized() };
        unsafe {
            kernel32::GetConsoleScreenBufferInfo(self.hndl, &mut csbi);
        }
        self.flush()?;
        let mut coord = csbi.dwCursorPosition;
        if count > 0 {
            coord.X = 0;
        }
        if count > 1 {
            coord.Y -= ::std::cmp::min(count - 1, coord.Y as u16) as i16;
        }
        unsafe {
            kernel32::SetConsoleCursorPosition(self.hndl, coord);
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
