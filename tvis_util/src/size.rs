use Handle;

/// The dimensions of a terminal window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WinSize {
    pub cols: i32,
    pub rows: i32,
}

impl Default for WinSize {
    fn default() -> WinSize {
        WinSize { cols: 80, rows: 24 }
    }
}

/// The size of the console connected to the `handle`.
///
/// Does not work in a Cygwin/MSYS2 window.
#[cfg(windows)]
pub fn get_size(handle: Handle) -> Option<WinSize> {
    use win32;
    let hndl = handle.win_handle();
    let mut csbi: win32::ConsoleScreenBufferInfo = Default::default();
    let res = unsafe { win32::GetConsoleScreenBufferInfo(hndl, &mut csbi) };
    if res == 0 {
        return None;
    }
    Some(WinSize {
        cols: (csbi.window.right - csbi.window.left + 1) as i32,
        rows: (csbi.window.bottom - csbi.window.top + 1) as i32,
    })
}

#[cfg(windows)]
use std::io;
#[cfg(windows)]
pub fn get_cygwin_size(w: &io::Write, defsz: WinSize) -> WinSize {
    // TODO: use cursor report?
    let _ = w;
    let _ = defsz;
    Default::default()
}

/// The size of the terminal connected to the `handle`.
#[cfg(not(windows))]
pub fn get_size(handle: Handle) -> Option<WinSize> {
    use libc;
    let win: libc::winsize = unsafe { ::std::mem::uninitialized() };
    let res = unsafe { libc::ioctl(handle.fd(), libc::TIOCGWINSZ, &win) };
    if res != 0 {
        return None;
    }
    Some(WinSize {
        cols: win.ws_col as i32,
        rows: win.ws_row as i32,
    })
}

#[cfg(windows)]
pub fn get_default_console_size() -> WinSize {
    use std::ptr;
    use libc;
    use win32;

    let mut key: *mut libc::c_void = ptr::null_mut();
    let res = unsafe {
        win32::RegOpenKeyExA(
            win32::HKEY_CURRENT_USER,
            "Console\x00".as_ptr(),
            0,
            win32::KEY_READ,
            &mut key,
        )
    };
    if res != 0 {
        return Default::default();
    }
    let mut data = 0u32;
    let mut data_size = 4u32;
    let res = unsafe {
        win32::RegQueryValueExA(
            key,
            "WindowSize\x00".as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut data as *mut _ as *mut u8,
            &mut data_size,
        )
    };
    if res != 0 {
        return Default::default();
    }
    WinSize {
        cols: (data & 0xffff) as i32,
        rows: (data >> 16) as i32,
    }

}
