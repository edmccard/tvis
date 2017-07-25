use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

use libc;
use tinf::{Desc, tputs};

use {SCREEN, Screen, Error, Event, Result};

pub struct TerminalScreen {
    init_ios: Option<libc::termios>,
    desc: Desc,
}

impl TerminalScreen {
    pub fn init(tx: Sender<Box<Event>>, desc: &Desc) -> Result<Box<Screen>> {
        // TODO: make sure console is not redirected, etc.
        if SCREEN.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let mut screen = TerminalScreen {
            init_ios: None,
            desc: (*desc).clone(),
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
        Ok(())
    }
}

impl Screen for TerminalScreen {}

impl Drop for TerminalScreen {
    fn drop(&mut self) {
        if let Some(init_ios) = self.init_ios {
            unsafe {
                libc::tcsetattr(0, 0, &init_ios);
            }
        }
    }
}
