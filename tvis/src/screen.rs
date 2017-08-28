#![cfg(not(windows))]

use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use libc;
use tinf::Desc;
use {SCREEN, Screen, Error, Event, Result, is_rxvt};


pub struct TerminalScreen {
    init_ios: Option<libc::termios>,
    rxvt: bool,
}

impl TerminalScreen {
    pub fn init(tx: Sender<Box<Event>>) -> Result<Box<Screen>> {
        // TODO: make sure console is not redirected, etc.
        if SCREEN.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let mut screen = TerminalScreen {
            init_ios: None,
            rxvt: is_rxvt(Desc::current())
        };
        screen.set_ios()?;
        screen.init_term()?;
        ::input::start_threads(tx)?;
        Ok(Box::new(screen))
    }

    fn set_ios(&mut self) -> Result<()> {
        unsafe {
            let mut init_ios = ::std::mem::zeroed();
            if 0 != libc::tcgetattr(0, &mut init_ios) {
                return Error::ffi_err("tcgetattr failed");
            }
            self.init_ios = Some(init_ios);
            let mut ios = init_ios;
            libc::cfmakeraw(&mut ios);
            if 0 != libc::tcsetattr(0, 0, &ios) {
                return Error::ffi_err("tcsetattr failed");
            }
        }
        Ok(())
    }

    fn init_term(&self) -> Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        let _ = stdout.write_all(b"\x1b[?1000h");
        let _ = stdout.write_all(b"\x1b[?1003h");
        if self.rxvt {
            let _ = stdout.write_all(b"\x1b[?1015h");
        }
        let _ = stdout.write_all(b"\x1b[?1006h");
        let _ = stdout.flush();
        Ok(())
    }
}

impl Screen for TerminalScreen {
    #[cfg(debug_assertions)]
    fn log(&self, text: &str) {
        print!("{}", text);
        println!("\r");
    }
}

impl Drop for TerminalScreen {
    fn drop(&mut self) {
        if let Some(init_ios) = self.init_ios {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let _ = stdout.write_all(b"\x1b[?1000l");
            let _ = stdout.write_all(b"\x1b[?1003l");
            if self.rxvt {
                let _ = stdout.write_all(b"\x1b[?1015l");
            }
            let _ = stdout.write_all(b"\x1b[?1006l");
            let _ = stdout.flush();
            unsafe {
                libc::tcsetattr(0, 0, &init_ios);
            }
        }
    }
}
