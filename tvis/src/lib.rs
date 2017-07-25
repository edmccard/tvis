extern crate libc;
extern crate tvis_util;
#[macro_use]
extern crate tinf;

use std::any::Any;
use std::{error, fmt, io, result};
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

#[cfg(windows)]
mod win32;

mod input;

#[cfg(windows)]
#[path = "screen_win.rs"]
pub mod screen;
#[cfg(not(windows))]
#[path = "screen.rs"]
pub mod screen;

pub use input::InputEvent;

/////////////////////////////////////////////////////////////////
static SCREEN: AtomicBool = ATOMIC_BOOL_INIT;

pub trait Screen {}

pub trait Event: fmt::Debug + Send {
    fn as_any(&self) -> &Any;
}

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
            ErrorImpl::Io(ref err) => Some(err),
            ErrorImpl::FFI(_, ref err) => Some(err),
        }
    }
}

/// Either success or failure.
pub type Result<T> = result::Result<T, Error>;
