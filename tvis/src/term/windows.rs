#![cfg(windows)]

use std::ptr;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use winapi;
use kernel32;
use tvis_util::{ConsoleMode, Handle};
use tvis_util::size::get_screen_buffer_size;
use tvis_util::color;
use input::Event;
use term::{BoldOrBright, Color, Style, Terminal, UseTruecolor, WinSize, TERM};
use {Coords, Error, Result};

// winapi omits this.
const ENABLE_VIRTUAL_TERMINAL_INPUT: winapi::DWORD = 0x0200;

// Translate from windows console colors to ANSI terminal colors.
const CONSOLE_COLORS: [winapi::USHORT; 16] =
    [0, 4, 2, 6, 1, 5, 3, 7, 8, 12, 10, 14, 9, 13, 11, 15];

#[derive(Default)]
struct Styles {
    mode: ConsoleMode,
    def_clrs: color::CPair,
    colors: (usize, usize, bool),
    supported: Style,
    bold: bool,
    fg: Color,
    bg: Color,
    style: Style,
}

impl Styles {
    fn new(mode: ConsoleMode, use_tc: bool, bold: bool) -> Styles {
        let mut styles = Styles {
            mode,
            bold,
            ..Default::default()
        };
        match mode {
            ConsoleMode::Legacy => {
                styles.def_clrs = color::default_colors();
                let fgs = if styles.bold { 8 } else { 16 };
                styles.colors = (fgs, 16, use_tc);
                if styles.bold {
                    styles.supported = Style::BOLD;
                }
            }
            ConsoleMode::Win10 => {
                styles.colors = (256, 256, use_tc);
                styles.supported = Style::BOLD | Style::UNDERLINE;
            }
            _ => (),
        }
        styles
    }

    fn supported_styles(&self) -> Style {
        self.supported
    }

    fn max_colors(&self) -> (usize, usize, bool) {
        self.colors
    }

    fn get_style(&self) -> Style {
        if self.bold {
            let curr_fi = Styles::cidx(self.fg, self.def_clrs.0);
            if curr_fi > 7 {
                Style::BOLD
            } else {
                Style::empty()
            }
        } else {
            self.style
        }
    }

    fn get_fg(&self) -> Color {
        if self.bold {
            let curr_fi = Styles::cidx(self.fg, self.def_clrs.0);
            if curr_fi > 7 {
                Color::Palette((curr_fi - 8) as u8)
            } else {
                self.fg
            }
        } else {
            self.fg
        }
    }

    fn set_style(
        &mut self,
        h: winapi::HANDLE,
        fg: Color,
        bg: Color,
        style: Style,
    ) -> Result<()> {
        match self.mode {
            ConsoleMode::Legacy => self.style_legacy(h, fg, bg, style),
            ConsoleMode::Win10 => self.style_win10(h, fg, bg, style),
            _ => Ok(()),
        }
    }

    fn cidx(c: Color, default: u16) -> u16 {
        match c {
            Color::Default => default,
            Color::Palette(i) => i as u16,
            _ => unreachable!(),
        }
    }

    fn style_legacy(
        &mut self,
        h: winapi::HANDLE,
        fg: Color,
        bg: Color,
        style: Style,
    ) -> Result<()> {
        let curr_fi = Styles::cidx(self.fg, self.def_clrs.0);
        let curr_bi = Styles::cidx(self.bg, self.def_clrs.1);
        let mut fi = match Styles::cidx(fg, self.def_clrs.0) {
            c if (c as usize) < self.colors.0 => c,
            _ => curr_fi,
        };
        let bi = match Styles::cidx(bg, self.def_clrs.1) {
            c if (c as usize) < self.colors.1 => c,
            _ => curr_bi,
        };
        if self.bold {
            if style.contains(Style::BOLD) {
                fi |= 0b1000;
            } else {
                fi &= 0b0111;
            }
        }
        if fi != curr_fi || bi != curr_bi {
            self.fg = Color::Palette(fi as u8);
            self.bg = Color::Palette(bi as u8);
            let fi = CONSOLE_COLORS[fi as usize];
            let bi = CONSOLE_COLORS[bi as usize];
            color::set_colors(h, (fi, bi));
        }
        Ok(())
    }

    fn style_win10(
        &mut self,
        h: winapi::HANDLE,
        fg: Color,
        bg: Color,
        style: Style,
    ) -> Result<()> {
        let clears = self.style - style;
        let sets = style - self.style;
        if clears.contains(Style::BOLD)
            || (1 + style.count()) <= (clears.count() + sets.count())
        {
            self.sgr0(h)?;
            self.style_win10(h, fg, bg, style)?;
        } else {
            if clears.contains(Style::UNDERLINE) {
                write_handle(h, "\x1b[24m")?;
                self.style.remove(Style::UNDERLINE);
            }
            if sets.contains(Style::BOLD) {
                write_handle(h, "\x1b[1m")?;
                self.style.insert(Style::BOLD);
            }
            if sets.contains(Style::UNDERLINE) {
                write_handle(h, "\x1b[4m")?;
                self.style.insert(Style::UNDERLINE);
            }
            if fg == Color::Default && fg != self.fg {
                self.fg = Color::Default;
                write_handle(h, "\x1b[39m")?;
            }
            if bg == Color::Default && bg != self.bg {
                self.bg = Color::Default;
                write_handle(h, "\x1b[49m")?;
            }
            if fg != self.fg {
                match fg {
                    Color::Palette(i) => {
                        write_handle(h, &Styles::palette_string(true, i))?;
                        self.fg = Color::Palette(i);
                    }
                    Color::TrueColor(r, g, b) => {
                        write_handle(h, &Styles::tc_string(true, r, g, b))?;
                        self.fg = fg;
                    }
                    _ => (),
                }
            }
            if bg != self.bg {
                match bg {
                    Color::Palette(i) => {
                        write_handle(h, &Styles::palette_string(false, i))?;
                        self.bg = bg;
                    }
                    Color::TrueColor(r, g, b) => {
                        write_handle(h, &Styles::tc_string(false, r, g, b))?;
                        self.bg = bg;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn palette_string(foreground: bool, c: u8) -> String {
        let mut color = String::from("\x1b[");
        if foreground {
            color.push('3')
        } else {
            color.push('4')
        }
        color.push_str("8;5;");
        color.push_str(&c.to_string());
        color.push('m');
        color
    }

    fn tc_string(foreground: bool, r: u8, g: u8, b: u8) -> String {
        let mut color = String::from("\x1b[");
        if foreground {
            color.push('3');
        } else {
            color.push('4');
        }
        color.push_str("8;2;");
        color.push_str(&r.to_string());
        color.push(';');
        color.push_str(&g.to_string());
        color.push(';');
        color.push_str(&b.to_string());
        color.push('m');
        color
    }

    fn sgr0(&mut self, h: winapi::HANDLE) -> Result<()> {
        self.style = Style::empty();
        self.fg = Color::Default;
        self.bg = Color::Default;
        write_handle(h, "\x1b[0m")?;
        Ok(())
    }
}

pub(in term) struct Term {
    styles: Styles,
    in_hndl: winapi::HANDLE,
    out_hndl: winapi::HANDLE,
    init_out_hndl: winapi::HANDLE,
    init_in_mode: Option<winapi::DWORD>,
    tx: Option<Sender<Box<Event>>>,
    cmode: (ConsoleMode, ConsoleMode),
    init_cp: (winapi::UINT, winapi::UINT),
    cursor_height: winapi::DWORD,
    cursor_visibility: bool,
}

impl Term {
    pub(in term) fn connect(
        tx: Option<Sender<Box<Event>>>,
        use_tc: UseTruecolor,
        b_b: BoldOrBright,
    ) -> Result<Box<Terminal>> {
        if TERM.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let out_mode = Handle::Stdout.console_mode();
        let tc =
            (out_mode == ConsoleMode::Win10) && (use_tc != UseTruecolor::Never);
        let bold =
            (out_mode == ConsoleMode::Legacy) && (b_b == BoldOrBright::Bold);
        let mut term = Term {
            styles: Styles::new(out_mode, tc, bold),
            in_hndl: Handle::Stdin.win_handle(),
            out_hndl: ptr::null_mut(),
            init_out_hndl: Handle::Stdout.win_handle(),
            init_in_mode: None,
            tx,
            cmode: (out_mode, Handle::Stdin.console_mode()),
            init_cp: (0, 0),
            cursor_height: 0,
            cursor_visibility: true,
        };
        term.set_mode()?;
        term.set_buffer()?;
        term.set_cp()?;
        Ok(Box::new(term))
    }

    fn set_mode(&mut self) -> Result<()> {
        let mut init_in_mode: winapi::DWORD = 0;
        if 0 == unsafe {
            kernel32::GetConsoleMode(self.in_hndl, &mut init_in_mode)
        } {
            return Error::ffi_err("GetConsoleMode failed");
        }
        let mut in_mode = init_in_mode | winapi::ENABLE_MOUSE_INPUT
            | winapi::ENABLE_WINDOW_INPUT & !winapi::ENABLE_PROCESSED_INPUT;
        if self.cmode.1 == ConsoleMode::Win10 {
            in_mode = in_mode & !winapi::ENABLE_QUICK_EDIT_MODE
                & !ENABLE_VIRTUAL_TERMINAL_INPUT
                | winapi::ENABLE_EXTENDED_FLAGS;
        }
        if 0 == unsafe {
            kernel32::SetConsoleMode(self.in_hndl, in_mode)
        } {
            return Error::ffi_err("SetConsoleMode failed");
        }
        self.init_in_mode = Some(init_in_mode);
        Ok(())
    }

    fn set_buffer(&mut self) -> Result<()> {
        let hndl = unsafe {
            kernel32::CreateConsoleScreenBuffer(
                winapi::GENERIC_READ | winapi::GENERIC_WRITE,
                winapi::FILE_SHARE_READ | winapi::FILE_SHARE_WRITE,
                ptr::null(),
                winapi::CONSOLE_TEXTMODE_BUFFER,
                ptr::null_mut(),
            )
        };
        if hndl == winapi::INVALID_HANDLE_VALUE {
            return Error::ffi_err("CreateConsoleScreenBuffer failed");
        }

        ::input::Resizer::from_hndl(hndl)?;

        if 0 == unsafe { kernel32::SetConsoleActiveScreenBuffer(hndl) } {
            return Error::ffi_err("SetConsoleActiveScreenBuffer failed");
        }

        let mut cci: winapi::CONSOLE_CURSOR_INFO =
            unsafe { ::std::mem::uninitialized() };
        if 0 == unsafe { kernel32::GetConsoleCursorInfo(hndl, &mut cci) } {
            return Error::ffi_err("GetConsoleCursorInfo failed");
        }
        self.cursor_height = cci.dwSize;
        self.cursor_visibility = if cci.bVisible == 0 { false } else { true };

        self.out_hndl = hndl;
        Ok(())
    }

    fn set_cp(&mut self) -> Result<()> {
        unsafe {
            self.init_cp =
                (kernel32::GetConsoleOutputCP(), kernel32::GetConsoleCP());
        }
        if 0 == unsafe { kernel32::SetConsoleOutputCP(65_001) } {
            return Error::ffi_err("SetConsoleOutputCP failed");
        }
        if 0 == unsafe { kernel32::SetConsoleCP(65_001) } {
            return Error::ffi_err("SetConsoleCP failed");
        }
        Ok(())
    }

    fn clear_legacy(&mut self) -> Result<()> {
        let size = self.get_size()?;
        let num_blanks: winapi::DWORD = size.rows as u32 * size.cols as u32;
        let mut blanks_written: winapi::DWORD = 0;
        let mut csbi: winapi::CONSOLE_SCREEN_BUFFER_INFO =
            unsafe { ::std::mem::uninitialized() };
        if 0 == unsafe {
            kernel32::FillConsoleOutputCharacterA(
                self.out_hndl,
                32,
                num_blanks,
                winapi::COORD { X: 0, Y: 0 },
                &mut blanks_written,
            )
        } {
            return Error::ffi_err("FillConsoleOutputCharacterA failed");
        }
        if 0 == unsafe {
            kernel32::GetConsoleScreenBufferInfo(self.out_hndl, &mut csbi)
        } {
            return Error::ffi_err("GetConsoleScreenBufferInfo failed");
        }
        if 0 == unsafe {
            kernel32::FillConsoleOutputAttribute(
                self.out_hndl,
                csbi.wAttributes,
                num_blanks,
                winapi::COORD { X: 0, Y: 0 },
                &mut blanks_written,
            )
        } {
            return Error::ffi_err("FillConsoleOutputAttribute failed");
        }
        self.set_cursor((0, 0))?;
        Ok(())
    }

    fn clear_win10(&mut self) -> Result<()> {
        // Using the virtual terminal escape sequence interacts
        // weirdly when the screen is not the same size as the screen
        // buffer.
        // write_handle(self.out_hndl, "\x1b[2J")

        let size = self.get_size()?;
        let num_blanks = size.rows as usize * size.cols as usize;
        self.set_cursor((0, 0))?;
        self.write(&" ".repeat(num_blanks))?;
        self.set_cursor((0, 0))?;

        let vis = self.cursor_visibility;
        self.cursor_visible(vis)
    }
}

impl Terminal for Term {
    fn is_tty_input(&self) -> bool {
        self.cmode.1 == ConsoleMode::Legacy
            || self.cmode.1 == ConsoleMode::Win10
    }

    fn is_tty_output(&self) -> bool {
        self.cmode.0 == ConsoleMode::Legacy
            || self.cmode.1 == ConsoleMode::Win10
    }

    fn start_input(&mut self) -> Result<()> {
        ::input::start_threads(
            self.tx.take().expect("start_input may only be called once"),
        )
    }

    fn supported_styles(&self) -> Style {
        self.styles.supported_styles()
    }

    fn max_colors(&self) -> (usize, usize, bool) {
        self.styles.max_colors()
    }

    fn get_size(&self) -> Result<WinSize> {
        match get_screen_buffer_size(self.out_hndl) {
            Some(ws) => Ok(ws),
            None => Error::ffi_err("GetConsoleScreenBufferInfo failed"),
        }
    }

    fn get_style(&self) -> Style {
        self.styles.get_style()
    }

    fn get_fg(&self) -> Color {
        self.styles.get_fg()
    }

    fn get_bg(&self) -> Color {
        self.styles.bg
    }

    fn set_style(&mut self, style: Style, fg: Color, bg: Color) -> Result<()> {
        self.styles.set_style(self.out_hndl, fg, bg, style)
    }

    fn set_cursor(&mut self, coords: Coords) -> Result<()> {
        if coords.0 > 32_767 || coords.1 > 32_767 {
            panic!("coords out of range");
        }
        let pos = winapi::COORD {
            X: coords.0 as i16,
            Y: coords.1 as i16,
        };
        // No error on invalid coordinates, to match Linux behavior
        // (and to prevent unexpected panics if set_cursor gets called
        // after the screen shrinks but before a repaint event is
        // processed).
        unsafe {
            kernel32::SetConsoleCursorPosition(self.out_hndl, pos)
        };
        Ok(())
    }

    fn cursor_visible(&mut self, visible: bool) -> Result<()> {
        let cci = winapi::CONSOLE_CURSOR_INFO {
            dwSize: self.cursor_height,
            bVisible: if visible { 1 } else { 0 },
        };
        if 0 == unsafe {
            kernel32::SetConsoleCursorInfo(self.out_hndl, &cci)
        } {
            return Error::ffi_err("SetConsoleCursorInfo failed");
        }
        self.cursor_visibility = visible;
        Ok(())
    }

    fn write(&mut self, text: &str) -> Result<()> {
        write_handle(self.out_hndl, text)
    }

    fn clear(&mut self) -> Result<()> {
        match self.cmode.0 {
            ConsoleMode::Legacy => self.clear_legacy(),
            ConsoleMode::Win10 => self.clear_win10(),
            _ => Ok(()),
        }
    }

    fn flush_output(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str) {
        let crlf = [13u8, 10u8];
        let mut count: winapi::DWORD = 0;
        let _ = self.write(text);
        unsafe {
            kernel32::WriteConsoleA(
                self.out_hndl,
                crlf.as_ptr() as *const _ as *const winapi::VOID,
                2,
                &mut count,
                ptr::null_mut(),
            );
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        unsafe {
            kernel32::SetConsoleActiveScreenBuffer(self.init_out_hndl);
            kernel32::SetConsoleOutputCP(self.init_cp.0);
            kernel32::SetConsoleCP(self.init_cp.1);
            if let Some(mode) = self.init_in_mode {
                kernel32::SetConsoleMode(self.in_hndl, mode);
            }
        }
    }
}

fn write_handle(hndl: winapi::HANDLE, text: &str) -> Result<()> {
    let mut count: winapi::DWORD = 0;
    if 0 == unsafe {
        kernel32::WriteConsoleA(
            hndl,
            text.as_ptr() as *const _ as *const winapi::VOID,
            text.len() as winapi::DWORD,
            &mut count,
            ptr::null_mut(),
        )
    } {
        return Error::ffi_err("WriteConsoleA failed");
    }
    Ok(())
}
