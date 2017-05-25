#![cfg(windows)]

use std::io::{self, Write};

use win32;
use {Color, ConsoleMode, Handle, LockableStream, Result, Stream, UseColor};


pub struct ConStream<T> {
    w: T,
    hndl: win32::Handle,
    orig_pair: CPair,
    do_style: bool,
}

impl ConStream<io::Stdout> {
    pub fn stdout(use_color: UseColor) -> ConStream<io::Stdout> {
        ConStream::init(io::stdout(), Handle::Stdout, use_color)
    }
}

impl ConStream<io::Stderr> {
    pub fn stderr(use_color: UseColor) -> ConStream<io::Stderr> {
        ConStream::init(io::stderr(), Handle::Stderr, use_color)
    }
}

impl<T: Write> ConStream<T> {
    fn init(w: T, handle: Handle, use_color: UseColor) -> ConStream<T> {
        let hndl = unsafe { win32::GetStdHandle(handle as u32) };
        let is_atty = super::console_mode(hndl) != ConsoleMode::None;
        let do_style = is_atty && (use_color != UseColor::Never);
        let orig_pair = match do_style {
            true => *ORIG_PAIR,
            false => (7, 0),
        };
        ConStream {
            w,
            hndl,
            orig_pair,
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
}

impl LockableStream for ConStream<io::Stdout> {
    fn lock<'a>(&'a self) -> Box<Stream + 'a> {
        let locked = ConStream {
            w: self.w.lock(),
            hndl: self.hndl,
            orig_pair: self.orig_pair,
            do_style: self.do_style,
        };
        Box::new(locked)
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
}

impl LockableStream for ConStream<io::Stderr> {
    fn lock<'a>(&'a self) -> Box<Stream + 'a> {
        let locked = ConStream {
            w: self.w.lock(),
            hndl: self.hndl,
            orig_pair: self.orig_pair,
            do_style: self.do_style,
        };
        Box::new(locked)
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
        match super::console_mode(hndl) {
            ConsoleMode::None => (7, 0),
            _ => get_colors(hndl),
        }
    };
}
