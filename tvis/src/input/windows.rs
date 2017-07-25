#![cfg(windows)]

use std::{ptr, result, time, thread};
use std::sync::mpsc::{Sender, channel};

use tvis_util::Handle;
use win32;
use input::InputEvent;
use {Error, Event, Result};

const SHUTDOWN_KEY: u16 = 0x1111;
const SIGINT_KEY: u16 = 0x2222;
const SIGQUIT_KEY: u16 = 0x3333;

pub(crate) fn start_threads(tx: Sender<Box<Event>>) -> Result<()> {
    register_ctrl_handler()?;
    let (init_tx, init_rx) = channel();
    thread::spawn(move || unsafe {
        create_session_wnd()
            .and_then(|_| register_layout_hook())
            .and_then(|_| {
                init_tx.send(Ok(()));
                run_message_pump();
                Ok(())
            })
            .or_else(|e| init_tx.send(Err(e)));
    });
    init_rx.recv().unwrap()?;
    thread::spawn(move || raw_event_loop(tx));
    Ok(())
}

fn register_ctrl_handler() -> Result<()> {
    extern "C" fn handler(ctrl_type: u32) -> win32::Bool {
        match ctrl_type {
            win32::CTRL_C_EVENT => {
                write_fake_key(SIGINT_KEY);
                1
            }
            win32::CTRL_BREAK_EVENT => {
                write_fake_key(SIGQUIT_KEY);
                1
            }
            win32::CTRL_CLOSE_EVENT => {
                write_fake_key(SHUTDOWN_KEY);
                thread::sleep(time::Duration::from_secs(5));
                0
            }
            _ => 0,
        }
    }

    match unsafe { win32::SetConsoleCtrlHandler(handler, 1) } {
        0 => Error::ffi_err("SetConsoleCtrlHandler failed"),
        _ => Ok(()),
    }
}

unsafe fn create_session_wnd() -> Result<()> {
    extern "C" fn wnd_proc(
        hwnd: win32::Handle,
        msg: u32,
        wparam: u32,
        lparam: usize,
    ) -> usize {
        match msg {
            win32::WM_ENDSESSION => {
                write_fake_key(SHUTDOWN_KEY);
                thread::sleep(time::Duration::from_secs(5));
                0
            }
            _ => unsafe { win32::DefWindowProcA(hwnd, msg, wparam, lparam) },
        }
    }

    let mut wnd_class: win32::WndClassA = Default::default();
    wnd_class.wnd_proc = Some(wnd_proc);
    wnd_class.instance = win32::GetModuleHandleA(ptr::null());
    if wnd_class.instance.is_null() {
        return Error::ffi_err("GetModuleHandle failed");
    }
    wnd_class.class_name = "HiddenShutdownClass\x00".as_ptr();
    if 0 == win32::RegisterClassA(&wnd_class) {
        return Error::ffi_err("RegisterClass failed");
    }
    let hwnd = win32::CreateWindowExA(
        0,
        "HiddenShutdownClass\x00".as_ptr(),
        ptr::null(),
        0,
        0,
        0,
        0,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        win32::GetModuleHandleA(ptr::null()),
        ptr::null(),
    );
    if hwnd.is_null() {
        return Error::ffi_err("CreateWindowEx failed");
    }
    Ok(())
}

fn register_layout_hook() -> Result<()> {
    extern "C" fn layout_hook(
        _: win32::Handle,
        _: u32,
        hwnd: win32::Handle,
        _: i32,
        _: i32,
        _: u32,
        _: u32,
    ) {
        // Filter out events from consoles in other processes.
        if hwnd != unsafe { win32::GetConsoleWindow() } {
            return;
        }
        // Use an "empty" window buffer size event as a resize
        // notification.
        let mut ir: win32::InputRecord = Default::default();
        ir.event_type = win32::WINDOW_BUFFER_SIZE_EVENT;
        let con_hndl = Handle::Stdin.win_handle();
        let mut write_count = 0u32;
        unsafe {
            win32::WriteConsoleInputW(con_hndl, &ir, 1, &mut write_count);
        }
    }

    let hook = unsafe {
        win32::SetWinEventHook(
            win32::EVENT_CONSOLE_LAYOUT,
            win32::EVENT_CONSOLE_LAYOUT,
            ptr::null_mut(),
            layout_hook,
            // Listen for events from all threads/processes and filter
            // in the callback, because there doesn't seem to be a way
            // to get the id for the thread that actually delivers
            // WinEvents for the console (it's not the thread returned
            // by GetWindowThreadProcessId(GetConsoleWindow())).
            0,
            0,
            win32::WINEVENT_OUTOFCONTEXT | win32::WINEVENT_SKIPOWNTHREAD,
        )
    };

    if hook.is_null() {
        return Error::ffi_err("SetWinEventHook failed");
    }
    Ok(())
}

// Windows events and WinEvents require a thread with a message pump.
unsafe fn run_message_pump() {
    let mut msg: win32::Msg = Default::default();
    while 0 != win32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) {
        win32::TranslateMessage(&msg);
        win32::DispatchMessageW(&msg);
    }
}

fn write_fake_key(key_code: u16) -> win32::Bool {
    let mut key: win32::InputRecord = Default::default();
    key.event_type = win32::KEY_EVENT;
    {
        let key = unsafe { key.as_key_event_mut() };
        key.keydown = 1;
        key.virtual_key_code = key_code;
    }
    let con_hndl = Handle::Stdin.win_handle();
    let mut write_count = 0u32;
    unsafe { win32::WriteConsoleInputW(con_hndl, &key, 1, &mut write_count) }
}

fn raw_event_loop(tx: Sender<Box<Event>>) {
    let mut scrn_size = match ScreenSize::from_conout() {
        Ok(c) => c,
        _ => return,
    };
    let in_hndl = Handle::Stdin.win_handle();
    let mut buffer: [win32::InputRecord; 128] = [Default::default(); 128];
    loop {
        let mut read_count = 0u32;
        let ok = unsafe {
            win32::ReadConsoleInputW(
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
            if input.event_type == win32::FOCUS_EVENT ||
                input.event_type == win32::MENU_EVENT
            {
                continue;
            }
            let event = match input.event_type {
                win32::MOUSE_EVENT => {
                    let mevt = unsafe { input.as_mouse_event() };
                    process_mouse(mevt)
                }
                win32::KEY_EVENT => {
                    let kevt = unsafe { input.as_key_event() };
                    match kevt.virtual_key_code {
                        // XXXX
                        0x1b => return,
                        SHUTDOWN_KEY => return,
                        SIGINT_KEY => Some(Box::new(InputEvent::Interrupt)),
                        SIGQUIT_KEY => Some(Box::new(InputEvent::Break)),
                        _ => process_key(kevt),
                    }
                }
                win32::WINDOW_BUFFER_SIZE_EVENT => {
                    match scrn_size.update() {
                        Ok(true) => Some(Box::new(InputEvent::Repaint)),
                        Ok(false) => None,
                        Err(_) => return,
                    }
                }
                _ => unreachable!(),
            };
            if let Some(event) = event {
                if tx.send(event).is_err() {
                    return;
                }
            }
        }
    }
}

pub(crate) struct ScreenSize {
    hndl: win32::Handle,
    size: win32::Coord,
}

impl ScreenSize {
    fn from_conout() -> Result<ScreenSize> {
        let hndl = unsafe {
            win32::CreateFileA(
                "CONOUT$\x00".as_ptr(),
                win32::GENERIC_READ | win32::GENERIC_WRITE,
                win32::FILE_SHARE_READ | win32::FILE_SHARE_WRITE,
                ptr::null(),
                win32::OPEN_EXISTING,
                0,
                ptr::null_mut(),
            )
        };
        if hndl == win32::INVALID_HANDLE_VALUE {
            return Error::ffi_err("CreateFileA failed");
        }
        ScreenSize::from_hndl(hndl)
    }

    pub(crate) fn from_hndl(hndl: win32::Handle) -> Result<ScreenSize> {
        let mut scrn_size = ScreenSize {
            hndl,
            size: win32::Coord { x: -1, y: -1 },
        };
        scrn_size.update()?;
        Ok(scrn_size)
    }

    fn update(&mut self) -> Result<bool> {
        let mut csbi: win32::ConsoleScreenBufferInfo = Default::default();
        csbi.load_from_hndl(self.hndl)?;
        let init_size = csbi.window_size();

        // Set the origin of the window to the origin of the buffer.
        if csbi.window_offset() != (win32::Coord { x: 0, y: 0 }) {
            let pos = win32::SmallRect {
                left: 0,
                top: 0,
                right: init_size.x - 1,
                bottom: init_size.y - 1,
            };
            if 0 == unsafe { win32::SetConsoleWindowInfo(self.hndl, 1, &pos) } {
                return Error::ffi_err("SetConsoleWindowInfo failed");
            }
        }

        // Set the buffer size to (window width+1, window height+1).
        // This allows resizing to a larger size, while minimizing the
        // potential for layout changes caused by scrolling.
        let max_win = unsafe { win32::GetLargestConsoleWindowSize(self.hndl) };
        if max_win.x == 0 && max_win.y == 0 {
            return Error::ffi_err("GetLargestConsoleWindowSize failed");
        }
        let size = win32::Coord {
            x: ::std::cmp::min(init_size.x + 1, max_win.x),
            y: ::std::cmp::min(init_size.y + 1, max_win.y),
        };
        if size != self.size {
            if 0 ==
                unsafe { win32::SetConsoleScreenBufferSize(self.hndl, size) }
            {
                return Error::ffi_err("SetConsoleScreenBufferSize failed");
            }
            self.size = size;
            return Ok(true);
        }
        Ok(false)
    }
}

fn reset_buffer_size(new_size: win32::Coord, hndl: win32::Handle) {
    let max_win = unsafe { win32::GetLargestConsoleWindowSize(hndl) };
    if max_win.x == 0 && max_win.y == 0 {
        return;
    }
    let size = win32::Coord {
        x: ::std::cmp::min(new_size.x + 1, max_win.x),
        y: ::std::cmp::min(new_size.y + 1, max_win.y),
    };
    unsafe {
        win32::SetConsoleScreenBufferSize(hndl, size);
    }
}

fn process_mouse(input: &win32::MouseEventRecord) -> Option<Box<InputEvent>> {
    None
}

fn process_key(input: &win32::KeyEventRecord) -> Option<Box<InputEvent>> {
    if input.keydown == 0 {
        return None;
    }
    Some(Box::new(InputEvent::Key))
}
