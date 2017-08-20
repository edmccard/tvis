#![cfg(not(windows))]

use std::{mem, ptr, thread};
use std::sync::mpsc::Sender;
use std::time::Instant;

use libc::{self, c_int};
use tinf::{Desc, cap};

use input::{InputEvent, Key, Mod};
use {Error, Event, Result};

mod esckey;

static mut SIGNAL_FDS: Option<(c_int, c_int)> = None;

// Starts the event loop thread and initializes signal handling.
pub(crate) fn start_threads(tx: Sender<Box<Event>>) -> Result<()> {
    init_pipe()?;
    thread::spawn(|| unsafe {
        // Prevent signals from being delivered to the event loop thread.
        let mut set: libc::sigset_t = mem::uninitialized();
        libc::sigfillset(&mut set);
        libc::pthread_sigmask(libc::SIG_BLOCK, &set, ptr::null_mut());
        raw_event_loop(tx);
    });
    init_signals();
    Ok(())
}

// Set up the "self-pipe" used for the signal handler.
fn init_pipe() -> Result<()> {
    unsafe {
        let mut fds: [c_int; 2] = mem::uninitialized();
        if -1 == libc::pipe(fds.as_mut_ptr()) {
            return Error::ffi_err("pipe failed");
        }
        let flags = libc::fcntl(fds[1], libc::F_GETFL);
        libc::fcntl(fds[1], libc::F_SETFL, flags | libc::O_NONBLOCK);
        SIGNAL_FDS = Some((fds[0], fds[1]));
    }
    Ok(())
}

// Define and register the signal handler for WINCH, TERM, INT, QUIT.
fn init_signals() {
    // The signal handler, which uses the "self-pipe" trick.
    extern "C" fn handler(signum: c_int) {
        let val = match signum {
            libc::SIGWINCH => 1,
            libc::SIGTERM => 2,
            libc::SIGINT => 3,
            libc::SIGQUIT => 4,
            _ => return,
        };
        unsafe {
            let signal_fds = SIGNAL_FDS.unwrap();
            let valptr = &val as *const _ as *const libc::c_void;
            libc::write(signal_fds.1, valptr, 1);
        }
    }

    unsafe {
        let mut sa: libc::sigaction = mem::uninitialized();
        let mut set: libc::sigset_t = mem::uninitialized();
        libc::sigemptyset(&mut set);
        sa.sa_sigaction = mem::transmute(handler as extern "C" fn(c_int));
        sa.sa_flags = libc::SA_RESTART;
        sa.sa_mask = set;
        libc::sigaction(libc::SIGWINCH, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGTERM, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGINT, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGQUIT, &sa, ptr::null_mut());
    }
}

const WAIT_MICROS: libc::suseconds_t = 25_000;

// Convert input from stdin (and the signal pipe) into InputEvents.
unsafe fn raw_event_loop(tx: Sender<Box<Event>>) {
    // TODO: indicate errors?
    let mut reader = Reader::new(Desc::current(), tx);
    let signal_fd = SIGNAL_FDS.unwrap().0;
    let mut stdin_buf = [0u8; 1024];
    let mut override_timeout = false;
    let mut timeout = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    loop {
        let mut read_fds: libc::fd_set = mem::uninitialized();
        let nfds: c_int = signal_fd + 1;
        libc::FD_ZERO(&mut read_fds);
        libc::FD_SET(0, &mut read_fds);
        libc::FD_SET(signal_fd, &mut read_fds);
        let timeout_ptr = if timeout.tv_usec == 0 {
            ptr::null_mut()
        } else {
            override_timeout = false;
            &mut timeout
        };
        let tstart = Instant::now();
        let select = libc::select(
            nfds + 1,
            &mut read_fds,
            ptr::null_mut(),
            ptr::null_mut(),
            timeout_ptr,
        );
        match select {
            -1 => return,
            0 => {
                timeout.tv_usec = 0;
                if !override_timeout {
                    if reader.reset().is_err() {
                        return;
                    }
                }
            }
            _ => (),
        }
        if libc::FD_ISSET(0, &mut read_fds) {
            let bufptr = stdin_buf.as_mut_ptr() as *mut libc::c_void;
            let len = libc::read(0, bufptr, 1024);
            if len < 1 {
                return;
            }
            match reader.parse_stdin(&stdin_buf[0..len as usize]) {
                Err(_) => return,
                Ok(ParseOk::Continue) => {
                    override_timeout = true;
                    timeout.tv_usec = 0;
                }
                Ok(ParseOk::Wait) => {
                    timeout.tv_usec = WAIT_MICROS;
                }
            }
        }
        if libc::FD_ISSET(signal_fd, &mut read_fds) {
            let mut buf = 0u8;
            let bufptr = &mut buf as *mut _ as *mut libc::c_void;
            let len = libc::read(signal_fd, bufptr, 1);
            if len < 1 || !reader.parse_signal(buf).is_ok() {
                return;
            }
            // Just in case we're here handling a signal in the middle
            // of waiting for partial input to be complete (and we're
            // on an OS where select doesn't decrement the timeout).
            if timeout.tv_usec != 0 {
                let elapsed = (tstart.elapsed().subsec_nanos() / 1000) as i64;
                if elapsed < WAIT_MICROS {
                    timeout.tv_usec = WAIT_MICROS - elapsed;
                }
            }
        }
    }
}

#[derive(Eq, PartialEq)]
enum ParseOk {
    Continue,
    Wait,
}

type ParseResult = ::std::result::Result<ParseOk, ()>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum ParseState {
    Init,
    KeySeq1,
    KeySeq2,
}

struct Reader {
    seq: esckey::KeyParser,
    utf8: Utf8Parser,
    tx: Sender<Box<Event>>,
    hold_keys: Vec<Utf8Val>,
    state: ParseState,
    bs: u8,
    cbs: u8,
}

impl Reader {
    fn new(desc: &Desc, tx: Sender<Box<Event>>) -> Reader {
        let (bs, cbs) = if desc[cap::kbs] == [0x7f] {
            (0x7f, 0x08)
        } else {
            (0x08, 0x7f)
        };
        Reader {
            seq: esckey::KeyParser::new(desc),
            utf8: Default::default(),
            tx,
            hold_keys: Vec::with_capacity(25),
            state: ParseState::Init,
            bs,
            cbs,
        }
    }

    fn parse_signal(&self, data: u8) -> ParseResult {
        let event = match data {
            1 => InputEvent::Repaint,
            2 => return Err(()), // break out of loop for SIGTERM
            3 => InputEvent::Interrupt,
            4 => InputEvent::Break,
            _ => unreachable!(),
        };
        self.send(event)
    }

    fn send(&self, event: InputEvent) -> ParseResult {
        if self.tx.send(Box::new(event)).is_ok() {
            Ok(ParseOk::Continue)
        } else {
            Err(())
        }
    }

    fn reset(&mut self) -> ParseResult {
        use self::ParseState::*;
        match self.state {
            KeySeq1 => {
                if self.hold_keys.is_empty() {
                    let event = InputEvent::Key(Key::Esc, Mod::none());
                    self.send(event)?;
                } else {
                    let (key, mods) = self.xlate_cp(self.hold_keys[0]);
                    let event = InputEvent::Key(key, mods.add_alt());
                    self.send(event)?;
                    for cp in &self.hold_keys[1..] {
                        let (key, mods) = self.xlate_cp(*cp);
                        let event = InputEvent::Key(key, mods);
                        self.send(event)?;
                    }
                }
            }
            KeySeq2 => {
                let event = InputEvent::Key(Key::Esc, Mod::alt());
                self.send(event)?;
                for cp in &self.hold_keys {
                    let (key, mods) = self.xlate_cp(*cp);
                    let event = InputEvent::Key(key, mods);
                    self.send(event)?;
                }
            }
            _ => (),
        }
        self.hold_keys.clear();
        self.state = Init;
        Ok(ParseOk::Continue)
    }

    fn parse_stdin(&mut self, data: &[u8]) -> ParseResult {
        println!("DATA {:?}\r", data);
        // data does not start empty (len check in raw_event_loop).
        let mut pos = 0usize;
        while pos < data.len() {
            let read = self.utf8.read(&data[pos..]);
            match read {
                Utf8Result::Wait => return Ok(ParseOk::Wait),
                Utf8Result::Err(cp, _) => {
                    self.reset()?;
                    let event =
                        InputEvent::Key(Key::Err(cp.0, cp.1), Mod::none());
                    return self.send(event);
                }
                Utf8Result::Ok(cp, len) => {
                    pos += len as usize;
                    self.parse_cp(cp)?;
                    if self.state != ParseState::Init && pos == data.len() {
                        return Ok(ParseOk::Wait);
                    }
                }
            }
        }
        Ok(ParseOk::Continue)
    }

    // Parse one codepoint.
    fn parse_cp(&mut self, cp: Utf8Val) -> ParseResult {
        use self::ParseState::*;

        match self.state {
            Init => {
                let (key, mods) = self.xlate_cp(cp);
                if key == Key::Esc {
                    self.reset()?;
                    self.state = KeySeq1;
                    Ok(ParseOk::Continue)
                } else {
                    self.send(InputEvent::Key(key, mods))
                }
            }
            KeySeq1 => {
                if cp.1 > 1 {
                    self.hold_keys.push(cp);
                    return self.reset();
                }
                if cp.0[0] == 27 {
                    if self.hold_keys.is_empty() {
                        self.state = KeySeq2
                    } else {
                        self.reset()?;
                        self.state = KeySeq1
                    }
                    return Ok(ParseOk::Continue);
                }
                self.search_key_seq(cp)
            }
            KeySeq2 => {
                if cp.1 > 1 {
                    self.hold_keys.push(cp);
                    return self.reset();
                }
                if cp.0[0] == 27 {
                    self.reset()?;
                    self.state = KeySeq1;
                    return Ok(ParseOk::Continue);
                }
                self.search_key_seq(cp)
            }
        }
    }

    fn search_key_seq(&mut self, cp: Utf8Val) -> ParseResult {
        use self::esckey::KeyResult::*;

        self.hold_keys.push(cp);
        match self.seq.search(cp.0[0]) {
            No => self.reset(),
            Maybe => Ok(ParseOk::Continue),
            Found((k, m)) => {
                let event = match self.state {
                    ParseState::KeySeq2 => InputEvent::Key(k, m.add_alt()),
                    _ => InputEvent::Key(k, m),
                };
                self.hold_keys.clear();
                self.state = ParseState::Init;
                self.send(event)
            }
        }
    }

    // Translate ascii special chars (C-<char>, Esc, Tab, Enter, BS)
    // and pass through the rest as Key::Char
    fn xlate_cp(&self, cp: Utf8Val) -> (Key, Mod) {
        match cp.0[0] {
            0 => (Key::ascii(32), Mod::ctrl()),
            9 => (Key::Tab, Mod::none()),
            13 => (Key::Enter, Mod::none()),
            27 => (Key::Esc, Mod::none()),
            b if b == self.bs => (Key::BS, Mod::none()),
            b if b == self.cbs => (Key::BS, Mod::ctrl()),
            b if b < b' ' => (Key::ascii(b + 64), Mod::ctrl()),
            _ => (Key::Char(cp.0, cp.1), Mod::none()),
        }
    }
}

type Utf8Val = ([u8; 4], u8);

enum Utf8Result {
    Ok(Utf8Val, u8),
    Err(Utf8Val, u8),
    Wait,
}

#[derive(Default)]
struct Utf8Parser {
    bytes_read: usize,
    clen: usize,
    bytes: [u8; 4],
    b2_min: u8,
    b2_max: u8,
}

impl Utf8Parser {
    #[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
    fn set_char_info(&mut self, b: u8) {
        let (clen, b2_min, b2_max) = if b < 0x80 {
            (1, 0, 0)
        } else if b < 0xc2 {
            (0, 0, 0)
        } else if b < 0xe0 {
            (2, 0x80, 0xbf)
        } else if b == 0xe0 {
            (3, 0xa0, 0xbf)
        } else if b < 0xed {
            (3, 0x80, 0xbf)
        } else if b == 0xed {
            (3, 0x80, 0x9f)
        } else if b < 0xf0 {
            (3, 0x80, 0xbf)
        } else if b == 0xf0 {
            (4, 0x90, 0xbf)
        } else if b < 0xf4 {
            (4, 0x80, 0xbf)
        } else if b == 0xf4 {
            (4, 0x80, 0x8f)
        } else {
            (0, 0, 0)
        };
        self.clen = clen;
        self.b2_min = b2_min;
        self.b2_max = b2_max;
    }

    fn ok(&mut self, pos: usize) -> Utf8Result {
        let ret =
            Utf8Result::Ok((self.bytes, self.bytes_read as u8), pos as u8);
        self.bytes_read = 0;
        ret
    }

    fn err(&mut self, pos: usize) -> Utf8Result {
        let ret =
            Utf8Result::Err((self.bytes, self.bytes_read as u8), pos as u8);
        self.bytes_read = 0;
        ret
    }

    fn wait(&self) -> Utf8Result {
        Utf8Result::Wait
    }

    fn read(&mut self, data: &[u8]) -> Utf8Result {
        assert!(!data.is_empty());
        let mut pos = 0usize;
        if self.bytes_read < 1 {
            self.bytes[0] = data[pos];
            self.set_char_info(data[pos]);
            pos += 1;
            self.bytes_read = 1;
            if self.clen == 0 {
                return self.err(pos);
            }
            if self.clen == 1 {
                return self.ok(pos);
            }
        }
        if self.bytes_read < 2 {
            if pos == data.len() {
                return self.wait();
            }
            self.bytes[1] = data[pos];
            if self.bytes[1] < self.b2_min || self.bytes[1] > self.b2_max {
                return self.err(pos);
            }
            pos += 1;
            self.bytes_read = 2;
            if self.clen == 2 {
                return self.ok(pos);
            }
        }
        if self.bytes_read < 3 {
            if pos == data.len() {
                return self.wait();
            }
            self.bytes[2] = data[pos];
            if self.bytes[2] < 0x80 || self.bytes[2] > 0xbf {
                return self.err(pos);
            }
            pos += 1;
            self.bytes_read = 3;
            if self.clen == 3 {
                return self.ok(pos);
            }
        }
        if self.bytes_read < 4 {
            if pos == data.len() {
                return self.wait();
            }
            self.bytes[3] = data[pos];
            if self.bytes[3] < 0x80 || self.bytes[3] > 0xbf {
                return self.err(pos);
            }
            pos += 1;
            self.bytes_read = 4;
            return self.ok(pos);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use tinf::Desc;

    fn desc() -> Desc {
        use tinf::cap::*;
        desc![
            kf5 => b"\x1b[15~",
        ]
    }

    #[test]
    fn esc() {
    }
}
