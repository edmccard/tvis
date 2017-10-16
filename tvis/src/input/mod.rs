use std::any::Any;
use std::fmt;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;
#[cfg(not(windows))]
#[path = "unix/mod.rs"]
mod platform;

pub(crate) use self::platform::start_threads;
#[cfg(windows)]
pub(crate) use self::platform::ScreenSize;

use Coords;

pub trait Event: fmt::Debug + Send {
    fn as_any(&self) -> &Any;
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum Key {
    Char(char, [u8; 4], usize),
    Err([u8; 4], usize),
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    BS,
    Tab,
    Enter,
    Ins,
    Del,
    Home,
    End,
    PgUp,
    PgDn,
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
impl Key {
    fn ascii(byte: u8) -> Key {
        Key::Char(
            unsafe { ::std::char::from_u32_unchecked(byte as u32) },
            [byte, 0, 0, 0],
            1,
        )
    }

    fn empty() -> Key {
        Key::Char('\x00', [0, 0, 0, 0], 0)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Char(c, _, _) => {
                write!(f, "{}", c)
            }
            Key::Err(ref bytes, len) => {
                write!(f, "{:?}", &bytes[0..len as usize])
            }
            k => write!(f, "{:?}", k),
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Mods: u8 {
        const SHIFT = 0b001;
        const ALT = 0b010;
        const CTRL = 0b100;
        const CTRL_ALT = Self::CTRL.bits | Self::ALT.bits;
    }
}


// 0b001 = Shift
// 0b010 = Alt
// 0b100 = Control
impl fmt::Display for Mods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self.bits {
            1 => "Shift-",
            2 => "Alt-",
            3 => "Alt-Shift-",
            4 => "Ctrl-",
            5 => "Ctrl-Shift-",
            6 => "Ctrl-Alt-",
            7 => "Ctrl-Alt-Shift-",
            _ => "",
        };
        write!(f, "{}", val)
    }
}

#[allow(dead_code)]
impl Mods {
    #[cfg(windows)]
    fn win32(ckeys: u32) -> Mods {
        let mut mods: u8 = 0;
        if ckeys & 0b1_0000 != 0 {
            mods += 1;
        }
        if ckeys & 0b11 != 0 {
            mods += 2;
        }
        if ckeys & 0b1100 != 0 {
            mods += 4;
        }
        Mods::from_bits(mods).unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Unknown,
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonMotion {
    Press,
    Release,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WheelMotion {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputEvent {
    Repaint,
    Interrupt,
    Break,
    Mouse(ButtonMotion, MouseButton, Mods, Coords),
    MouseWheel(WheelMotion, Mods, Coords),
    MouseMove(Mods, Coords),
    Key(Key, Mods),
}

impl Event for InputEvent {
    fn as_any(&self) -> &Any {
        self
    }
}
