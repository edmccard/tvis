#![allow(non_upper_case_globals)]

#[macro_use]
extern crate tinf;
#[macro_use]
extern crate lazy_static;

use std::{error, fmt, io};

#[cfg(windows)]
mod win32;
#[cfg(windows)]
mod console;
#[cfg(windows)]
pub use self::console::ConStream;

mod buf;
pub use self::buf::BufStream;
mod term;
pub use self::term::TermStream;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Handle {
    Stdin = 0xfffffff6,
    Stdout = 0xfffffff5,
    Stderr = 0xfffffff4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseColor {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(i32, u16);

pub const Black: Color = Color(0, 0);
pub const Red: Color = Color(1, 4);
pub const Green: Color = Color(2, 2);
pub const Yellow: Color = Color(3, 6);
pub const Blue: Color = Color(4, 1);
pub const Magenta: Color = Color(5, 5);
pub const Cyan: Color = Color(6, 3);
pub const White: Color = Color(7, 7);

pub trait Stream: io::Write {
    fn reset(&mut self) -> Result<()>;
    fn fg(&mut self, fg: Color) -> Result<()>;
    fn em(&mut self) -> Result<()>;
}

pub trait LockableStream: Stream {
    fn lock<'a>(&'a self) -> Box<Stream + 'a>;
}


#[derive(Debug)]
pub struct Error(());

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error(())
    }
}

impl From<::tinf::CapError> for Error {
    fn from(err: ::tinf::CapError) -> Error {
        Error(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO Error::fmt()")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "TODO Error::description()"
    }

    fn cause(&self) -> Option<&error::Error> {
        // TODO
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;


// Based on https://github.com/dtolnay/isatty.

enum TerminalMode {
    None,
    Redir,
    Term,
    #[cfg(windows)]
    Win10,
}

#[cfg(not(windows))]
fn terminal_mode(handle: Handle) -> TerminalMode {
    extern crate libc;

    let handle = match handle {
        Handle::Stdout => libc::STDOUT_FILENO,
        Handle::Stderr => libc::STDERR_FILENO,
        _ => unreachable!(),
    };
    match unsafe { libc::isatty(handle) } {
        0 => TerminalMode::Redir,
        _ => TerminalMode::Term,
    }
}

#[cfg(windows)]
fn terminal_mode(handle: Handle) -> TerminalMode {
    use win32;
    use std::os::windows::ffi::OsStringExt;

    // Return false if (non-terminfo) console.
    let hndl = unsafe { win32::GetStdHandle(handle as u32) };
    match console_mode(hndl) {
        ConsoleMode::Default => return TerminalMode::None,
        ConsoleMode::VT => return TerminalMode::Win10,
        ConsoleMode::None => (),
    }

    let sz = ::std::mem::size_of::<win32::FileNameInfo>();
    let mut raw_info = vec![0u8; sz + win32::MAX_PATH];

    let ok = unsafe {
        win32::GetFileInformationByHandleEx(
            hndl,
            2,
            raw_info.as_mut_ptr() as *mut win32::Void,
            raw_info.len() as u32,
        )
    };
    if ok == 0 {
        // We already checked for a console, and we only want to
        // return true if its an actual cygwin/msys thingy.
        return TerminalMode::None;
    }

    let file_info =
        unsafe { *(raw_info[0..sz].as_ptr() as *const win32::FileNameInfo) };
    let name = &raw_info[sz..sz + file_info.file_name_length as usize];
    let name = unsafe {
        ::std::slice::from_raw_parts(
            name.as_ptr() as *const win32::WChar,
            name.len() / 2,
        )
    };
    let name = ::std::ffi::OsString::from_wide(name);
    let name = name.to_string_lossy();

    if name.starts_with("\\cygwin-") || name.starts_with("\\msys-") {
        TerminalMode::Term
    } else {
        TerminalMode::Redir
    }
}

#[cfg(windows)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConsoleMode {
    None,
    Default,
    VT,
}

#[cfg(windows)]
fn console_mode(hndl: ::win32::Handle) -> ConsoleMode {
    use win32;

    if hndl == win32::INVALID_HANDLE_VALUE {
        return ConsoleMode::None;
    }
    unsafe {
        let mut mode: u32 = 0;
        if 0 == win32::GetConsoleMode(hndl, &mut mode) {
            return ConsoleMode::None;
        }
        if (mode & win32::ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0 {
            return ConsoleMode::VT;
        }
        mode |= win32::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if 0 == win32::SetConsoleMode(hndl, mode) {
            ConsoleMode::Default
        } else {
            ConsoleMode::VT
        }
    }
}
