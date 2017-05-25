#![cfg(windows)]

pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
pub const FOREGROUND_INTENSITY: u16 = 0x0008;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
pub const MAX_PATH: usize = 260;

pub type Bool = i32;
pub type Handle = *mut Void;
pub type WChar = u16;

pub enum Void {}

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
pub struct FileNameInfo {
    pub file_name_length: u32,
    pub file_name: [WChar; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SmallRect {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
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
        file_information: *mut Void,
        buffer_size: u32,
    ) -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn SetConsoleMode(console_handle: Handle, mode: u32) -> Bool;
    pub fn SetConsoleTextAttribute(
        console_output: Handle,
        attributes: u16,
    ) -> Bool;
}
