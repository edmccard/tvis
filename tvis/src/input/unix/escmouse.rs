use input::{ButtonMotion, InputEvent, Key, Mods, MouseButton, WheelMotion};
use super::EscNode;
use super::esckey::Parser as KeyParser;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum Type {
    Uninit,
    Normal,
    SGR,
    Urxvt,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum ParseResult {
    Found(InputEvent),
    Maybe,
    No,
}

#[derive(Copy, Clone, Debug)]
enum State {
    B,
    X,
    Y,
}

pub(super) const MOUSE_MAGIC: u8 = 10;
pub(super) const SGR_MAGIC: u8 = 20;

pub(super) struct Parser {
    ty: Type,
    state: State,
    param: u16,
    plen: usize,
    evt: InputEvent,
    x: u16,
}

impl Parser {
    pub(super) fn new() -> Parser {
        Parser {
            ty: Type::Uninit,
            state: State::B,
            param: 0,
            plen: 0,
            evt: InputEvent::Break,
            x: 0,
        }
    }

    pub(super) fn add_mouse_keys(nodes: &mut Vec<EscNode>) {
        KeyParser::add_key_bytes(
            nodes,
            b"[M",
            (Key::Char([0, 0, 0, 0], MOUSE_MAGIC), Mods::empty()),
        );
        KeyParser::add_key_bytes(
            nodes,
            b"[<",
            (Key::Char([0, 0, 0, 0], SGR_MAGIC), Mods::empty()),
        );
    }

    pub(super) fn reset(&mut self, ty: Type) {
        self.ty = ty;
        self.next_state(State::B);
    }

    fn next_state(&mut self, state: State) {
        self.state = state;
        self.param = 0;
        self.plen = 0;
    }

    pub(super) fn parse(&mut self, byte: u8) -> ParseResult {
        match self.ty {
            Type::Normal => self.parse_normal(byte),
            Type::SGR | Type::Urxvt => self.parse_extended(byte),
            _ => unreachable!(),
        }
    }

    fn parse_normal(&mut self, byte: u8) -> ParseResult {
        use self::ParseResult::*;

        match self.state {
            State::B => {
                if byte < 32 {
                    return No;
                }
                if !self.x10_button(byte) {
                    return No;
                }
                self.state = State::X;
            }
            State::X => {
                if byte < 33 {
                    return No;
                }
                self.x = u16::from(byte - 33);
                self.state = State::Y;
            }
            State::Y => {
                if byte < 33 {
                    return No;
                }
                self.set_coords(u16::from(byte - 33));
                return self.mouse_event();
            }
        }
        Maybe
    }

    fn parse_extended(&mut self, byte: u8) -> ParseResult {
        let sgr = self.ty == Type::SGR;
        match self.state {
            State::B => if !self.parse_ext_b(byte, sgr) {
                return ParseResult::No;
            },
            State::X => if !self.parse_ext_x(byte) {
                return ParseResult::No;
            },
            State::Y => {
                return self.parse_ext_y(byte, sgr);
            }
        }
        ParseResult::Maybe
    }

    fn parse_ext_b(&mut self, byte: u8, sgr: bool) -> bool {
        if byte == b';' {
            if self.plen == 0 {
                return false;
            }
            let param = self.param as u8;
            if !self.x10_button(param + if sgr { 32 } else { 0 }) {
                return false;
            }
            self.next_state(State::X);
        } else if byte >= b'0' && byte <= b'9' {
            return self.update_param(byte, 255);
        } else {
            return false;
        }
        true
    }

    fn parse_ext_x(&mut self, byte: u8) -> bool {
        if byte == b';' {
            if self.plen == 0 {
                return false;
            }
            self.x = self.param;
            self.next_state(State::Y);
        } else if byte >= b'0' && byte <= b'9' {
            if !self.update_param(byte, 9999) {
                return false;
            }
        } else {
            return false;
        }
        true
    }

    fn parse_ext_y(&mut self, byte: u8, sgr: bool) -> ParseResult {
        use self::InputEvent::*;
        use self::ButtonMotion::*;

        if byte == b'M' || (sgr && byte == b'm') {
            if self.plen == 0 {
                return ParseResult::No;
            }
            let param = self.param;
            self.set_coords(param);
            if sgr {
                match (byte, self.evt) {
                    (b'm', Mouse(Press, b, ms, cs)) => {
                        self.evt = Mouse(ButtonMotion::Release, b, ms, cs);
                    }
                    (b'M', Mouse(Release, _, ms, cs)) => {
                        self.evt = MouseMove(ms, cs);
                    }
                    _ => (),
                }
            }
            self.mouse_event()
        } else if byte >= b'0' && byte <= b'9' {
            if !self.update_param(byte, 9999) {
                return ParseResult::No;
            }
            ParseResult::Maybe
        } else {
            ParseResult::No
        }
    }

    fn update_param(&mut self, byte: u8, max: u16) -> bool {
        self.plen += 1;
        self.param *= 10;
        self.param += u16::from(byte - b'0');
        self.param <= max
    }

    fn x10_button(&mut self, byte: u8) -> bool {
        use self::InputEvent::*;
        use self::MouseButton::*;
        use self::ButtonMotion::*;

        let mods = Mods::from_bits((byte >> 2) & 0b111).unwrap();
        let byte = byte & 0b1110_0011;
        if byte > 95 {
            if byte == 96 {
                self.evt = MouseWheel(WheelMotion::Up, mods, (0, 0));
            } else if byte == 97 {
                self.evt = MouseWheel(WheelMotion::Down, mods, (0, 0));
            } else if self.ty == Type::Urxvt && (byte == 128 || byte == 129) {
                // Urxvt reports mouse movements after a wheel event
                // as though the wheel motion was still "pressed",
                // until another mouse button is pressed.
                self.evt = MouseMove(mods, (0, 0));
            } else {
                return false;
            }
        } else if byte > 63 {
            self.evt = MouseMove(mods, (0, 0));
        } else {
            self.evt = match byte & 0b11 {
                0 => Mouse(Press, Left, mods, (0, 0)),
                1 => Mouse(Press, Middle, mods, (0, 0)),
                2 => Mouse(Press, Right, mods, (0, 0)),
                3 => Mouse(Release, Unknown, mods, (0, 0)),
                _ => unreachable!(),
            }
        }
        true
    }

    fn set_coords(&mut self, y: u16) {
        use self::InputEvent::*;

        let coords = (self.x - 1, y - 1);
        let evt = match self.evt {
            Mouse(m, b, ms, _) => Mouse(m, b, ms, coords),
            MouseMove(ms, _) => MouseMove(ms, coords),
            MouseWheel(wm, ms, _) => MouseWheel(wm, ms, coords),
            _ => return,
        };
        self.evt = evt;
    }

    fn mouse_event(&mut self) -> ParseResult {
        let ty = self.ty;
        self.reset(ty);
        ParseResult::Found(self.evt)
    }
}
