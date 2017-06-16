#![cfg(windows)]

use libc;

pub const FOREGROUND_INTENSITY: u16 = 0x0008;

pub type Bool = i32;
pub type Handle = *mut libc::c_void;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
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
    pub fn GetConsoleScreenBufferInfo(
        console_output: Handle,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn SetConsoleCursorPosition(
        console_output: Handle,
        cursor_position: Coord
    ) -> Bool;
    pub fn SetConsoleTextAttribute(
        console_output: Handle,
        attributes: u16,
    ) -> Bool;
}
