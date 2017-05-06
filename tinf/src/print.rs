use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

/// A parameter for [`tparm`](fn.tparm.html).
///
/// The `params!` macro](../macros.params.html) defines a convenient
/// literal syntax for groups of parameters.
#[derive(Clone, Debug)]
pub enum Param {
    Absent,
    Int(i32),
    Str(Vec<u8>),
}

// `Params` holds a group of parameters. It deals with the one-based
// indexing used in capability strings, as well as the "%i" command
// which turns the first two parameters from zero-based coordinates to
// one-based coordinates.
#[derive(Debug)]
struct Params<'a>(&'a mut [Param]);

impl<'a> Params<'a> {
    fn new(params: &mut [Param]) -> Params {
        Params(params)
    }

    fn get(&self, idx: usize) -> Result<Param, CapError> {
        if idx < 1 || idx > 9 {
            return Err(stx_error("param index must be 1-9"));
        }
        match self.0.get(idx - 1) {
            None |
            Some(&Param::Absent) => Err(run_error("unspecified parameter")),
            Some(&ref p) => Ok(p.clone()),
        }
    }

    fn make_one_based(&mut self) {
        use self::Param::*;
        if let Int(i) = self.0[0] {
            self.0[0] = Int(i + 1);
        }
        if let Int(i) = self.0[1] {
            self.0[1] = Int(i + 1);
        }
    }
}

/// Parameter lists for [`tparm`](fn.tparm.html).
#[macro_export]
macro_rules! params {
    ($($p:expr),* $(,)*) => {{
        #[allow(unused_imports)]
        use $crate::{ToParamFromInt, ToParamFromStr};
        [$($p.to_param()),*]
    }}
}

// ToParamFromStr and ToParamFromInt are only public for use in the
// `params!` macro.
#[doc(hidden)]
pub trait ToParamFromStr {
    fn to_param(&self) -> Param;
}

#[doc(hidden)]
pub trait ToParamFromInt {
    fn to_param(self) -> Param;
}

impl<T> ToParamFromStr for T
where
    T: AsRef<[u8]>,
{
    fn to_param(&self) -> Param {
        Param::Str(self.as_ref().into())
    }
}

impl<T> ToParamFromInt for T
where
    T: Into<i32>,
{
    fn to_param(self) -> Param {
        Param::Int(self.into())
    }
}


/// Variables for [`tparm`](fn.tparm.html).
#[derive(Debug, Default)]
pub struct Vars(Vec<Param>);

impl Vars {
    pub fn new() -> Vars {
        Vars(Vec::new())
    }

    fn set(&mut self, name: char, param: Param) -> Result<(), CapError> {
        let idx = self.idx(name)?;
        if self.0.is_empty() {
            self.0 = vec![Param::Int(0); 52];
        }
        self.0[idx] = param;
        Ok(())
    }

    fn get(&self, name: char) -> Result<Param, CapError> {
        let idx = self.idx(name)?;
        if self.0.is_empty() {
            return Err(var_error(name));
        }
        match self.0[idx] {
            Param::Absent => Err(var_error(name)),
            ref p => Ok(p.clone()),
        }
    }

    fn idx(&self, name: char) -> Result<usize, CapError> {
        if name >= 'A' && name <= 'Z' {
            Ok((name as u8 - b'A') as usize)
        } else if name >= 'a' && name <= 'z' {
            Ok(((name as u8 - b'a') as usize) + 26)
        } else {
            Err(stx_error("invalid variable name"))
        }
    }
}

struct ParamStack(Vec<Param>);

impl ParamStack {
    fn new() -> ParamStack {
        ParamStack(Vec::new())
    }

    fn pop(&mut self) -> Result<Param, CapError> {
        self.0
            .pop()
            .ok_or_else(|| stx_error("pop from empty stack"))
    }

    fn pop_int(&mut self) -> Result<i32, CapError> {
        match self.pop()? {
            Param::Int(i) => Ok(i),
            _ => Err(run_error("expected int parameter")),
        }
    }

    fn pop_str(&mut self) -> Result<Vec<u8>, CapError> {
        match self.pop()? {
            Param::Str(s) => Ok(s),
            _ => Err(run_error("expected str parameter")),
        }
    }

    fn push(&mut self, param: Param) {
        self.0.push(param);
    }
}

struct CapReader<'a> {
    cap: &'a [u8],
    idx: usize,
}

impl<'a> CapReader<'a> {
    fn new(cap: &[u8]) -> CapReader {
        CapReader { cap, idx: 0 }
    }

    fn read(&mut self) -> Option<u8> {
        if self.idx < self.cap.len() {
            self.idx += 1;
            Some(self.cap[self.idx - 1])
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<u8> {
        if self.idx < self.cap.len() {
            Some(self.cap[self.idx])
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Result<char, CapError> {
        match self.peek() {
            Some(c) => Ok(c as char),
            _ => Err(stx_error("unexpected string end")),
        }
    }

    fn read_char(&mut self) -> Result<char, CapError> {
        match self.read() {
            Some(c) => Ok(c as char),
            _ => Err(stx_error("unexpected string end")),
        }
    }

    fn try_number(&mut self) -> Result<Option<u32>, CapError> {
        let mut num = 0u32;
        let mut found = false;
        while let Some(d) = self.peek_char()?.to_digit(10) {
            self.read_char()?;
            num = num.wrapping_mul(10).wrapping_add(d);
            found = true;
        }
        if num >= 10000 {
            Err(stx_error("numeric literal too large"))
        } else {
            Ok(if found { Some(num) } else { None })
        }
    }
}

/// Interpolate parameters into a string capability.
pub fn tparm(
    output: &mut Vec<u8>,
    input: &[u8],
    params: &mut [Param],
    vars: &mut Vars,
) -> Result<(), CapError> {
    use self::Param::*;

    let mut params = Params::new(params);
    let mut cap = CapReader::new(input);
    let mut stack = ParamStack::new();
    output.clear();

    loop {
        // output literal data
        loop {
            match cap.read() {
                Some(b'%') => break,
                Some(c) => output.push(c),
                None => return Ok(()),
            }
        }
        // handle format specifier, if present
        let mut fmt: Formatter = Default::default();
        if ":# 0".find(cap.peek_char()?).is_some() {
            while let Some(_) = ":# -0".find(cap.peek_char()?) {
                fmt.add_flag(cap.read_char()?);
            }
        }
        if let Some(width) = cap.try_number()? {
            fmt.set_width(width);
        }
        if cap.peek_char()? == '.' {
            cap.read_char()?;
            if let Some(prec) = cap.try_number()? {
                fmt.set_prec(prec);
            } else {
                fmt.set_prec(0);
            }
        }
        if fmt.specified() && !"cdoxXs".find(cap.peek_char()?).is_some() {
            return Err(stx_error("unknown format specifier"));
        }
        // handle percent commands
        match cap.read_char()? {
            // push parameter
            'p' => {
                let c = cap.read_char()?;
                match c.to_digit(10) {
                    Some(d) => stack.push(params.get(d as usize)?),
                    _ => return Err(stx_error("invalid param index")),
                }
            }
            // add one to first two parameters
            'i' => {
                params.make_one_based();
            }
            // printing
            'c' => {
                // matching ncurses, we permit but ignore flags for
                // the 'c' specifier
                match stack.pop_int()? {
                    0 => output.push(0x80),
                    i => output.push(i as u8),
                }
            }
            fs @ 'd' | fs @ 'o' | fs @ 'x' | fs @ 'X' => {
                fmt.printf_int(output, fs, stack.pop_int()?);
            }
            's' => {
                fmt.printf_str(output, stack.pop_str()?);
            }
            // if/then/else/endif
            '?' | ';' => (),
            't' => {
                if stack.pop_int()? != 0 {
                    continue;
                }
                let mut lev = 0;
                loop {
                    if cap.read_char()? == '%' {
                        match cap.read_char()? {
                            '?' => lev += 1,
                            ';' if lev == 0 => break,
                            ';' => lev -= 1,
                            'e' if lev == 0 => break,
                            _ => (),
                        }
                    }
                }
            }
            'e' => {
                let mut lev = 0;
                loop {
                    if cap.read_char()? == '%' {
                        match cap.read_char()? {
                            '?' => lev += 1,
                            ';' if lev == 0 => break,
                            ';' => lev -= 1,
                            _ => (),
                        }
                    }
                }
            }
            // push integer constant
            '{' => {
                let ic = cap.try_number()?;
                if ic.is_some() && cap.read_char()? == '}' {
                    stack.push(Int(ic.expect("is_some") as i32));
                } else {
                    return Err(stx_error("invalid int constant"));
                }
            }
            // push char constant
            '\'' => {
                stack.push(Int(cap.read_char()? as i32));
                if cap.read_char()? != '\'' {
                    return Err(stx_error("invalid char constant"));
                }
            }
            // push strlen (top of stack)
            'l' => {
                let str = stack.pop_str()?;
                stack.push(Int(str.len() as i32));
            }
            // unary operators
            '!' => {
                let v1 = stack.pop_int()?;
                stack.push(Int(if v1 == 0 { 1 } else { 0 }));
            }
            '~' => {
                let v1 = stack.pop_int()?;
                stack.push(Int(!v1));
            }
            // logical operators
            op @ '=' | op @ '<' | op @ '>' | op @ 'A' | op @ 'O' => {
                let (v2, v1) = (stack.pop_int()?, stack.pop_int()?);
                let res = match op {
                    '=' => v1 == v2,
                    '<' => v1 < v2,
                    '>' => v1 > v2,
                    'A' => v1 != 0 && v2 != 0,
                    'O' => v1 != 0 || v2 != 0,
                    _ => unreachable!(),
                };
                stack.push(Int(if res { 1 } else { 0 }));
            }
            // arithmetic operators
            op @ '+' | op @ '-' | op @ '*' | op @ '/' | op @ 'm' => {
                let (v2, v1) = (stack.pop_int()?, stack.pop_int()?);
                let res = match op {
                    '+' => v1.wrapping_add(v2),
                    '-' => v1.wrapping_sub(v2),
                    '*' => v1.wrapping_mul(v2),
                    '/' if v2 != 0 => v1.wrapping_div(v2),
                    '/' if v2 == 0 => 0,
                    'm' if v2 != 0 => v1.wrapping_rem(v2),
                    'm' if v2 == 0 => 0,
                    _ => unreachable!(),
                };
                stack.push(Int(res));
            }
            // bitwise operators
            op @ '&' | op @ '|' | op @ '^' => {
                let (v2, v1) = (stack.pop_int()?, stack.pop_int()?);
                let res = match op {
                    '&' => v1 & v2,
                    '|' => v1 | v2,
                    '^' => v1 ^ v2,
                    _ => unreachable!(),
                };
                stack.push(Int(res));
            }
            // output literal %
            '%' => {
                output.push(b'%');
            }
            // set/get variables
            'P' => {
                vars.set(cap.read_char()?, stack.pop()?)?;
            }
            'g' => {
                stack.push(vars.get(cap.read_char()?)?);
            }
            _ => return Err(stx_error("unknown command")),
        }
    }
}

struct Formatter {
    width: u32,
    prec: i32,
    align: Align,
    alt: bool,
    space: bool,
    spec: bool,
}

#[derive(PartialEq)]
#[repr(u8)]
enum Align {
    None = 0,
    LeftJust = 1,
    ZeroPad = 2,
}

impl Default for Formatter {
    fn default() -> Formatter {
        Formatter {
            width: 0,
            prec: -1,
            align: Align::None,
            alt: false,
            space: false,
            spec: false,
        }
    }
}

impl Formatter {
    fn add_flag(&mut self, flag: char) {
        use self::Align::*;
        self.spec = true;
        match flag {
            ':' => (),
            '#' => self.alt = true,
            ' ' => self.space = true,
            '-' => self.align = LeftJust,
            '0' => {
                if self.align != LeftJust {
                    self.align = ZeroPad;
                }
            }
            _ => unreachable!(),
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
        self.spec = true;
    }

    fn set_prec(&mut self, prec: u32) {
        self.prec = prec as i32;
        self.spec = true;
    }

    fn specified(&self) -> bool {
        self.spec
    }

    fn printf_int(&self, w: &mut Vec<u8>, fs: char, val: i32) {
        if self.prec == 0 && val == 0 {
            return;
        }
        let mut output: Vec<u8> = Vec::new();
        if self.alt && (fs == 'o' || fs == 'x' || fs == 'X') {
            output.push(b'0');
            if fs == 'x' {
                output.push(b'x');
            } else if fs == 'X' {
                output.push(b'X');
            }
        }
        let num = match fs {
                'd' => format!("{}", val),
                'o' => format!("{:o}", val),
                'x' => format!("{:x}", val),
                'X' => format!("{:X}", val),
                _ => unreachable!(),
            }
            .into_bytes();
        let mut prec = self.prec;
        if prec != -1 {
            if fs == 'o' && self.alt {
                prec -= 1;
            }
            for _ in 0..(prec - num.len() as i32) {
                output.push(b'0')
            }
        }
        output.extend(num);
        self.printf(w, output);
    }

    fn printf_str(&self, w: &mut Vec<u8>, mut val: Vec<u8>) {
        if self.prec != -1 {
            val.truncate(self.prec as usize);
        }
        self.printf(w, val);
    }

    fn printf(&self, w: &mut Vec<u8>, val: Vec<u8>) {
        if self.align == Align::LeftJust {
            w.extend(val.iter());
        }
        for _ in 0..(self.width as i32 - val.len() as i32) {
            w.push(b' ');
        }
        if self.align != Align::LeftJust {
            w.extend(val.iter());
        }
    }
}


#[derive(Copy, Clone)]
enum PadState {
    Normal,
    Dollar,
    Number(u32, NumPart),
    Finish(u32, NumPart),
}

#[derive(Copy, Clone, PartialEq)]
enum NumPart {
    Whole,
    Dot,
    Frac,
}

/// Print a string capability, applying padding.
///
/// `pad_factor` should be either `1`, or the number of lines affected
/// by executing the capability, for capabilities with proportional
/// padding; `baud` should be the baud rate of the terminal; for a
/// terminal description `desc`, `pad_char` should be:
///
/// * `Some(0)` if `!desc[npc]` and `&desc[pad_char].is_empty()`
/// * `Some(x)` if `![desc[npc]` and `&desc[pad_char] == [x]`
/// * `None` if `desc[npc]`
///
/// If a capability does not use padding, then `tputs(w, cap, ...)` is
/// equivalent to `w.write_all(cap)`. For modern terminal emulators,
/// the only capability that requires padding is `flash` (i.e., visual
/// bell).
pub fn tputs(
    output: &mut Write,
    input: &[u8],
    pad_factor: u32,
    baud: usize,
    pad_char: Option<u8>,
) -> Result<(), CapError> {
    use self::PadState::*;
    use self::NumPart::*;

    let mut start = 0;
    let mut idx = 0;
    let mut state = Normal;
    while idx < input.len() {
        match state {
            Normal => {
                if input[idx] == b'$' {
                    state = Dollar;
                }
                idx += 1;
            }
            Dollar => {
                if input[idx] == b'<' {
                    output.write_all(&input[start..(idx - 1)])?;
                    start = idx - 1;
                    state = Number(0, Whole);
                    idx += 1;
                } else {
                    state = Normal;
                }
            }
            Number(ms, part) => {
                match input[idx] {
                    c if c >= b'0' && c <= b'9' => {
                        idx += 1;
                        let d = (c - b'0') as u32;
                        let x = ms.wrapping_mul(10).wrapping_add(d);
                        match part {
                            Whole => state = Number(x, Whole),
                            Dot => state = Number(x, Frac),
                            // Like ncurses, only use the first digit
                            // after the decimal point.
                            Frac => state = Number(ms, Frac),
                        }
                    }
                    b'.' => {
                        idx += 1;
                        if part == Whole {
                            state = Number(ms, Dot);
                        } else {
                            state = Normal;
                        }
                    }
                    b'*' | b'/' | b'>' => state = Finish(ms, part),
                    _ => state = Normal,
                }
            }
            Finish(mut ms, part) => {
                if let Some(&b'*') = input.get(idx) {
                    ms = ms.wrapping_mul(pad_factor);
                    idx += 1;
                }
                if let Some(&b'/') = input.get(idx) {
                    idx += 1;
                }
                if let Some(&b'>') = input.get(idx) {
                    if part == Frac {
                        ms /= 10;
                    }
                    match pad_char {
                        Some(c) => {
                            let amt = ((baud / 8) * (ms as usize)) / 1000;
                            for _ in 0..amt {
                                output.write_all(&[c])?;
                            }
                        }
                        None => {
                            output.flush()?;
                            sleep(Duration::from_millis(ms as u64));
                        }
                    }
                    idx += 1;
                    start = idx;
                    state = Normal;
                }
            }
        }
    }
    if start < input.len() {
        output.write_all(&input[start..])?;
    }
    Ok(())
}


/// An error that occurred while preparing or printing a string
/// capability.
#[derive(Debug)]
pub struct CapError {
    inner: CapErrorImpl,
}

fn stx_error(msg: &str) -> CapError {
    CapError { inner: CapErrorImpl::Stx(msg.to_owned()) }
}

fn run_error(msg: &str) -> CapError {
    CapError { inner: CapErrorImpl::Run(msg.to_owned()) }
}

fn var_error(c: char) -> CapError {
    CapError { inner: CapErrorImpl::Run(format!("variable {} not set", c)) }
}

#[derive(Debug)]
enum CapErrorImpl {
    Io(io::Error),
    Stx(String),
    Run(String),
}

impl ::std::fmt::Display for CapError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::CapErrorImpl::*;
        match self.inner {
            Io(ref err) => err.fmt(f),
            Stx(ref msg) => write!(f, "{}", msg),
            Run(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl ::std::error::Error for CapError {
    fn description(&self) -> &str {
        use self::CapErrorImpl::*;
        match self.inner {
            Io(ref err) => err.description(),
            Stx(..) => "capability syntax error",
            Run(..) => "capability runtime error",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        use self::CapErrorImpl::*;
        match self.inner {
            Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for CapError {
    fn from(err: io::Error) -> CapError {
        CapError { inner: CapErrorImpl::Io(err) }
    }
}
