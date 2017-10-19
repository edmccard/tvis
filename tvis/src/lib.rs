#[macro_use]
extern crate bitflags;
#[cfg(windows)]
extern crate kernel32;
#[cfg(not(windows))]
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[cfg(not(windows))]
#[macro_use]
extern crate tinf;
extern crate tvis_util;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate winapi;

use std::{error, fmt, io, result};
use std::sync::mpsc::SendError;

pub mod term;
pub mod input;

/////////////////////////////////////////////////////////////////
pub type Coords = (u16, u16);

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
    TX,
    #[cfg(not(windows))] Cap(::tinf::CapError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            inner: ErrorImpl::Io(err),
        }
    }
}

impl From<SendError<Box<input::Event>>> for Error {
    fn from(_: SendError<Box<input::Event>>) -> Error {
        Error {
            inner: ErrorImpl::TX,
        }
    }
}

#[cfg(not(windows))]
impl From<::tinf::CapError> for Error {
    fn from(err: ::tinf::CapError) -> Error {
        Error {
            inner: ErrorImpl::Cap(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ErrorImpl::Io(ref err) => err.fmt(f),
            ErrorImpl::FFI(ref msg, _) => write!(f, "{}", msg),
            ErrorImpl::TX => write!(f, "channel send error"),
            #[cfg(not(windows))]
            ErrorImpl::Cap(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.inner {
            ErrorImpl::Io(ref err) => err.description(),
            ErrorImpl::FFI(..) => "terminal FFI error",
            ErrorImpl::TX => "channel send error",
            #[cfg(not(windows))]
            ErrorImpl::Cap(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.inner {
            ErrorImpl::Io(ref err) | ErrorImpl::FFI(_, ref err) => Some(err),
            _ => None,
        }
    }
}

/// Either success or failure.
pub type Result<T> = result::Result<T, Error>;

#[allow(dead_code)]
#[cfg(not(windows))]
const SILENCE_WARNING_FOR_TEST_ONLY_MACRO_USE: [tinf::Param; 0] = params!();

#[cfg(not(windows))]
fn is_rxvt(desc: &::tinf::Desc) -> bool {
    !desc.names().is_empty() && desc.names()[0].starts_with("rxvt-unicode")
}
