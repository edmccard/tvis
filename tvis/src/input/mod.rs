use std::any::Any;

use Event;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;
#[cfg(not(windows))]
#[path = "unix.rs"]
mod platform;

pub(crate) use self::platform::start_threads;

#[cfg(windows)]
pub(crate) use self::platform::ScreenSize;

#[derive(Debug)]
pub enum InputEvent {
    Repaint,
    Interrupt,
    Break,
    Mouse,
    Key,
}

impl Event for InputEvent {
    fn as_any(&self) -> &Any {
        self
    }
}
