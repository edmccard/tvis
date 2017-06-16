#![cfg(windows)]

use libc;

pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
pub const HKEY_CURRENT_USER: Handle = 0x80000001u32 as i32 as usize as Handle;
pub const KEY_READ: u32 = 0x20019;
pub const MAX_PATH: usize = 260;

pub type Bool = i32;
pub type Handle = *mut libc::c_void;
pub type WChar = u16;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct FileNameInfo {
    pub file_name_length: u32,
    pub file_name: [WChar; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Coord {
    x: i16,
    y: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ConsoleScreenBufferInfo {
    pub size: Coord,
    pub cursor_position: Coord,
    pub attributes: u16,
    pub window: SmallRect,
    pub maximum_window_size: Coord,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SmallRect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
extern "system" {
    pub fn GetConsoleMode(console_handle: Handle, mode: *mut u32) -> Bool;
    pub fn GetConsoleScreenBufferInfo(
        console_output: Handle,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> Bool;
    pub fn GetFileInformationByHandleEx(
        file: Handle,
        file_information_class: i32,
        file_information: *mut libc::c_void,
        buffer_size: u32,
    ) -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn RegOpenKeyExA(
        key: Handle,
        sub_key: *const u8,
        options: u32,
        desired: u32,
        result: *mut *mut libc::c_void,
    ) -> libc::c_long;
    pub fn RegQueryValueExA(
        key: Handle,
        value_name: *const u8,
        reserved: *mut u32,
        _type: *mut u32,
        data: *mut u8,
        data_size: *mut u32
    ) -> libc::c_long;
    pub fn SetConsoleMode(console_handle: Handle, mode: u32) -> Bool;
}
