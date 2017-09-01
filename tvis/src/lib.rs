#[macro_use]
extern crate bitflags;
#[cfg(windows)]
extern crate kernel32;
extern crate libc;
#[macro_use]
extern crate tinf;
extern crate tvis_util;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate winapi;

use std::any::Any;
use std::{error, fmt, io, result};
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

mod input;

#[cfg(windows)]
#[path = "screen_win.rs"]
pub mod screen;
#[cfg(not(windows))]
#[path = "screen.rs"]
pub mod screen;

pub use input::{ButtonMotion, InputEvent, Key, Mods, MouseButton, WheelMotion};

/////////////////////////////////////////////////////////////////
static SCREEN: AtomicBool = ATOMIC_BOOL_INIT;

pub trait Screen {
    #[cfg(debug_assertions)]
    fn log(&self, text: &str);
}

pub trait Event: fmt::Debug + Send {
    fn as_any(&self) -> &Any;
}

pub type Coords = (u32, u32);

#[derive(Debug)]
pub struct Error {
    inner: ErrorImpl,
}

impl Error {
    fn ffi_err<T>(msg: &str) -> Result<T> {
        Err(Error {
            inner: ErrorImpl::FFI(msg.to_owned(), io::Error::last_os_error()),
        })
    }
}

#[derive(Debug)]
enum ErrorImpl {
    Io(io::Error),
    FFI(String, io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            inner: ErrorImpl::Io(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ErrorImpl::Io(ref err) => err.fmt(f),
            ErrorImpl::FFI(ref msg, _) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.inner {
            ErrorImpl::Io(ref err) => err.description(),
            ErrorImpl::FFI(..) => "terminal FFI error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.inner {
            ErrorImpl::Io(ref err) | ErrorImpl::FFI(_, ref err) => Some(err),
        }
    }
}

/// Either success or failure.
pub type Result<T> = result::Result<T, Error>;

#[allow(dead_code)]
const SILENCE_WARNING_FOR_TEST_ONLY_MACRO_USE: [tinf::Param; 0] = params!();

#[cfg(not(windows))]
fn is_rxvt(desc: &::tinf::Desc) -> bool {
    !desc.names().is_empty() && desc.names()[0].starts_with("rxvt-unicode")
}
