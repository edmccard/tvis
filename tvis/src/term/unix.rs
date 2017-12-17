#![cfg(not(windows))]

use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use libc;
use tinf::{tparm, Desc};
use tinf::cap::{self, UserDef};
use tvis_util::{Handle, TerminalMode};
use tvis_util::size::get_size;
use input::Event;
use term::{BoldOrBright, Color, Style, Terminal, UseTruecolor, WinSize, TERM};
use {is_rxvt, Coords, Error, Result};

lazy_static! {
    static ref STDOUT: io::Stdout = io::stdout();
}

#[derive(Default)]
struct Styles {
    sgr0: Vec<u8>,
    bold: Vec<u8>,
    sitm: Vec<u8>,
    ritm: Vec<u8>,
    smul: Vec<u8>,
    rmul: Vec<u8>,
    op: Vec<u8>,
    setaf: Vec<u8>,
    setab: Vec<u8>,
    setftc: Vec<u8>,
    setbtc: Vec<u8>,
    vars: ::tinf::Vars,
    colors: usize,
    fg: Color,
    bg: Color,
    style: Style,
    bright16: bool,
}

impl Styles {
    fn new(desc: &Desc, use_tc: UseTruecolor, b_b: BoldOrBright) -> Styles {
        let mut styles: Styles = Default::default();

        // If we can't return to the default style, don't activate
        // style support.
        if desc[cap::sgr0].is_empty() {
            return styles;
        }

        styles.sgr0 = desc[cap::sgr0].to_vec();
        styles.colors = match desc[cap::colors] {
            0xffff => 0,
            c => c as usize,
        };

        // Prioritize colors as colors over colors as styles.
        let ncv = match desc[cap::ncv] {
            0xffff => 0,
            n => n,
        };
        if (styles.colors == 0) || ((ncv & 0x20) == 0) {
            styles.bold = desc[cap::bold].to_vec();
        }
        if (styles.colors == 0) || ((ncv & 0x8000) == 0) {
            styles.sitm = desc[cap::sitm].to_vec();
            styles.ritm = desc[cap::ritm].to_vec();
        }
        if (styles.colors == 0) || ((ncv & 0x02) == 0) {
            styles.smul = desc[cap::smul].to_vec();
            styles.rmul = desc[cap::rmul].to_vec();
        }

        // If we can't return to the default colors, or there aren't
        // any, don't activate color support.
        if desc[cap::op].is_empty() {
            return styles;
        }
        if desc[cap::colors] == 0 || desc[cap::colors] == 0xffff {
            return styles;
        }

        styles.op = desc[cap::op].to_vec();
        styles.setaf = desc[cap::setaf].to_vec();
        styles.setab = desc[cap::setab].to_vec();

        if use_tc == UseTruecolor::Always
            || (Styles::has_truecolor(desc) && use_tc != UseTruecolor::Never)
        {
            styles.setftc = b"\x1b[38;2;%p1%d;%p2%d;%p3%dm".to_vec();
            styles.setbtc = b"\x1b[48;2;%p1%d;%p2%d;%p3%dm".to_vec();
        }

        styles.bright16 = styles.colors == 8 && !styles.bold.is_empty()
            && b_b == BoldOrBright::Bright;
        if styles.bright16 {
            styles.colors = 16;
        }

        styles
    }

    fn has_truecolor(desc: &Desc) -> bool {
        if let Ok(val) = ::std::env::var("COLORTERM") {
            val == "truecolor" || val == "24bit"
        } else {
            !desc.get_str_ext(&UserDef::named("setf24")).is_empty()
        }
    }

    fn supported_styles(&self) -> Style {
        let mut supported = Style::empty();
        if !self.bold.is_empty() && !self.bright16 {
            supported |= Style::BOLD;
        }
        if !self.sitm.is_empty() {
            supported |= Style::ITALIC;
        }
        if !self.smul.is_empty() {
            supported |= Style::UNDERLINE;
        }
        supported
    }

    fn max_colors(&self) -> (usize, usize, bool) {
        let tc = !self.setftc.is_empty();
        let bgs = if self.bright16 {
            8
        } else {
            self.colors
        };
        (self.colors, bgs, tc)
    }

    fn sgr(
        &mut self,
        w: &mut Write,
        fg: Color,
        bg: Color,
        style: Style,
    ) -> Result<()> {
        let mut style = style;
        let mut fg = fg;
        if self.bright16 {
            match fg {
                Color::Palette(i) if i > 7 && i < 16 => {
                    style.insert(Style::BOLD);
                    fg = Color::Palette(i - 8);
                }
                _ => style.remove(Style::BOLD),
            }
        }
        self.sgr_(w, fg, bg, style)
    }

    fn sgr_(
        &mut self,
        w: &mut Write,
        fg: Color,
        bg: Color,
        style: Style,
    ) -> Result<()> {
        if self.sgr0.is_empty() {
            return Ok(());
        }

        let clears = self.style - style;
        let sets = style - self.style;
        if clears.contains(Style::BOLD)
            || (1 + style.count()) <= (clears.count() + sets.count())
        {
            self.sgr0(w)?;
            self.sgr(w, fg, bg, style)?;
        } else {
            if clears.contains(Style::ITALIC) && !self.ritm.is_empty() {
                w.write_all(&self.ritm)?;
                self.style.remove(Style::ITALIC);
            }
            if clears.contains(Style::UNDERLINE) && !self.rmul.is_empty() {
                w.write_all(&self.rmul)?;
                self.style.remove(Style::UNDERLINE);
            }
            if sets.contains(Style::BOLD) && !self.bold.is_empty() {
                w.write_all(&self.bold)?;
                self.style.insert(Style::BOLD);
            }
            if sets.contains(Style::ITALIC) && !self.sitm.is_empty() {
                w.write_all(&self.sitm)?;
                self.style.insert(Style::ITALIC);
            }
            if sets.contains(Style::UNDERLINE) && !self.smul.is_empty() {
                w.write_all(&self.smul)?;
                self.style.insert(Style::UNDERLINE);
            }
            if self.colors == 0 {
                return Ok(());
            }
            if (fg == Color::Default && fg != self.fg)
                || (bg == Color::Default && bg != self.bg)
            {
                self.fg = Color::Default;
                self.bg = Color::Default;
                w.write_all(&self.op)?;
            }
            if fg != self.fg {
                match fg {
                    Color::Palette(i) => if (i as usize) < self.colors {
                        let s = &self.setaf;
                        tparm(w, s, &mut params!(i), &mut self.vars)?;
                        self.fg = Color::Palette(i);
                    },
                    Color::TrueColor(r, g, b) => if !self.setftc.is_empty() {
                        let s = &self.setftc;
                        tparm(w, s, &mut params!(r, g, b), &mut self.vars)?;
                        self.fg = fg;
                    },
                    _ => (),
                }
            }
            if bg != self.bg {
                match bg {
                    Color::Palette(i) => if (i as usize) < self.colors {
                        let s = &self.setab;
                        tparm(w, s, &mut params!(i), &mut self.vars)?;
                        self.bg = bg;
                    },
                    Color::TrueColor(r, g, b) => if !self.setbtc.is_empty() {
                        let s = &self.setbtc;
                        tparm(w, s, &mut params!(r, g, b), &mut self.vars)?;
                        self.bg = bg;
                    },
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn sgr0(&mut self, w: &mut Write) -> Result<()> {
        self.style = Style::empty();
        self.fg = Color::Default;
        self.bg = Color::Default;
        w.write_all(&self.sgr0)?;
        Ok(())
    }
}

pub(in term) struct Term<'a> {
    styles: Styles,
    cup: Vec<u8>,
    smcup: Vec<u8>,
    rmcup: Vec<u8>,
    civis: Vec<u8>,
    cnorm: Vec<u8>,
    clear: Vec<u8>,
    stdout: io::StdoutLock<'a>,
    init_ios: libc::termios,
    tx: Option<Sender<Box<Event>>>,
    tmode: (TerminalMode, TerminalMode),
    rxvt: bool,
}

impl<'a> Term<'a> {
    pub(in term) fn connect(
        tx: Option<Sender<Box<Event>>>,
        use_tc: UseTruecolor,
        b_b: BoldOrBright,
    ) -> Result<Box<Terminal>> {
        if TERM.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let init_ios = Term::set_ios()?;
        let desc = Desc::current();
        let mut term = Term {
            styles: Styles::new(desc, use_tc, b_b),
            stdout: STDOUT.lock(),
            cup: desc[cap::cup].to_vec(),
            smcup: desc[cap::smcup].to_vec(),
            rmcup: desc[cap::rmcup].to_vec(),
            civis: desc[cap::civis].to_vec(),
            cnorm: desc[cap::cnorm].to_vec(),
            clear: desc[cap::clear].to_vec(),
            tmode: (
                Handle::Stdout.terminal_mode(),
                Handle::Stdin.terminal_mode(),
            ),
            rxvt: is_rxvt(desc),
            init_ios,
            tx,
        };
        term.init()?;
        Ok(Box::new(term))
    }

    fn set_ios() -> Result<libc::termios> {
        unsafe {
            let mut init_ios = ::std::mem::zeroed();
            if 0 != libc::tcgetattr(0, &mut init_ios) {
                return Error::ffi_err("tcgetattr failed");
            }
            let mut ios = init_ios;
            libc::cfmakeraw(&mut ios);
            if 0 != libc::tcsetattr(0, 0, &ios) {
                return Error::ffi_err("tcsetattr failed");
            }
            Ok(init_ios)
        }
    }

    fn init(&mut self) -> Result<()> {
        if !self.is_tty_output() {
            return Ok(());
        }
        self.stdout.write_all(&self.smcup)?;
        Ok(())
    }

    fn uninit(&mut self) -> Result<()> {
        if !::std::thread::panicking() {
            self.stdout.write_all(&self.rmcup)?;
        }
        self.stdout.write_all(&self.cnorm)?;
        Ok(())
    }

    fn start_mouse_input(&mut self) -> Result<()> {
        if !self.is_tty_input() {
            return Ok(());
        }
        self.stdout.write_all(b"\x1b[?1000h")?;
        self.stdout.write_all(b"\x1b[?1003h")?;
        if self.rxvt {
            self.stdout.write_all(b"\x1b[?1015h")?;
        }
        self.stdout.write_all(b"\x1b[?1006h")?;
        Ok(())
    }

    fn end_mouse_input(&mut self) -> Result<()> {
        if !self.is_tty_input() {
            return Ok(());
        }
        self.stdout.write_all(b"\x1b[?1000l")?;
        self.stdout.write_all(b"\x1b[?1003l")?;
        if self.rxvt {
            self.stdout.write_all(b"\x1b[?1015l")?;
        }
        self.stdout.write_all(b"\x1b[?1006l")?;
        Ok(())
    }
}

impl<'a> Terminal for Term<'a> {
    fn is_tty_input(&self) -> bool {
        self.tmode.1 != TerminalMode::Redir
    }

    fn is_tty_output(&self) -> bool {
        self.tmode.0 != TerminalMode::Redir
    }

    fn start_input(&mut self) -> Result<()> {
        self.start_mouse_input()?;
        self.stdout.flush()?;
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
        match get_size(Handle::Stdout) {
            Some(ws) => Ok(ws),
            None => Error::ffi_err("ioctl failed"),
        }
    }

    fn get_style(&self) -> Style {
        self.styles.style
    }

    fn get_fg(&self) -> Color {
        self.styles.fg
    }

    fn get_bg(&self) -> Color {
        self.styles.bg
    }

    fn set_style(&mut self, style: Style, fg: Color, bg: Color) -> Result<()> {
        self.styles.sgr(&mut self.stdout, fg, bg, style)
    }

    fn set_cursor(&mut self, coords: Coords) -> Result<()> {
        if coords.0 > 32_767 || coords.1 > 32_767 {
            panic!("coords out of range");
        }
        tparm(
            &mut self.stdout,
            &self.cup,
            &mut params!(coords.1, coords.0),
            &mut ::tinf::Vars::new(),
        )?;
        Ok(())
    }

    fn cursor_visible(&mut self, visible: bool) -> Result<()> {
        // TODO: error if capability not present?
        let cmd = if visible {
            &self.cnorm
        } else {
            &self.civis
        };
        self.stdout.write_all(cmd)?;
        Ok(())
    }

    fn write(&mut self, text: &str) -> Result<()> {
        write!(self.stdout, "{}", text)?;
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.stdout.write_all(&self.clear)?;
        Ok(())
    }

    fn flush_output(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str) {
        let mut stderr = io::stderr();
        let _ = write!(stderr, "{}", text);
        let _ = writeln!(stderr, "\r");
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        let _ = self.end_mouse_input();
        let _ = self.uninit();
        let _ = self.styles.sgr0(&mut self.stdout);
        let _ = self.stdout.flush();
        unsafe {
            libc::tcsetattr(0, 0, &self.init_ios);
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::{BoldOrBright, Color, Style, Styles, UseTruecolor};
    use tinf::Desc;

    fn sgr_desc() -> Desc {
        use tinf::cap::*;
        desc![
            "sgr", "sgr",
            sgr0 => "s",
            bold => "B",
            sitm => "I",
            ritm => "i",
            smul => "U",
            rmul => "u",
            op => "c",
            colors => 256,
            setaf => "%p1%d",
            setab => "(%p1%d)",
            UserDef::named("setf24") => "%p1%d%p2%d%p3%d",
            UserDef::named("setb24") => "(%p1%d%p2%d%p3%d)",
        ]
    }

    #[test]
    fn sgr_style() {
        let mut styles =
            Styles::new(&sgr_desc(), UseTruecolor::Auto, BoldOrBright::Bold);

        let mut w: Vec<u8> = Vec::new();
        styles.sgr(
            &mut w,
            Color::Default,
            Color::Default,
            Style::BOLD | Style::ITALIC | Style::UNDERLINE,
        );
        assert_eq!(b"BIU".to_vec(), w);

        w.clear();
        styles.sgr(&mut w, Color::Default, Color::Default, Style::BOLD);
        assert_eq!(b"sB".to_vec(), w);

        w.clear();
        styles.sgr(&mut w, Color::Default, Color::Default, Style::BOLD);
        assert_eq!(b"".to_vec(), w);

        w.clear();
        styles.sgr(
            &mut w,
            Color::Default,
            Color::Default,
            Style::ITALIC | Style::UNDERLINE,
        );
        assert_eq!(b"sIU".to_vec(), w);
    }

    #[test]
    fn sgr_color() {
        let mut styles =
            Styles::new(&sgr_desc(), UseTruecolor::Auto, BoldOrBright::Bold);

        let mut w: Vec<u8> = Vec::new();
        styles.sgr(
            &mut w,
            Color::Palette(1),
            Color::Palette(2),
            Style::empty(),
        );
        assert_eq!(b"1(2)".to_vec(), w);

        w.clear();
        styles.sgr(
            &mut w,
            Color::Palette(1),
            Color::Palette(2),
            Style::empty(),
        );
        assert_eq!(b"".to_vec(), w);

        w.clear();
        styles.sgr(&mut w, Color::Default, Color::Palette(2), Style::empty());
        assert_eq!(b"c(2)".to_vec(), w);
    }

    fn bright_desc() -> Desc {
        use tinf::cap::*;
        desc![
            "bright", "bright",
            sgr0 => "s",
            bold => "B",
            op => "c",
            colors => 8,
            setaf => "%p1%d",
            setab => "(%p1%d)",
        ]
    }

    #[test]
    fn bright_colors() {
        let mut styles = Styles::new(
            &bright_desc(),
            UseTruecolor::Auto,
            BoldOrBright::Bright,
        );
        let mut w: Vec<u8> = Vec::new();
        // from default to regular
        styles.sgr(&mut w, Color::Palette(1), Color::Default, Style::empty());
        assert_eq!(b"1".to_vec(), w);
        // from regular to bold
        w.clear();
        styles.sgr(&mut w, Color::Palette(9), Color::Default, Style::empty());
        assert_eq!(b"B".to_vec(), w);
        // from bold to regular
        w.clear();
        styles.sgr(&mut w, Color::Palette(2), Color::Default, Style::empty());
        assert_eq!(b"s2".to_vec(), w);
        // from regular to default
        w.clear();
        styles.sgr(&mut w, Color::Default, Color::Default, Style::empty());
        assert_eq!(b"c".to_vec(), w);
        // from default to bold
        w.clear();
        styles.sgr(&mut w, Color::Palette(10), Color::Default, Style::empty());
        assert_eq!(b"B2".to_vec(), w);
        // from bold to default
        w.clear();
        styles.sgr(&mut w, Color::Default, Color::Default, Style::empty());
        assert_eq!(b"s".to_vec(), w);
    }
}
