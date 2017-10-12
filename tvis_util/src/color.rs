#![cfg(windows)]

use winapi;
use kernel32;
use {ConsoleMode, Handle};

pub type CPair = (winapi::USHORT, winapi::USHORT);

// Assumes `hndl` is a console screen buffer.
pub fn get_colors(hndl: winapi::HANDLE) -> CPair {
    use std::mem;

    let mut csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO =
        unsafe { mem::uninitialized() };
    unsafe {
        kernel32::GetConsoleScreenBufferInfo(hndl, &mut csbi);
    }
    (csbi.wAttributes & 0x7, (csbi.wAttributes & 0x70) >> 4)
}

// Assumes `hndl` is a console screen buffer.
pub fn set_colors(hndl: winapi::HANDLE, clrs: CPair) {
    unsafe {
        kernel32::SetConsoleTextAttribute(hndl, clrs.0 | ((clrs.1) << 4));
    }
}

/// The default Windows console colors.
pub fn default_colors() -> CPair {
    *ORIG_PAIR
}

lazy_static! {
    static ref ORIG_PAIR: CPair = {
        let hndl =
            unsafe { kernel32::GetStdHandle(Handle::Stdout as winapi::DWORD) };
        match Handle::Stdout.console_mode() {
            ConsoleMode::None => (7, 0),
            _ => get_colors(hndl),
        }
    };
}
