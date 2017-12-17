use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use std::sync::mpsc::Sender;
use input::Event;
use {Coords, Result};

pub use tvis_util::size::WinSize;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;
#[cfg(not(windows))]
#[path = "unix.rs"]
mod platform;

#[cfg(windows)]
use self::platform::Term;
#[cfg(not(windows))]
use self::platform::Term;

static TERM: AtomicBool = ATOMIC_BOOL_INIT;

bitflags! {
    #[derive(Default)]
    pub struct Style: u32 {
        const BOLD = 1;
        const ITALIC = 2;
        const UNDERLINE = 4;
    }
}

impl Style {
    fn count(&self) -> u32 {
        self.bits.count_ones()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Default,
    Palette(u8),
    TrueColor(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Color {
        Color::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UseTruecolor {
    /// Always support truecolor.
    Always,
    /// Support truecolor if the environment variable `COLORTERM` is
    /// set to 'truecolor' or '24bit', or if the terminal's terminfo
    /// description contains the `setf24`/`setb24` user-defined
    /// capabilities.
    Auto,
    /// Never support truecolor.
    Never,
}

/// The [Linux console](https://en.wikipedia.org/wiki/Linux_console)
/// and the Windows legacy console can be thought of as having either
/// 16 foreground colors, or 8 foreground colors plus bold.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoldOrBright {
    /// 8 foreground colors plus bold.
    Bold,
    /// 16 foreground colors.
    Bright,
}

pub trait Terminal {
    fn is_tty_input(&self) -> bool;
    fn is_tty_output(&self) -> bool;
    fn start_input(&mut self) -> Result<()>;
    fn supported_styles(&self) -> Style;
    fn max_colors(&self) -> (usize, usize, bool);
    fn get_size(&self) -> Result<WinSize>;
    fn get_style(&self) -> Style;
    fn get_fg(&self) -> Color;
    fn get_bg(&self) -> Color;
    fn set_style(&mut self, style: Style, fg: Color, bg: Color) -> Result<()>;
    fn set_cursor(&mut self, coords: Coords) -> Result<()>;
    fn cursor_visible(&mut self, visible: bool) -> Result<()>;
    fn write(&mut self, text: &str) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
    fn flush_output(&mut self) -> Result<()>;
    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str);
}

pub fn connect(
    use_tc: UseTruecolor,
    b_b: BoldOrBright,
) -> Result<Box<Terminal>> {
    Term::connect(None, use_tc, b_b)
}

pub fn connect_with_input(
    tx: Sender<Box<Event>>,
    use_tc: UseTruecolor,
    b_b: BoldOrBright,
) -> Result<Box<Terminal>> {
    Term::connect(Some(tx), use_tc, b_b)
}
