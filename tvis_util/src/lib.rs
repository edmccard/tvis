#[cfg(not(windows))]
extern crate libc;
#[cfg(windows)]
extern crate advapi32;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

mod mode;
pub mod size;

pub use mode::TerminalMode;
#[cfg(windows)]
pub use mode::ConsoleMode;

/// An abstract handle to a standard input or output stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Handle {
    Stdin = 0xffff_fff6,
    Stdout = 0xffff_fff5,
    Stderr = 0xffff_fff4,
}

#[cfg(windows)]
pub type WinHandle = winapi::HANDLE;

impl Handle {
    /// The raw Windows handle for the given `handle`.
    #[cfg(windows)]
    pub fn win_handle(self) -> winapi::HANDLE {
        unsafe { kernel32::GetStdHandle(self as winapi::DWORD) }
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
