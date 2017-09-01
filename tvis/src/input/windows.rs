#![cfg(windows)]

use std::{ptr, thread, time};
use std::sync::mpsc::{channel, Sender};
use winapi;
use kernel32;
use user32;
use tvis_util::Handle;
use input::{InputEvent, Key, Mods};
use {Error, Event, Result};

const SHUTDOWN_KEY: u16 = 0x1111;
const SIGINT_KEY: u16 = 0x2222;
const SIGQUIT_KEY: u16 = 0x3333;

// winapi-rs omits these.
const EVENT_CONSOLE_LAYOUT: winapi::DWORD = 0x4005;
const WINEVENT_OUTOFCONTEXT: winapi::DWORD = 0;
const WINEVENT_SKIPOWNTHREAD: winapi::DWORD = 1;

pub(crate) fn start_threads(tx: Sender<Box<Event>>) -> Result<()> {
    register_ctrl_handler()?;
    let (init_tx, init_rx) = channel();
    thread::spawn(move || unsafe {
        create_session_wnd()
            .and_then(|_| register_layout_hook())
            .and_then(|_| {
                init_tx.send(Ok(())).unwrap();
                run_message_pump();
                Ok(())
            })
            .or_else(|e| init_tx.send(Err(e)))
            .unwrap();
    });
    init_rx.recv().unwrap()?;
    thread::spawn(move || raw_event_loop(tx));
    Ok(())
}

fn register_ctrl_handler() -> Result<()> {
    extern "system" fn handler(ctrl_type: winapi::DWORD) -> winapi::BOOL {
        match ctrl_type {
            winapi::CTRL_C_EVENT => {
                write_fake_key(SIGINT_KEY);
                1
            }
            winapi::CTRL_BREAK_EVENT => {
                write_fake_key(SIGQUIT_KEY);
                1
            }
            winapi::CTRL_CLOSE_EVENT => {
                write_fake_key(SHUTDOWN_KEY);
                thread::sleep(time::Duration::from_secs(5));
                0
            }
            _ => 0,
        }
    }

    match unsafe { kernel32::SetConsoleCtrlHandler(Some(handler), 1) } {
        0 => Error::ffi_err("SetConsoleCtrlHandler failed"),
        _ => Ok(()),
    }
}

// winapi-rs omits this.
#[allow(non_snake_case)]
#[repr(C)]
struct WNDCLASS {
    pub style: winapi::UINT,
    pub lpfnWndProc: winapi::WNDPROC,
    pub cbClsExtra: winapi::c_int,
    pub cbWndExtra: winapi::c_int,
    pub instance: winapi::HINSTANCE,
    pub hIcon: winapi::HICON,
    pub hCursor: winapi::HCURSOR,
    pub hbrBackground: winapi::HBRUSH,
    pub lpszMenuName: winapi::LPCSTR,
    pub lpszClassName: winapi::LPCSTR,
}

// winapi-rs omits this.
extern "system" {
    fn RegisterClassA(lpWndClass: *const WNDCLASS) -> winapi::ATOM;
}

unsafe fn create_session_wnd() -> Result<()> {
    extern "system" fn wnd_proc(
        hwnd: winapi::HWND,
        msg: winapi::UINT,
        wparam: winapi::WPARAM,
        lparam: winapi::LPARAM,
    ) -> winapi::LRESULT {
        match msg {
            winapi::WM_ENDSESSION => {
                write_fake_key(SHUTDOWN_KEY);
                thread::sleep(time::Duration::from_secs(5));
                0
            }
            _ => unsafe { user32::DefWindowProcA(hwnd, msg, wparam, lparam) },
        }
    }

    let mut wnd_class: WNDCLASS = ::std::mem::zeroed();
    wnd_class.lpfnWndProc = Some(wnd_proc);
    wnd_class.instance = kernel32::GetModuleHandleA(ptr::null());
    if wnd_class.instance.is_null() {
        return Error::ffi_err("GetModuleHandle failed");
    }
    wnd_class.lpszClassName =
        "HiddenShutdownClass\x00".as_ptr() as *const _ as winapi::LPCSTR;
    if 0 == RegisterClassA(&wnd_class) {
        return Error::ffi_err("RegisterClass failed");
    }
    let hwnd = user32::CreateWindowExA(
        0,
        "HiddenShutdownClass\x00".as_ptr() as *const _ as winapi::LPCSTR,
        ptr::null(),
        0,
        0,
        0,
        0,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        kernel32::GetModuleHandleA(ptr::null()),
        ptr::null_mut(),
    );
    if hwnd.is_null() {
        return Error::ffi_err("CreateWindowEx failed");
    }
    Ok(())
}

fn register_layout_hook() -> Result<()> {
    extern "system" fn layout_hook(
        _: winapi::HWINEVENTHOOK,
        _: winapi::DWORD,
        hwnd: winapi::HWND,
        _: winapi::LONG,
        _: winapi::LONG,
        _: winapi::DWORD,
        _: winapi::DWORD,
    ) {
        // Filter out events from consoles in other processes.
        if hwnd != unsafe { kernel32::GetConsoleWindow() } {
            return;
        }
        // Use an "empty" window buffer size event as a resize
        // notification.
        let mut ir: winapi::INPUT_RECORD =
            unsafe { ::std::mem::uninitialized() };
        ir.EventType = winapi::WINDOW_BUFFER_SIZE_EVENT;
        {
            let mut ir = unsafe { ir.WindowBufferSizeEvent_mut() };
            ir.dwSize.X = 0;
            ir.dwSize.Y = 0;
        }
        let con_hndl = Handle::Stdin.win_handle();
        let mut write_count: winapi::DWORD = 0;
        unsafe {
            kernel32::WriteConsoleInputW(con_hndl, &ir, 1, &mut write_count);
        }
    }

    let hook = unsafe {
        user32::SetWinEventHook(
            EVENT_CONSOLE_LAYOUT,
            EVENT_CONSOLE_LAYOUT,
            ptr::null_mut(),
            Some(layout_hook),
            // Listen for events from all threads/processes and filter
            // in the callback, because there doesn't seem to be a way
            // to get the id for the thread that actually delivers
            // WinEvents for the console (it's not the thread returned
            // by GetWindowThreadProcessId(GetConsoleWindow())).
            0,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNTHREAD,
        )
    };

    if hook.is_null() {
        return Error::ffi_err("SetWinEventHook failed");
    }
    Ok(())
}

// Windows events and WinEvents require a thread with a message pump.
unsafe fn run_message_pump() {
    let mut msg: winapi::MSG = ::std::mem::uninitialized();
    while 0 != user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) {
        user32::TranslateMessage(&msg);
        user32::DispatchMessageW(&msg);
    }
}

fn write_fake_key(key_code: u16) -> winapi::BOOL {
    let mut key: winapi::INPUT_RECORD = unsafe { ::std::mem::uninitialized() };
    key.EventType = winapi::KEY_EVENT;
    {
        let key = unsafe { key.KeyEvent_mut() };
        key.bKeyDown = 1;
        key.wVirtualKeyCode = key_code;
    }
    let con_hndl = Handle::Stdin.win_handle();
    let mut write_count: winapi::DWORD = 0;
    unsafe { kernel32::WriteConsoleInputW(con_hndl, &key, 1, &mut write_count) }
}

fn raw_event_loop(tx: Sender<Box<Event>>) {
    let mut scrn_size = match ScreenSize::from_conout() {
        Ok(c) => c,
        _ => return,
    };
    let in_hndl = Handle::Stdin.win_handle();
    let mut buffer: [winapi::INPUT_RECORD; 128] =
        unsafe { ::std::mem::uninitialized() };
    let mut surrogate: Option<u16> = None;
    loop {
        let mut read_count: winapi::DWORD = 0;
        let ok = unsafe {
            kernel32::ReadConsoleInputW(
                in_hndl,
                buffer.as_mut_ptr(),
                128,
                &mut read_count,
            )
        };
        if ok == 0 {
            return;
        }
        for i in 0..read_count as usize {
            let input = buffer[i];
            if input.EventType == winapi::FOCUS_EVENT ||
                input.EventType == winapi::MENU_EVENT
            {
                continue;
            }
            let event = match input.EventType {
                winapi::MOUSE_EVENT => {
                    let mevt = unsafe { input.MouseEvent() };
                    process_mouse(mevt)
                }
                winapi::KEY_EVENT => {
                    let kevt = unsafe { input.KeyEvent() };
                    match kevt.wVirtualKeyCode {
                        SHUTDOWN_KEY => return,
                        SIGINT_KEY => Some(InputEvent::Interrupt),
                        SIGQUIT_KEY => Some(InputEvent::Break),
                        _ => {
                            let uchar = kevt.UnicodeChar;
                            if surrogate.is_some() &&
                                (uchar >= 0xdc00 && uchar <= 0xdfff)
                            {
                                let s1 = surrogate.unwrap();
                                surrogate = None;
                                Some(surrogate_pair(s1, kevt))
                            } else if uchar >= 0xd800 && uchar <= 0xdbff {
                                surrogate = Some(uchar);
                                None
                            } else {
                                process_key(kevt)
                            }
                        }
                    }
                }
                winapi::WINDOW_BUFFER_SIZE_EVENT => match scrn_size.update() {
                    Ok(true) => Some(InputEvent::Repaint),
                    Ok(false) => None,
                    Err(_) => return,
                },
                _ => unreachable!(),
            };
            if let Some(event) = event {
                if tx.send(Box::new(event)).is_err() {
                    return;
                }
            }
        }
    }
}


pub(crate) struct ScreenSize {
    hndl: winapi::HANDLE,
    size: winapi::COORD,
}

impl ScreenSize {
    fn from_conout() -> Result<ScreenSize> {
        let hndl = unsafe {
            kernel32::CreateFileA(
                "CONOUT$\x00".as_ptr() as *const _ as winapi::LPCSTR,
                winapi::GENERIC_READ | winapi::GENERIC_WRITE,
                winapi::FILE_SHARE_READ | winapi::FILE_SHARE_WRITE,
                ptr::null_mut(),
                winapi::OPEN_EXISTING,
                0,
                ptr::null_mut(),
            )
        };
        if hndl == winapi::INVALID_HANDLE_VALUE {
            return Error::ffi_err("CreateFileA failed");
        }
        ScreenSize::from_hndl(hndl)
    }

    pub(crate) fn from_hndl(hndl: winapi::HANDLE) -> Result<ScreenSize> {
        let mut scrn_size = ScreenSize {
            hndl,
            size: winapi::COORD { X: -1, Y: -1 },
        };
        scrn_size.update()?;
        Ok(scrn_size)
    }

    fn update(&mut self) -> Result<bool> {
        let mut csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO =
            unsafe { ::std::mem::uninitialized() };
        if 0 == unsafe {
            kernel32::GetConsoleScreenBufferInfo(self.hndl, &mut csbi)
        } {
            return Error::ffi_err("GetConsoleScreenBufferInfo failed");
        }
        let init_size = winapi::COORD {
            X: csbi.srWindow.Right - csbi.srWindow.Left + 1,
            Y: csbi.srWindow.Bottom - csbi.srWindow.Top + 1,
        };
        let window_offset = winapi::COORD {
            X: csbi.srWindow.Left,
            Y: csbi.srWindow.Top,
        };

        // Set the origin of the window to the origin of the buffer.
        if window_offset.X != 0 || window_offset.Y != 0 {
            let pos = winapi::SMALL_RECT {
                Left: 0,
                Top: 0,
                Right: init_size.X - 1,
                Bottom: init_size.Y - 1,
            };
            if 0 == unsafe {
                kernel32::SetConsoleWindowInfo(self.hndl, 1, &pos)
            } {
                return Error::ffi_err("SetConsoleWindowInfo failed");
            }
        }

        // Set the buffer size to (window width+1, window height+1).
        // This allows resizing to a larger size, while minimizing the
        // potential for layout changes caused by scrolling.
        let max_win =
            unsafe { kernel32::GetLargestConsoleWindowSize(self.hndl) };
        if max_win.X == 0 && max_win.Y == 0 {
            return Error::ffi_err("GetLargestConsoleWindowSize failed");
        }
        let size = winapi::COORD {
            X: ::std::cmp::min(init_size.X + 1, max_win.X),
            Y: ::std::cmp::min(init_size.Y + 1, max_win.Y),
        };
        if size.X != self.size.X || size.Y != self.size.Y {
            if 0 == unsafe {
                kernel32::SetConsoleScreenBufferSize(self.hndl, size)
            } {
                return Error::ffi_err("SetConsoleScreenBufferSize failed");
            }
            self.size = size;
            return Ok(true);
        }
        Ok(false)
    }
}

fn process_mouse(_: &winapi::MOUSE_EVENT_RECORD) -> Option<InputEvent> {
    // XXX
    None
}

fn surrogate_pair(s1: u16, kevt: &winapi::KEY_EVENT_RECORD) -> InputEvent {
    let s1 = s1 as u32;
    let s2 = kevt.UnicodeChar as u32;
    let mut utf8 = [0u8; 4];
    let c: u32 = 0x10000 | ((s1 - 0xd800) << 10) | (s2 - 0xdc00);
    let c = ::std::char::from_u32(c).unwrap();
    let len = c.encode_utf8(&mut utf8).len();
    InputEvent::Key(
        Key::Char(utf8, len as u8),
        Mods::win32(kevt.dwControlKeyState),
    )
}

fn process_key(input: &winapi::KEY_EVENT_RECORD) -> Option<InputEvent> {
    use input::CTRL;

    if input.bKeyDown == 0 {
        return None;
    }

    let uc = input.UnicodeChar;
    let mods = Mods::win32(input.dwControlKeyState);
    let (key, mods) = match input.wVirtualKeyCode {
        0x08 => (Key::BS, mods),
        0x09 => (Key::Tab, mods),
        0x0d => (Key::Enter, mods),
        0x1b => (Key::Esc, mods),
        0x21 => (Key::PgUp, mods),
        0x22 => (Key::PgDn, mods),
        0x23 => (Key::End, mods),
        0x24 => (Key::Home, mods),
        0x25 => (Key::Left, mods),
        0x26 => (Key::Up, mods),
        0x27 => (Key::Right, mods),
        0x28 => (Key::Down, mods),
        0x2d => (Key::Ins, mods),
        0x2e => (Key::Del, mods),
        0x70 => (Key::F1, mods),
        0x71 => (Key::F2, mods),
        0x72 => (Key::F3, mods),
        0x73 => (Key::F4, mods),
        0x74 => (Key::F5, mods),
        0x75 => (Key::F6, mods),
        0x76 => (Key::F7, mods),
        0x77 => (Key::F8, mods),
        0x78 => (Key::F9, mods),
        0x79 => (Key::F10, mods),
        0x7a => (Key::F11, mods),
        0x7b => (Key::F12, mods),
        _ => {
            if uc == 0 {
                return None;
            } else if uc < 0x80 {
                match uc {
                    3 => return Some(InputEvent::Interrupt),
                    8 => (Key::BS, mods - CTRL),
                    9 => (Key::Tab, mods - CTRL),
                    13 => (Key::Enter, mods - CTRL),
                    27 => (Key::Esc, mods - CTRL),
                    b if b < 32 => (Key::ascii(b as u8 + 64), mods | CTRL),
                    _ => (Key::Char([uc as u8, 0, 0, 0], 1), mods),
                }
            } else if uc < 0x800 {
                let key = Key::Char(
                    [0xc0 | (uc >> 6) as u8, 0x80 | (uc & 0x3f) as u8, 0, 0],
                    2,
                );
                (key, mods)
            } else {
                // Surrogate pairs have already been processed.
                let key = Key::Char(
                    [
                        0xe0 | (uc >> 12) as u8,
                        0x80 | ((uc >> 6) & 0x3f) as u8,
                        0x80 | (uc & 0x3f) as u8,
                        0,
                    ],
                    3,
                );
                (key, mods)
            }
        }
    };
    Some(InputEvent::Key(key, mods))
}
