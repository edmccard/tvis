use libc;
use Handle;
#[cfg(windows)]
use win32;

// Based on https://github.com/dtolnay/isatty.

/// The type of terminal, if any, for a standard stream handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalMode {
    /// An error occurred while checking the mode.
    #[cfg(windows)]
    None,
    /// Not connected to a terminal or console.
    Redir,
    /// A terminal (on Windows, a Cygwin/MSYS shell).
    Term,
    /// A legacy console buffer.
    #[cfg(windows)]
    Console,
    /// A Windows 10 console buffer that supports Console Virtual
    /// Terminal Sequences.
    #[cfg(windows)]
    Win10,
}

impl TerminalMode {
    #[cfg(not(windows))]
    pub(super) fn from_handle(handle: Handle) -> TerminalMode {
        let handle = match handle {
            Handle::Stdin => libc::STDIN_FILENO,
            Handle::Stdout => libc::STDOUT_FILENO,
            Handle::Stderr => libc::STDERR_FILENO,
        };
        match unsafe { libc::isatty(handle) } {
            0 => TerminalMode::Redir,
            _ => TerminalMode::Term,
        }
    }

    #[cfg(windows)]
    pub(super) fn from_handle(handle: Handle) -> TerminalMode {
        use win32;

        match handle.console_mode() {
            ConsoleMode::Legacy => return TerminalMode::Console,
            ConsoleMode::Win10 => return TerminalMode::Win10,
            ConsoleMode::None => (),
        }

        let hndl = unsafe { win32::GetStdHandle(handle as u32) };
        match msys_cygwin(hndl) {
            None => TerminalMode::None,
            Some(true) => TerminalMode::Term,
            Some(false) => TerminalMode::Redir,
        }
    }
}

// Assumes console_mode(hndl) has already returned None.
#[cfg(windows)]
fn msys_cygwin(hndl: ::win32::Handle) -> Option<bool> {
    use std::os::windows::ffi::OsStringExt;

    let sz = ::std::mem::size_of::<win32::FileNameInfo>();
    let mut raw_info = vec![0u8; sz + win32::MAX_PATH];

    let ok = unsafe {
        ::win32::GetFileInformationByHandleEx(
            hndl,
            2,
            raw_info.as_mut_ptr() as *mut libc::c_void,
            raw_info.len() as u32,
        )
    };
    if ok == 0 {
        return None;
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
        Some(true)
    } else {
        Some(false)
    }
}

/// The type of console buffer, if any, for a standard stream handle.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsoleMode {
    /// No console buffer.
    None,
    /// A legacy console buffer.
    Legacy,
    /// A Windows 10 console buffer that supports Console Virtual
    /// Terminal Sequences.
    Win10,
}

#[cfg(windows)]
impl ConsoleMode {
    pub(super) fn from_out_handle(hndl: win32::Handle) -> ConsoleMode {
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
                return ConsoleMode::Win10;
            }
            mode |= win32::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if 0 == win32::SetConsoleMode(hndl, mode) {
                ConsoleMode::Legacy
            } else {
                ConsoleMode::Win10
            }
        }
    }

    pub(super) fn from_in_handle(hndl: win32::Handle) -> ConsoleMode {
        use win32;

        if hndl == win32::INVALID_HANDLE_VALUE {
            return ConsoleMode::None;
        }
        unsafe {
            let mut mode: u32 = 0;
            if 0 == win32::GetConsoleMode(hndl, &mut mode) {
                return ConsoleMode::None;
            }
            if (mode & win32::ENABLE_VIRTUAL_TERMINAL_INPUT) != 0 {
                return ConsoleMode::Win10;
            }
            let newmode = mode | win32::ENABLE_VIRTUAL_TERMINAL_INPUT;
            if 0 == win32::SetConsoleMode(hndl, newmode) {
                win32::SetConsoleMode(hndl, mode);
                ConsoleMode::Legacy
            } else {
                ConsoleMode::Win10
            }
        }
    }
}
