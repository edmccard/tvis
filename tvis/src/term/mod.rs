use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use std::sync::mpsc::Sender;
use input::Event;
use Result;

pub use tvis_util::size::WinSize;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;
#[cfg(not(windows))]
#[path = "unix.rs"]
mod platform;

#[cfg(windows)]
use self::platform::Term;
#[cfg(not(windows))]
use self::platform::Term;

static TERM: AtomicBool = ATOMIC_BOOL_INIT;

pub trait Terminal {
    fn get_size(&self) -> Result<WinSize>;
    fn start_input(&mut self) -> Result<()>;
    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str);
}

pub fn connect() -> Result<Box<Terminal>> {
    Term::connect(None)
}

pub fn connect_with_input(tx: Sender<Box<Event>>) -> Result<Box<Terminal>> {
    Term::connect(Some(tx))
}
