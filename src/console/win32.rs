#![cfg(windows)]
#![allow(dead_code)]

extern "system" {
    pub fn AllocConsole() -> Bool;
    pub fn AttachConsole(process_id: u32) -> Bool;
    pub fn CloseHandle(object: Handle) -> Bool;
    pub fn CreateConsoleScreenBuffer(
        desired_access: u32,
        share_mode: u32,
        security_attributes: *const SecurityAttributes,
        flags: u32,
        screen_buffer_data: *mut Void)
        -> Handle;
    pub fn FillConsoleOutputAttribute(
        console_output: Handle,
        attribute: u16,
        length: u32,
        write_coord: Coord,
        number_of_attrs_written: *mut u32)
        -> Bool;
    pub fn FillConsoleOutputCharacterA(
        console_output: Handle,
        character: Char,
        length: u32,
        write_coord: Coord,
        number_of_chars_written: *mut u32)
        -> Bool;
    pub fn FillConsoleOutputCharacterW(
        console_output: Handle,
        character: WChar,
        length: u32,
        write_coord: Coord,
        number_of_chars_written: *mut u32)
        -> Bool;
    pub fn FlushConsoleInputBuffer(console_input: Handle) -> Bool;
    pub fn FreeConsole() -> Bool;
    pub fn GenerateConsoleCtrlEvent(
        control_event: u32,
        process_group_id: u32)
        -> Bool;
    pub fn GetConsoleCP() -> u32;
    pub fn GetConsoleCursorInfo(
        console_output: Handle,
        console_cursor_info: *mut ConsoleCursorInfo)
        -> Bool;
    pub fn GetConsoleDisplayMode(mode_flags: *mut u32) -> Bool;
    pub fn GetConsoleFontSize(console_output: Handle, font: u32) -> Coord;
    pub fn GetConsoleHistoryInfo(
        console_history_info: *mut ConsoleHistoryInfo)
        -> Bool;
    pub fn GetConsoleMode(console_handle: Handle, mode: *mut u32) -> Bool;
    pub fn GetConsoleOriginalTitleA(
        console_title: *mut Char,
        size: u32)
        -> u32;
    pub fn GetConsoleOriginalTitleW(
        console_title: *mut WChar,
        size: u32)
        -> u32;
    pub fn GetConsoleOutputCP() -> u32;
    pub fn GetConsoleProcessList(
        process_list: *mut u32,
        process_count: u32)
        -> u32;
    pub fn GetConsoleScreenBufferInfo(
        console_output: Handle,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo)
        -> Bool;
    pub fn GetConsoleScreenBufferInfoEx(
        console_output: Handle,
        console_screen_buffer_info_ex: *mut ConsoleScreenBufferInfoEx)
        -> Bool;
    pub fn GetConsoleSelectionInfo(
        console_selection_info: *mut ConsoleSelectionInfo)
        -> Bool;
    pub fn GetConsoleTitleA(console_title: *mut Char, size: u32) -> u32;
    pub fn GetConsoleTitleW(console_title: *mut WChar, size: u32) -> u32;
    pub fn GetConsoleWindow() -> Handle;
    pub fn GetCurrentConsoleFont(
        console_output: Handle,
        maximum_window: Bool,
        console_current_font: *mut ConsoleFontInfo)
        -> Bool;
    pub fn GetCurrentConsoleFontEx(
        console_output: Handle,
        maximum_window: Bool,
        console_current_font_ex: *mut ConsoleFontInfoEx)
        -> Bool;
    pub fn GetLargestConsoleWindowSize(console_output: Handle) -> Coord;
    pub fn GetLastError() -> u32;
    pub fn GetNumberOfConsoleInputEvents(
        console_input: Handle,
        number_of_events: *mut u32)
        -> Bool;
    pub fn GetNumberOfConsoleMouseButtons(
        number_of_mouse_buttons: *mut u32)
        -> Bool;
    pub fn GetStdHandle(std_handle: u32) -> Handle;
    pub fn PeekConsoleInput(
        console_input: Handle,
        buffer: *mut InputRecord,
        length: u32,
        number_of_events_read: *mut u32)
        -> Bool;
    pub fn ReadConsole(
        console_input: Handle,
        buffer: *mut Void,
        number_of_chars_to_read: u32,
        number_of_chars_read: *mut u32,
        input_control: *mut Void)
        -> Bool;
    pub fn ReadConsoleInput(
        console_input: Handle,
        buffer: *mut InputRecord,
        length: u32,
        number_of_events_read: *mut u32)
        -> Bool;
    pub fn ReadConsoleOutputA(
        console_output: Handle,
        buffer: *mut CharInfo,
        buffer_size: Coord,
        buffer_coord: Coord,
        read_region: *mut SmallRect)
        -> Bool;
    pub fn ReadConsoleOutputW(
        console_output: Handle,
        buffer: *mut CharInfo,
        buffer_size: Coord,
        buffer_coord: Coord,
        read_region: *mut SmallRect)
        -> Bool;
    pub fn ReadConsoleOutputAttribute(
        console_output: Handle,
        attribute: *mut u16,
        length: u32,
        read_coord: Coord,
        number_of_attrs_read: *mut u32)
        -> Bool;
    pub fn ReadConsoleOutputCharacterA(
        console_output: Handle,
        character: *mut Char,
        length: u32,
        read_coord: Coord,
        number_of_chars_read: *mut u32)
        -> Bool;
    pub fn ReadConsoleOutputCharacterW(
        console_output: Handle,
        character: *mut WChar,
        length: u32,
        read_coord: Coord,
        number_of_chars_read: *mut u32)
        -> Bool;
    pub fn ScrollConsoleScreenBufferA(
        console_output: Handle,
        scroll_rectangle: *const SmallRect,
        clip_rectangle: *const SmallRect,
        destination_origin: Coord,
        fill: *const CharInfo)
        -> Bool;
    pub fn ScrollConsoleScreenBufferW(
        console_output: Handle,
        scroll_rectangle: *const SmallRect,
        clip_rectangle: *const SmallRect,
        destination_origin: Coord,
        fill: *const CharInfo)
        -> Bool;
    pub fn SetConsoleActiveScreenBuffer(console_output: Handle) -> Bool;
    pub fn SetConsoleCP(code_page_id: u32) -> Bool;
    pub fn SetConsoleCtrlHandler(
        handler_routine: HandlerRoutine,
        add: Bool)
        -> Bool;
    pub fn SetConsoleCursorInfo(
        console_output: Handle,
        console_cursor_info: *const ConsoleCursorInfo)
        -> Bool;
    pub fn SetConsoleCursorPosition(
        console_output: Handle,
        cursor_position: Coord)
        -> Bool;
    pub fn SetConsoleDisplayMode(
        console_output: Handle,
        flags: u32,
        new_screen_buffer_dimensions: *mut Coord)
        -> Bool;
    pub fn SetConsoleHistoryInfo(
        console_history_info: *mut ConsoleHistoryInfo)
        -> Bool;
    pub fn SetConsoleMode(console_handle: Handle, mode: u32) -> Bool;
    pub fn SetConsoleOutputCP(code_page_id: u32) -> Bool;
    pub fn SetConsoleScreenBufferInfoEx(
        console_output: Handle,
        console_screen_buffer_info_ex: *mut ConsoleScreenBufferInfoEx)
        -> Bool;
    pub fn SetConsoleScreenBufferSize(
        console_output: Handle,
        size: Coord)
        -> Bool;
    pub fn SetConsoleTextAttribute(
        console_output: Handle,
        attributes: u16)
        -> Bool;
    pub fn SetConsoleTitleA(console_title: *const Char) -> Bool;
    pub fn SetConsoleTitleW(console_title: *const WChar) -> Bool;
    pub fn SetConsoleWindowInfo(
        console_output: Handle,
        absolute: Bool,
        console_window: *const SmallRect)
        -> Bool;
    pub fn SetCurrentConsoleFontEx(
        console_output: Handle,
        maximum_window: Bool,
        console_current_font_ex: *mut ConsoleFontInfoEx)
        -> Bool;
    pub fn SetStdHandle(std_handle: u32, handle: Handle) -> Bool;
    pub fn WriteConsole(
        console_output: Handle,
        buffer: *const Void,
        number_of_chars_to_write: u32,
        number_of_chars_written: *mut u32,
        reserved: *mut Void)
        -> Bool;
    pub fn WriteConsoleInput(
        console_input: Handle,
        buffer: *const InputRecord,
        length: u32,
        number_of_events_written: *mut u32)
        -> Bool;
    pub fn WriteConsoleOutputA(
        console_output: Handle,
        buffer: *const CharInfo,
        buffer_size: Coord,
        buffer_coord: Coord,
        write_region: *mut SmallRect)
        -> Bool;
    pub fn WriteConsoleOutputW(
        console_output: Handle,
        buffer: *const CharInfo,
        buffer_size: Coord,
        buffer_coord: Coord,
        write_region: *mut SmallRect)
        -> Bool;
    pub fn WriteConsoleOutputAttribute(
        console_output: Handle,
        attribute: *const u16,
        length: u32,
        write_coord: Coord,
        number_of_attrs_written: *mut u32)
        -> Bool;
    pub fn WriteConsoleOutputCharacterA(
        console_output: Handle,
        character: *const Char,
        length: u32,
        write_coord: Coord,
        number_of_chars_written: *mut u32)
        -> Bool;
    pub fn WriteConsoleOutputCharacterW(
        console_output: Handle,
        character: *const WChar,
        length: u32,
        write_coord: Coord,
        number_of_chars_written: *mut u32)
        -> Bool;
}

pub enum Void {}

pub type HandlerRoutine = Option<unsafe extern "system" fn(ctrl_type: u32)
                                                           -> Bool>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CharInfo {
    u_char: [u16; 1],
    attributes: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleCursorInfo {
    size: u32,
    visible: Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleFontInfo {
    font: u32,
    font_size: Coord,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleFontInfoEx {
    size: u32,
    font: u32,
    font_size: Coord,
    font_family: u32,
    font_weight: u32,
    face_name: [WChar; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleHistoryInfo {
    size: u32,
    history_buffer_size: u32,
    number_of_history_buffers: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleScreenBufferInfo {
    size: Coord,
    cursor_position: Coord,
    attributes: u16,
    window: SmallRect,
    maximum_window_size: Coord,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleScreenBufferInfoEx {
    b_size: u32,
    w_size: Coord,
    cursor_position: Coord,
    attributes: u16,
    window: SmallRect,
    maximum_window_size: Coord,
    popup_attributes: u16,
    fullscreen_supported: Bool,
    color_table: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConsoleSelectionInfo {
    flags: u32,
    selection_anchor: Coord,
    selection: SmallRect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Coord {
    x: i16,
    y: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FocusEventRecord {
    set_focus: Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputRecord {
    event_type: u16,
    event: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyEventRecord {
    key_down: Bool,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    u_char: [u16; 1],
    control_key_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MenuEventRecord {
    command_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MouseEventRecord {
    mouse_position: Coord,
    button_state: u32,
    control_key_state: u32,
    event_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecurityAttributes {
    length: u32,
    security_descriptor: *mut Void,
    inherit_handle: Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SmallRect {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WindowBufferSizeRecord {
    size: Coord,
}

pub type Bool = i32;
pub type Char = i8;
pub type WChar = u16;
pub type Handle = *mut Void;

pub const TRUE: Bool = 1;
pub const FALSE: Bool = 0;

pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;

pub const ATTACH_PARENT_PROCESS: u32 = 0xffffffff;

pub const CONSOLE_FULLSCREEN: u32 = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2;

pub const CONSOLE_FULLSCREEN_MODE: u32 = 1;
pub const CONSOLE_WINDOWED_MODE: u32 = 2;

pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1;

pub const CONSOLE_MOUSE_DOWN: u32 = 0x0008;
pub const CONSOLE_MOUSE_SELECTION: u32 = 0x0004;
pub const CONSOLE_NO_SELECTION: u32 = 0x0000;
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 0x0001;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 0x0002;

pub const CTRL_C_EVENT: u32 = 0;
pub const CTRL_BREAK_EVENT: u32 = 1;

pub const ENABLE_ECHO_INPUT: u32 = 0x0004;
pub const ENABLE_EXTENDED_FLAGS: u32 = 0x0080;
pub const ENABLE_INSERT_MODE: u32 = 0x0020;
pub const ENABLE_LINE_INPUT: u32 = 0x0002;
pub const ENABLE_MOUSE_INPUT: u32 = 0x0010;
pub const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
pub const ENABLE_QUICK_EDIT_MODE: u32 = 0x0040;
pub const ENABLE_WINDOW_INPUT: u32 = 0x0008;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

pub const ENABLE_PROCESSED_OUTPUT: u32 = 0x0001;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: u32 = 0x0002;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
pub const DISABLE_NEWLINE_AUTO_RETURN: u32 = 0x0008;
pub const ENABLE_LVB_GRID_WORLDWIDE: u32 = 0x0010;

pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_HANDLE: u32 = 6;
pub const ERROR_INVALID_PARAMETER: u32 = 87;

pub const FF_DECORATIVE: u32 = 0x50;
pub const FF_MODERN: u32 = 0x30;
pub const FF_ROMAN: u32 = 0x10;
pub const FF_SCRIPT: u32 = 0x40;
pub const FF_SWISS: u32 = 0x20;

pub const FILE_SHARE_READ: u32 = 0x00000001;
pub const FILE_SHARE_WRITE: u32 = 0x00000002;

pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;

pub const HISTORY_NO_DUP_FLAG: u32 = 1;

pub const STATUS_INVALID_PARAMETER: u32 = 0xc000000d;

pub const STD_INPUT_HANDLE: u32 = 0xfffffff6;
pub const STD_OUTPUT_HANDLE: u32 = 0xfffffff5;
pub const STD_ERROR_HANDLE: u32 = 0xfffffff4;

pub const TMPF_FIXED_PITCH: u8 = 0x01;
pub const TMPF_VECTOR: u8 = 0x02;
pub const TMPF_TRUETYPE: u8 = 0x04;
pub const TMPF_DEVICE: u8 = 0x08;

pub const FOCUS_EVENT: u16 = 0x0010;
pub const KEY_EVENT: u16 = 0x0001;
pub const MENU_EVENT: u16 = 0x0008;
pub const MOUSE_EVENT: u16 = 0x0002;
pub const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;

pub const CAPSLOCK_ON: u32 = 0x0080;
pub const ENHANCED_KEY: u32 = 0x0100;
pub const LEFT_ALT_PRESSED: u32 = 0x0002;
pub const LEFT_CTRL_PRESSED: u32 = 0x0008;
pub const NUMLOCK_ON: u32 = 0x0020;
pub const RIGHT_ALT_PRESSED: u32 = 0x0001;
pub const RIGHT_CTRL_PRESSED: u32 = 0x0004;
pub const SCROLLLOCK_ON: u32 = 0x0040;
pub const SHIFT_PRESSED: u32 = 0x0010;

pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 0x0001;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 0x0004;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 0x0008;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 0x0010;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 0x0002;

pub const DOUBLE_CLICK: u32 = 0x0002;
pub const MOUSE_HWHEELED: u32 = 0x0008;
pub const MOUSE_MOVED: u32 = 0x0001;
pub const MOUSE_WHEELED: u32 = 0x0004;

pub const FOREGROUND_BLUE: u16 = 0x0001;
pub const FOREGROUND_GREEN: u16 = 0x0002;
pub const FOREGROUND_RED: u16 = 0x0004;
pub const FOREGROUND_INTENSITY: u16 = 0x0008;
pub const BACKGROUND_BLUE: u16 = 0x0010;
pub const BACKGROUND_GREEN: u16 = 0x0020;
pub const BACKGROUND_RED: u16 = 0x0040;
pub const BACKGROUND_INTENSITY: u16 = 0x0080;
pub const COMMON_LVB_LEADING_BYTE: u16 = 0x0100;
pub const COMMON_LVB_TRAILING_BYTE: u16 = 0x0200;
pub const COMMON_LVB_GRID_HORIZONTAL: u16 = 0x0400;
pub const COMMON_LVB_GRID_LVERTICAL: u16 = 0x0800;
pub const COMMON_LVB_GRID_RVERTICAL: u16 = 0x1000;
pub const COMMON_LVB_REVERSE_VIDEO: u16 = 0x4000;
pub const COMMON_LVB_UNDERSCORE: u16 = 0x8000;
