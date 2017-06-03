#![cfg(windows)]

use libc;

pub const FOREGROUND_INTENSITY: u16 = 0x0008;

pub type Bool = i32;
pub type Handle = *mut libc::c_void;

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
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
extern "system" {
    pub fn GetConsoleScreenBufferInfo(
        console_output: Handle,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn SetConsoleTextAttribute(
        console_output: Handle,
        attributes: u16,
    ) -> Bool;
}
