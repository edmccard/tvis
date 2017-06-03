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
