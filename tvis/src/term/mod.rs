use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;
#[cfg(not(windows))]
#[path = "unix.rs"]
mod platform;

#[cfg(windows)]
pub use self::platform::Term;

#[cfg(not(windows))]
pub use self::platform::Term;

static SCREEN: AtomicBool = ATOMIC_BOOL_INIT;

pub trait Screen {
    #[cfg(debug_assertions)]
    fn log(&self, text: &str);
}
