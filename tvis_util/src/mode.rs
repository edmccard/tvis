use Handle;
#[cfg(windows)]
use winapi;
#[cfg(windows)]
use kernel32;

// Based on https://github.com/dtolnay/isatty.

/// The type of terminal, if any, for a standard stream handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalMode {
    /// Not connected to a terminal or console.
    Redir,
    /// A terminal emulator.
    Term,
    /// A Cygwin or MSYS2 terminal.
    #[cfg(windows)]
    Cygwin,
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
        match unsafe { ::libc::isatty(handle.fd()) } {
            0 => TerminalMode::Redir,
            _ => TerminalMode::Term,
        }
    }

    #[cfg(windows)]
    pub(super) fn from_handle(handle: Handle) -> TerminalMode {
        match handle.console_mode() {
            ConsoleMode::Legacy => return TerminalMode::Console,
            ConsoleMode::Win10 => return TerminalMode::Win10,
            ConsoleMode::None => (),
        }

        let hndl =
            unsafe { kernel32::GetStdHandle(handle as winapi::DWORD) };
        match msys_cygwin(hndl) {
            Some(true) => TerminalMode::Cygwin,
            _ => TerminalMode::Redir,
        }
    }
}

// Assumes console_mode(hndl) has already returned None.
#[cfg(windows)]
fn msys_cygwin(hndl: winapi::HANDLE) -> Option<bool> {
    use std::os::windows::ffi::OsStringExt;

    let sz = ::std::mem::size_of::<winapi::FILE_NAME_INFO>();
    let mut raw_info = vec![0u8; sz + winapi::MAX_PATH];

    let ok = unsafe {
        kernel32::GetFileInformationByHandleEx(
            hndl,
            winapi::FILE_INFO_BY_HANDLE_CLASS(2),
            raw_info.as_mut_ptr() as winapi::LPVOID,
            raw_info.len() as winapi::DWORD,
        )
    };
    if ok == 0 {
        return None;
    }

    let file_info = unsafe {
        *(raw_info[0..sz].as_ptr() as *const winapi::FILE_NAME_INFO)
    };
    let name = &raw_info[sz..sz + file_info.FileNameLength as usize];
    let name = unsafe {
        ::std::slice::from_raw_parts(
            name.as_ptr() as *const winapi::WCHAR,
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
impl Default for ConsoleMode {
    fn default() -> ConsoleMode {
        ConsoleMode::None
    }
}

// winapi-rs omits these
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: winapi::DWORD = 0x0004;
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_INPUT: winapi::DWORD = 0x0200;

#[cfg(windows)]
impl ConsoleMode {
    pub(super) fn from_out_handle(hndl: winapi::HANDLE) -> ConsoleMode {
        if hndl == winapi::INVALID_HANDLE_VALUE {
            return ConsoleMode::None;
        }
        unsafe {
            let mut mode: winapi::DWORD = 0;
            if 0 == kernel32::GetConsoleMode(hndl, &mut mode) {
                return ConsoleMode::None;
            }
            if (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0 {
                return ConsoleMode::Win10;
            }
            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if 0 == kernel32::SetConsoleMode(hndl, mode) {
                ConsoleMode::Legacy
            } else {
                ConsoleMode::Win10
            }
        }
    }

    pub(super) fn from_in_handle(hndl: winapi::HANDLE) -> ConsoleMode {
        if hndl == winapi::INVALID_HANDLE_VALUE {
            return ConsoleMode::None;
        }
        unsafe {
            let mut mode: winapi::DWORD = 0;
            if 0 == kernel32::GetConsoleMode(hndl, &mut mode) {
                return ConsoleMode::None;
            }
            if (mode & ENABLE_VIRTUAL_TERMINAL_INPUT) != 0 {
                return ConsoleMode::Win10;
            }
            let newmode = mode | ENABLE_VIRTUAL_TERMINAL_INPUT;
            if 0 == kernel32::SetConsoleMode(hndl, newmode) {
                kernel32::SetConsoleMode(hndl, mode);
                ConsoleMode::Legacy
            } else {
                kernel32::SetConsoleMode(hndl, mode);
                ConsoleMode::Win10
            }
        }
    }
}
