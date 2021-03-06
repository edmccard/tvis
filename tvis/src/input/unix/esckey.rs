use tinf::{cap, Desc};
use input::{Key, Mods};
use super::escmouse::{MOUSE_MAGIC, SGR_MAGIC};
use super::escmouse::Type as MouseType;
use super::escmouse::Parser as MouseParser;

type KeyPress = (Key, Mods);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub(super) struct EscNode {
    sibling: i16,
    children: i16,
    key: KeyPress,
    byte: u8,
    unused: u8,
}

impl Default for EscNode {
    fn default() -> EscNode {
        EscNode {
            sibling: -1,
            children: -1,
            key: (Key::empty(), Default::default()),
            byte: 0,
            unused: 0,
        }
    }
}

enum State {
    Plain(u8),
    ModDigit1,
    ModDigit2,
}

pub(super) enum ParseResult {
    Found(KeyPress),
    Mouse(MouseType),
    Maybe,
    No,
}

pub(super) struct Parser {
    nodes: Vec<EscNode>,
    state: State,
    idx: i16,
    xterm_mods: bool,
}

impl Parser {
    pub(super) fn new(desc: &Desc) -> Parser {
        let mut nodes: Vec<EscNode> = vec![Default::default()];
        MouseParser::add_mouse_keys(&mut nodes);
        // For the keys in APPKEYS, xterm-alikes deviate from their
        // normal behavior for modifier keys; instead of sending, for
        // example ';2' for shift just before the last byte of the
        // normal sequence, they send '1;2' (Also, since we do not
        // activate application mode, we add entries that swap SS3 for
        // CSI).
        let xterm_mods = Parser::xterm_mods(desc);
        for &(c, k) in APPKEYS.iter().filter(|x| !desc[x.0].is_empty()) {
            Parser::add_key_bytes(&mut nodes, &desc[c][1..], k);
            if xterm_mods && desc[c][1] == b'O' {
                let mut csi = vec![b'['];
                csi.extend(&desc[c][2..]);
                Parser::add_key_bytes(&mut nodes, &csi, k);
                csi = vec![b'[', b'1'];
                csi.extend(&desc[c][2..]);
                Parser::add_key_bytes(&mut nodes, &csi, k);
            }
        }
        for &(c, k) in KEYS.iter().filter(|x| !desc[x.0].is_empty()) {
            Parser::add_key_bytes(&mut nodes, &desc[c][1..], k);
        }
        Parser {
            nodes,
            state: State::Plain(0),
            idx: 1,
            xterm_mods,
        }
    }

    pub(super) fn add_key_bytes(
        nodes: &mut Vec<EscNode>,
        bytes: &[u8],
        key: KeyPress,
    ) {
        let mut idx = 0usize;
        let mut pos = 0usize;
        while pos < bytes.len() {
            while nodes[idx].byte != bytes[pos] && nodes[idx].sibling >= 0 {
                idx = nodes[idx].sibling as usize;
            }
            if nodes[idx].byte == bytes[pos] {
                pos += 1;
                if pos == bytes.len() {
                    nodes[idx].key = key;
                } else {
                    if nodes[idx].children < 0 {
                        nodes[idx].children = nodes.len() as i16;
                        nodes.push(EscNode {
                            byte: bytes[pos],
                            ..Default::default()
                        });
                    }
                    idx = nodes[idx].children as usize;
                }
            } else {
                // Here, nodes[idx].sibling is < 0.
                nodes[idx].sibling = nodes.len() as i16;
                nodes.push(EscNode {
                    byte: bytes[pos],
                    ..Default::default()
                });
                idx = nodes[idx].sibling as usize;
                // Keep same pos for next iteration
            }
        }
    }

    // If any shifted key ends with ";2<char>", assume the standard
    // scheme for modifier keys.
    fn xterm_mods(desc: &Desc) -> bool {
        cap::String::iter()
            .filter(Parser::is_shifted_key)
            .map(|c| &desc[c])
            .any(|s| {
                s.len() > 3 && s[s.len() - 2] == b'2' && s[s.len() - 3] == b';'
            })
    }

    // Capabilities named like "kXXX" are shifted keys.
    fn is_shifted_key(scap: &cap::String) -> bool {
        let mut bytes = scap.short_name().bytes();
        bytes.next() == Some(b'k') && bytes.all(|c| c >= b'A' && c <= b'Z')
    }

    pub(super) fn reset(&mut self) {
        self.idx = 1;
        self.state = State::Plain(0);
    }

    pub(super) fn search(&mut self, byte: u8) -> ParseResult {
        use self::State::*;
        use self::ParseResult::*;
        use super::escmouse::Type::*;

        match self.state {
            Plain(m) => {
                match self.check(byte) {
                    Found((k, m1)) => {
                        self.reset();
                        match k {
                            Key::Char(_, _, MOUSE_MAGIC) => Mouse(Normal),
                            Key::Char(_, _, SGR_MAGIC) => Mouse(SGR),
                            _ => {
                                // ignore "meta" bit
                                let m1 = m1.bits | (m & 7);
                                let m1 = Mods::from_bits(m1).unwrap();
                                Found((k, m1))
                            }
                        }
                    }
                    Maybe => Maybe,
                    No => {
                        if self.xterm_mods && byte == b';' && m == 0 {
                            self.state = ModDigit1;
                            return Maybe;
                        }
                        self.reset();
                        No
                    }
                    _ => unreachable!(),
                }
            }
            ModDigit1 => {
                if byte >= b'2' && byte <= b'9' {
                    self.state = Plain(byte - b'2' + 1);
                    return Maybe;
                }
                if byte == b'1' {
                    self.state = ModDigit2;
                    return Maybe;
                }
                self.reset();
                No
            }
            ModDigit2 => {
                if byte >= b'0' && byte <= b'6' {
                    self.state = Plain(byte - b'0' + 1);
                    return Maybe;
                }
                self.reset();
                No
            }
        }
    }

    fn check(&mut self, byte: u8) -> ParseResult {
        use self::ParseResult::*;

        let mut idx = self.idx;
        while idx >= 0 {
            let node = self.nodes[idx as usize];
            if node.byte == byte {
                self.idx = node.children;
                return match node.key.0 {
                    Key::Char(_, _, 0) => Maybe,
                    _ => Found(node.key),
                };
            }
            idx = node.sibling;
        }
        No
    }
}

lazy_static! {
    static ref APPKEYS: Vec<(cap::String, KeyPress)> = {
        vec![
            (cap::kcuu1, (Key::Up, Mods::empty())),
            (cap::kcud1, (Key::Down, Mods::empty())),
            (cap::kcub1, (Key::Left, Mods::empty())),
            (cap::kcuf1, (Key::Right, Mods::empty())),
            (cap::khome, (Key::Home, Mods::empty())),
            (cap::kend, (Key::End, Mods::empty())),
            (cap::kf1, (Key::F1, Mods::empty())),
            (cap::kf2, (Key::F2, Mods::empty())),
            (cap::kf3, (Key::F3, Mods::empty())),
            (cap::kf4, (Key::F4, Mods::empty())),
        ]
    };

    static ref KEYS: Vec<(cap::String, KeyPress)> = {
        vec![
            (cap::kich1, (Key::Ins, Mods::empty())),
            (cap::kdch1, (Key::Del, Mods::empty())),
            (cap::kpp, (Key::PgUp, Mods::empty())),
            (cap::knp, (Key::PgDn, Mods::empty())),
            (cap::kf5, (Key::F5, Mods::empty())),
            (cap::kf6, (Key::F6, Mods::empty())),
            (cap::kf7, (Key::F7, Mods::empty())),
            (cap::kf8, (Key::F8, Mods::empty())),
            (cap::kf9, (Key::F9, Mods::empty())),
            (cap::kf10, (Key::F10, Mods::empty())),
            (cap::kf11, (Key::F11, Mods::empty())),
            (cap::kf12, (Key::F12, Mods::empty())),
        ]
    };
}

#[cfg(test)]
mod test {
    use super::{EscNode, Key, KeyPress, Mods, ParseResult, Parser, State};

    fn node(s: i16, c: i16, b: u8) -> EscNode {
        EscNode {
            sibling: s,
            children: c,
            key: (Key::empty(), Default::default()),
            byte: b,
            unused: 0,
        }
    }

    fn node_final(b: u8, k: KeyPress) -> EscNode {
        EscNode {
            sibling: -1,
            children: -1,
            key: k,
            byte: b,
            unused: 0,
        }
    }

    fn empty_keys(xterm_mods: bool) -> Parser {
        Parser {
            nodes: vec![Default::default()],
            state: State::Plain(0),
            idx: 1,
            xterm_mods,
        }
    }

    fn search(kp: &mut Parser, bytes: &[u8]) -> Option<KeyPress> {
        use self::ParseResult::*;
        for byte in bytes {
            match kp.search(*byte) {
                Found(k) => return Some(k),
                Maybe => (),
                _ => return None,
            }
        }
        None
    }

    #[test]
    fn add_one() {
        let up_arr = (b"[A", (Key::Up, Mods::empty()));
        let down_arr = (b"[B", (Key::Down, Mods::empty()));
        let mut built = empty_keys(false);
        Parser::add_key_bytes(&mut built.nodes, up_arr.0, up_arr.1);
        assert_eq!(search(&mut built, up_arr.0), Some(up_arr.1));
        assert_eq!(search(&mut built, b"[A123"), Some(up_arr.1));
        assert_eq!(search(&mut built, down_arr.0), None);

        let expected = vec![
            Default::default(),
            node(-1, 2, b'['),
            node_final(b'A', up_arr.1),
        ];
        assert_eq!(built.nodes[1..], expected[1..]);
    }

    #[test]
    fn add_overlap() {
        let f5 = (b"[15~", (Key::F5, Mods::empty()));
        let f6 = (b"[17~", (Key::F6, Mods::empty()));
        let f7 = (b"[18~", (Key::F7, Mods::empty()));
        let mut built = empty_keys(false);
        Parser::add_key_bytes(&mut built.nodes, f5.0, f5.1);
        Parser::add_key_bytes(&mut built.nodes, f6.0, f6.1);
        Parser::add_key_bytes(&mut built.nodes, f7.0, f7.1);
        assert_eq!(search(&mut built, f7.0), Some(f7.1));
        assert_eq!(search(&mut built, f6.0), Some(f6.1));
        assert_eq!(search(&mut built, f5.0), Some(f5.1));

        let expected = vec![
            Default::default(),
            node(-1, 2, b'['),
            node(-1, 3, b'1'),
            node(5, 4, b'5'),
            node_final(b'~', f5.1),
            node(7, 6, b'7'),
            node_final(b'~', f6.1),
            node(-1, 8, b'8'),
            node_final(b'~', f7.1),
        ];
        assert_eq!(built.nodes[1..], expected[1..]);
    }

    #[test]
    fn add_disjoint() {
        let f1 = (b"OP", (Key::F1, Mods::empty()));
        let f5 = (b"[15~", (Key::F5, Mods::empty()));
        let mut built = empty_keys(false);
        Parser::add_key_bytes(&mut built.nodes, f1.0, f1.1);
        Parser::add_key_bytes(&mut built.nodes, f5.0, f5.1);
        assert_eq!(search(&mut built, f1.0), Some(f1.1));
        assert_eq!(search(&mut built, f5.0), Some(f5.1));

        let expected = vec![
            Default::default(),
            node(3, 2, b'O'),
            node_final(b'P', f1.1),
            node(-1, 4, b'['),
            node(-1, 5, b'1'),
            node(-1, 6, b'5'),
            node_final(b'~', f5.1),
        ];
        assert_eq!(built.nodes[1..], expected[1..]);
    }

    #[test]
    fn mods() {
        use input::Mods;
        let f5 = (b"[15~", (Key::F5, Mods::empty()));
        let sf5 = (b"[15;2~", (Key::F5, Mods::SHIFT));
        let acf5 = (b"[15;7~", (Key::F5, Mods::CTRL | Mods::ALT));
        let mf5 = (b"[15;9~", (Key::F5, Mods::empty()));
        let macf5 = (b"[15;15~", (Key::F5, Mods::CTRL | Mods::ALT));
        let mut with_mods = empty_keys(true);
        Parser::add_key_bytes(&mut with_mods.nodes, f5.0, f5.1);
        assert_eq!(search(&mut with_mods, f5.0), Some(f5.1));
        assert_eq!(search(&mut with_mods, sf5.0), Some(sf5.1));
        assert_eq!(search(&mut with_mods, acf5.0), Some(acf5.1));
        assert_eq!(search(&mut with_mods, mf5.0), Some(f5.1));
        assert_eq!(search(&mut with_mods, macf5.0), Some(acf5.1));
    }
}
