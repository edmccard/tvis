extern crate libc;

mod mode;
#[cfg(windows)]
mod win32;

pub use mode::TerminalMode;
#[cfg(windows)]
pub use mode::ConsoleMode;

/// An abstract handle to a standard input or output stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Handle {
    Stdin = 0xfffffff6,
    Stdout = 0xfffffff5,
    Stderr = 0xfffffff4,
}

impl Handle {
    /// The raw Windows handle for the given `handle`.
    #[cfg(windows)]
    pub fn win_handle(self) -> *mut libc::c_void {
        unsafe { win32::GetStdHandle(self as u32) }
    }

    /// The raw fd for the given `handle`.
    #[cfg(not(windows))]
    pub fn fd(self) -> i32 {
        match self {
            Handle::Stdin => libc::STDIN_FILENO,
            Handle::Stdout => libc::STDOUT_FILENO,
            Handle::Stderr => libc::STDERR_FILENO,
        }
    }


    /// The `TerminalMode` for the given `handle`.
    pub fn terminal_mode(self) -> TerminalMode {
        TerminalMode::from_handle(self)
    }

    /// The `ConsoleMode` for the given `handle`.
    #[cfg(windows)]
    pub fn console_mode(self) -> ConsoleMode {
        match self {
            Handle::Stdin => ConsoleMode::from_in_handle(self.win_handle()),
            _ => ConsoleMode::from_out_handle(self.win_handle()),
        }
    }
}

/// The dimensions of a terminal window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WinSize {
    cols: i32,
    rows: i32,
}

impl WinSize {
    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }
}

/// The size of the console connected to the `handle`.
///
/// Does not work in a Cygwin/MSYS2 window.
#[cfg(windows)]
pub fn get_size(handle: Handle) -> Option<WinSize> {
    let hndl = handle.win_handle();
    let mut csbi: win32::ConsoleScreenBufferInfo = Default::default();
    let res = unsafe { win32::GetConsoleScreenBufferInfo(hndl, &mut csbi) };
    if res == 0 {
        return None;
    }
    Some(
        WinSize {
            cols: (csbi.window.right - csbi.window.left + 1) as i32,
            rows: (csbi.window.bottom - csbi.window.top + 1) as i32,
        }
    )
}

/// The size of the terminal connected to the `handle`.
#[cfg(not(windows))]
pub fn get_size(handle: Handle) -> Option<WinSize> {
    let win: libc::winsize = unsafe { ::std::mem::uninitialized() };
    let res = unsafe { libc::ioctl(handle.fd(), libc::TIOCGWINSZ, &win) };
    if res != 0 {
        return None;
    }
    Some(
        WinSize {
            cols: win.ws_col as i32,
            rows: win.ws_row as i32,
        }
    )
}
