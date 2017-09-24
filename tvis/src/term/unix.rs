#![cfg(not(windows))]

use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use libc;
use tinf::Desc;
use tvis_util::{Handle, TerminalMode};
use tvis_util::size::get_size;
use input::Event;
use term::{Terminal, WinSize, TERM};
use {is_rxvt, Error, Result};

lazy_static! {
    static ref STDOUT: io::Stdout = io::stdout();
}

pub(in term) struct Term<'a> {
    stdout: io::StdoutLock<'a>,
    init_ios: libc::termios,
    tmode: (TerminalMode, TerminalMode),
    tx: Option<Sender<Box<Event>>>,
    rxvt: bool,
}

impl<'a> Term<'a> {
    pub(in term) fn connect(
        tx: Option<Sender<Box<Event>>>,
    ) -> Result<Box<Terminal>> {
        if TERM.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        // TODO: make sure console is not redirected, etc.
        let init_ios = Term::set_ios()?;
        Term::init(STDOUT.lock())?;
        let term = Term {
            stdout: STDOUT.lock(),
            init_ios,
            tmode: (
                Handle::Stdout.terminal_mode(),
                Handle::Stdin.terminal_mode(),
            ),
            tx,
            rxvt: is_rxvt(Desc::current()),
        };
        Ok(Box::new(term))
    }

    fn set_ios() -> Result<libc::termios> {
        unsafe {
            let mut init_ios = ::std::mem::zeroed();
            if 0 != libc::tcgetattr(0, &mut init_ios) {
                return Error::ffi_err("tcgetattr failed");
            }
            let mut ios = init_ios;
            libc::cfmakeraw(&mut ios);
            if 0 != libc::tcsetattr(0, 0, &ios) {
                return Error::ffi_err("tcsetattr failed");
            }
            Ok(init_ios)
        }
    }

    fn init(_: io::StdoutLock) -> Result<()> {
        Ok(())
    }

    fn init_mouse(&mut self) -> Result<()> {
        self.stdout.write_all(b"\x1b[?1000h")?;
        self.stdout.write_all(b"\x1b[?1003h")?;
        if self.rxvt {
            self.stdout.write_all(b"\x1b[?1015h")?;
        }
        self.stdout.write_all(b"\x1b[?1006h")?;
        Ok(())
    }

    fn drop_mouse(&mut self) {
        let _ = self.stdout.write_all(b"\x1b[?1000l");
        let _ = self.stdout.write_all(b"\x1b[?1003l");
        if self.rxvt {
            let _ = self.stdout.write_all(b"\x1b[?1015l");
        }
        let _ = self.stdout.write_all(b"\x1b[?1006l");
    }
}

impl<'a> Terminal for Term<'a> {
    fn is_tty_input(&self) -> bool {
        self.tmode.1 != TerminalMode::Redir
    }

    fn is_tty_output(&self) -> bool {
        self.tmode.0 != TerminalMode::Redir
    }

    fn get_size(&self) -> Result<WinSize> {
        match get_size(Handle::Stdout) {
            Some(ws) => Ok(ws),
            None => Error::ffi_err("ioctl failed"),
        }
    }

    fn start_input(&mut self) -> Result<()> {
        self.init_mouse()?;
        self.stdout.flush()?;
        ::input::start_threads(
            self.tx.take().expect("start_input may only be called once"),
        )
    }

    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str) {
        let mut stderr = io::stderr();
        let _ = write!(stderr, "{}", text);
        let _ = writeln!(stderr, "\r");
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        self.drop_mouse();
        let _ = self.stdout.flush();
        unsafe {
            libc::tcsetattr(0, 0, &self.init_ios);
        }
    }
}
