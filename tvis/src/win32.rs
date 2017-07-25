#![cfg(windows)]

use std::ptr;
use libc;

use {Error, Result};

pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1;
pub const CTRL_BREAK_EVENT: u32 = 1;
pub const CTRL_C_EVENT: u32 = 0;
pub const CTRL_CLOSE_EVENT: u32 = 2;
pub const ENABLE_WINDOW_INPUT: u32 = 0x0008;
pub const ENABLE_MOUSE_INPUT: u32 = 0x0010;
pub const EVENT_CONSOLE_LAYOUT: u32 = 0x4005;
pub const FILE_SHARE_READ: u32 = 1;
pub const FILE_SHARE_WRITE: u32 = 2;
pub const FOCUS_EVENT: u16 = 0x0010;
pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;
pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
pub const KEY_EVENT: u16 = 0x0001;
pub const MENU_EVENT: u16 = 0x0008;
pub const MOUSE_EVENT: u16 = 0x0002;
pub const OPEN_EXISTING: u32 = 3;
pub const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;
pub const WINEVENT_OUTOFCONTEXT: u32 = 0;
pub const WINEVENT_SKIPOWNTHREAD: u32 = 1;
pub const WM_ENDSESSION: u32 = 0x0016;

pub type Bool = i32;
pub type Handle = *mut libc::c_void;
pub type HandlerRoutine = unsafe extern "C" fn(u32) -> Bool;
pub type WinEventProc = unsafe extern "C" fn(
    Handle,
    u32,
    Handle,
    i32,
    i32,
    u32,
    u32,
);
pub type WndProc = unsafe extern "C" fn(Handle, u32, u32, usize)
    -> usize;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ConsoleScreenBufferInfo {
    pub size: Coord,
    pub cursor_position: Coord,
    pub attributes: u16,
    pub window: SmallRect,
    pub maximum_window_size: Coord,
}

impl ConsoleScreenBufferInfo {
    pub fn load_from_hndl(&mut self, hndl: Handle) -> Result<()> {
        if 0 == unsafe { GetConsoleScreenBufferInfo(hndl, self) } {
            return Error::ffi_err("GetConsoleScreenBufferInfo failed");
        }
        Ok(())
    }

    pub fn window_size(&self) -> Coord {
        Coord {
            x: self.window.right - self.window.left + 1,
            y: self.window.bottom - self.window.top + 1,
        }
    }

    pub fn window_offset(&self) -> Coord {
        Coord {
            x: self.window.left,
            y: self.window.top,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InputRecord {
    pub event_type: u16,
    event: [u32; 4],
}

impl InputRecord {
    pub unsafe fn as_key_event(&self) -> &KeyEventRecord {
        ::std::mem::transmute(&self.event)
    }

    pub unsafe fn as_key_event_mut(&mut self) -> &mut KeyEventRecord {
        ::std::mem::transmute(&mut self.event)
    }

    pub unsafe fn as_mouse_event(&self) -> &MouseEventRecord {
        ::std::mem::transmute(&self.event)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct KeyEventRecord {
    pub keydown: Bool,
    pub repeat_count: u16,
    pub virtual_key_code: u16,
    pub virtual_scan_code: u16,
    pub uchar: u16,
    pub control_key_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MouseEventRecord {
    pub mouse_position: Coord,
    pub button_state: u32,
    pub control_key_state: u32,
    pub event_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Msg {
    hwnd: Handle,
    message: u32,
    wparam: usize,
    lparam: usize,
    time: u32,
    pt: Point,
}

impl Default for Msg {
    fn default() -> Msg {
        Msg {
            hwnd: ptr::null_mut(),
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            pt: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Point {
    x: u32,
    y: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SmallRect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WindowBufferSizeRecord {
    pub size: Coord,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WndClassA {
    pub style: u32,
    pub wnd_proc: Option<WndProc>,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: Handle,
    pub icon: Handle,
    pub cursor: Handle,
    pub background: Handle,
    pub menu_name: *const u8,
    pub class_name: *const u8,
}

impl Default for WndClassA {
    fn default() -> WndClassA {
        WndClassA {
            style: 0,
            wnd_proc: None,
            cls_extra: 0,
            wnd_extra: 0,
            instance: ptr::null_mut(),
            icon: ptr::null_mut(),
            cursor: ptr::null_mut(),
            background: ptr::null_mut(),
            menu_name: ptr::null(),
            class_name: ptr::null(),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
extern "system" {
    pub fn CreateConsoleScreenBuffer(
        desired_access: u32,
        share_mode: u32,
        unused: *const libc::c_void,
        flags: u32,
        reserved: *mut libc::c_void
    ) -> Handle;
    pub fn CreateFileA(
        file_name: *const u8,
        desired_access: u32,
        share_mode: u32,
        unused: *const libc::c_void,
        creation_disposition: u32,
        flags_and_attributes: u32,
        template_file: Handle
    ) -> Handle;
    pub fn CreateWindowExA(
        ex_style: u32,
        class_name: *const u8,
        window_name: *const u8,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Handle,
        menu: Handle,
        instance: Handle,
        param: *const libc::c_void
    ) -> Handle;
    pub fn DefWindowProcA(
        hwnd: Handle,
        msg: u32,
        wparam: u32,
        lparam: usize
    ) -> usize;
    pub fn DispatchMessageW(msg: *const Msg) -> usize;
    pub fn GetConsoleMode(console_handle: Handle, mode: *mut u32) -> Bool;
    pub fn GetConsoleScreenBufferInfo(
        console_output: Handle,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> Bool;
    pub fn GetConsoleWindow() -> Handle;
    pub fn GetLargestConsoleWindowSize(console_output: Handle) -> Coord;
    pub fn GetMessageW(
        msg: *mut Msg,
        hwnd: Handle,
        msg_filter_min: u32,
        msg_filter_max: u32,
    ) -> Bool;
    pub fn GetModuleHandleA(module_name: *const u8) -> Handle;
    pub fn ReadConsoleInputW(
        console_input: Handle,
        buffer: *mut InputRecord,
        length: u32,
        number_of_events_read: *mut u32
    ) -> Bool;
    pub fn RegisterClassA(wnd_class: *const WndClassA) -> u16;
    pub fn SetConsoleActiveScreenBuffer(console_output: Handle) -> Bool;
    pub fn SetConsoleCtrlHandler(
        handler_routine: HandlerRoutine,
        add: Bool
    ) -> Bool;
    pub fn SetConsoleMode(console_handle: Handle, mode: u32) -> Bool;
    pub fn SetConsoleScreenBufferSize(
        console_output: Handle,
        size: Coord
    ) -> Bool;
    pub fn SetConsoleWindowInfo(
        console_output: Handle,
        absolute: Bool,
        console_window: *const SmallRect
    ) -> Bool;
    pub fn SetWinEventHook(
        event_min: u32,
        event_max: u32,
        hmod_win_event_proc: Handle,
        win_event_proc: WinEventProc,
        process: u32,
        thread: u32,
        flags: u32,
    ) -> Handle;
    pub fn TranslateMessage(msg: *const Msg) -> Bool;
    pub fn WriteConsoleInputW(
        console_input: Handle,
        buffer: *const InputRecord,
        length: u32,
        number_of_events_written: *mut u32
    ) -> Bool;
}
