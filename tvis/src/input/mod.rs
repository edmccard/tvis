use std::any::Any;
use std::fmt;

use Event;

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

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum Key {
    Char([u8; 4], u8),
    Err([u8; 4], u8),
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
        Key::Char([byte, 0, 0, 0], 1)
    }

    fn empty() -> Key {
        Key::Char([0, 0, 0, 0], 0)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str::from_utf8;
        match *self {
            Key::Char(ref bytes, len) => {
                write!(f, "{}", from_utf8(&bytes[0..len as usize]).unwrap())
            }
            Key::Err(ref bytes, len) => {
                write!(f, "{:?}", &bytes[0..len as usize])
            }
            k => write!(f, "{:?}", k),
        }
    }
}


// 0b001 = Shift
// 0b010 = Alt
// 0b100 = Control
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Mod {
    mods: u8,
}

impl fmt::Debug for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self.mods {
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
impl Mod {
    fn raw(mods: u8) -> Mod {
        Mod { mods }
    }

    fn none() -> Mod {
        Mod { mods: 0 }
    }

    fn ctrl() -> Mod {
        Mod { mods: 4 }
    }

    fn alt() -> Mod {
        Mod { mods: 2 }
    }

    fn shift() -> Mod {
        Mod { mods: 1 }
    }

    fn ctrl_alt() -> Mod {
        Mod { mods: 6 }
    }

    fn add_alt(&self) -> Mod {
        Mod {
            mods: self.mods | 2,
        }
    }

    fn add_ctrl(&self) -> Mod {
        Mod {
            mods: self.mods | 4,
        }
    }

    fn sub_ctrl(&self) -> Mod {
        Mod {
            mods: self.mods & !4,
        }
    }

    #[cfg(windows)]
    fn win32(ckeys: u32) -> Mod {
        let mut mods: u8 = 0;
        if ckeys & 0b10000 != 0 {
            mods += 1;
        }
        if ckeys & 0b11 != 0 {
            mods += 2;
        }
        if ckeys & 0b1100 != 0 {
            mods += 4;
        }
        Mod { mods }
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
    Mouse(ButtonMotion, MouseButton, Mod, Coords),
    MouseWheel(WheelMotion, Mod),
    MouseMove(Mod, Coords),
    Key(Key, Mod),
}

impl Event for InputEvent {
    fn as_any(&self) -> &Any {
        self
    }
}
