#![cfg(windows)]

use libc;

pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
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

#[cfg_attr(rustfmt, rustfmt_skip)]
extern "system" {
    pub fn GetConsoleMode(console_handle: Handle, mode: *mut u32) -> Bool;
    pub fn GetFileInformationByHandleEx(
        file: Handle,
        file_information_class: i32,
        file_information: *mut libc::c_void,
        buffer_size: u32,
    ) -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn SetConsoleMode(console_handle: Handle, mode: u32) -> Bool;
}