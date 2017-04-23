use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::iter;
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::slice;
use std::str;

use self::printf::Formatter;

pub mod cap;
mod printf;


pub struct Desc {
    names: Vec<String>,
    bools: Vec<bool>,
    nums: Vec<u16>,
    strings: Vec<Vec<u8>>,
    def_str: Vec<u8>,
    def_num: u16,
    def_bool: bool,
}

impl Desc {
    pub fn file(term_name: &str) -> Result<File, DescError> {
        if term_name.is_empty() {
            return Err(name_error(term_name));
        }
        match Path::new(term_name).file_name() {
            Some(fname) if fname == Path::new(term_name).as_os_str() => (),
            _ => return Err(name_error(term_name)),
        }

        let first_char = term_name.chars().next().unwrap();
        let first_hex = format!("{:x}", first_char as usize);
        let first_char = first_char.to_string();

        let d1 = to_path(env::var("TERMINFO").ok());
        let d2 = to_path(env::home_dir()).map(|d| d.join(".terminfo"));
        let d3 = to_paths(env::var("TERMINFO_DIRS").ok());
        let d3 = d3.into_iter()
            .map(
                |d| if d.as_os_str().is_empty() {
                    PathBuf::from("/usr/share/terminfo")
                } else {
                    d
                },
            );
        let d4 = vec![
            PathBuf::from("/etc/terminfo"),
            PathBuf::from("/lib/terminfo"),
            PathBuf::from("/usr/share/terminfo"),
        ];
        let ds = d1.into_iter()
            .chain(d2.into_iter())
            .chain(d3)
            .chain(d4.into_iter());

        for d in ds {
            if let Ok(f) = File::open(d.join(&first_char).join(term_name)) {
                return Ok(f);
            }
            if let Ok(f) = File::open(d.join(&first_hex).join(term_name)) {
                return Ok(f);
            }
        }
        return Err(absent_error(term_name));
    }

    pub fn parse(r: &mut Read) -> Result<Desc, DescError> {
        let header = read_words(r, 6)?;
        if header[0] != 282 {
            return Err(parse_error("wrong magic number"));
        }

        let name_sz = header[1] as usize;
        if name_sz == 0 {
            return Err(parse_error("zero-length name section"));
        }
        let name_buf = read_bytes(r, name_sz)?;
        if name_buf[name_sz - 1] != 0 {
            return Err(parse_error("name section is not null-terminated"));
        }
        let names = str::from_utf8(&name_buf[0..name_sz - 1])?;
        let names: Vec<_> = names.split('|').map(str::to_owned).collect();

        let bools_num = header[2] as usize;
        if bools_num > cap::NUM_BOOLS {
            return Err(parse_error("too many boolean flags"));
        }
        let bool_buf = read_bytes(r, bools_num)?;
        let bools = bool_buf
            .into_iter()
            .map(|b| if b == 0 { false } else { true })
            .collect();

        if (name_sz + bools_num) % 2 != 0 {
            read_bytes(r, 1)?;
        }

        let ints_num = header[3] as usize;
        if ints_num > cap::NUM_INTS {
            return Err(parse_error("too many numbers"));
        }
        let nums = read_words(r, ints_num)?;

        let strings_num = header[4] as usize;
        let string_sz = header[5] as usize;
        if strings_num > cap::NUM_STRINGS {
            return Err(parse_error("too many strings"));
        }
        let offsets = read_words(r, strings_num)?;
        let str_table = read_bytes(r, string_sz)?;
        let mut strings = Vec::new();
        for pos in offsets {
            let pos = pos as usize;
            if pos == 0xffff || pos == 0xfffe {
                strings.push(Vec::new());
            } else if pos >= string_sz {
                return Err(parse_error("invalid string offset"));
            } else {
                match str_table[pos..].iter().position(|&b| b == 0) {
                    None => return Err(parse_error("unterminated string")),
                    Some(end) => {
                        strings.push(str_table[pos..pos + end].to_vec());
                    }
                }
            }
        }

        Ok(
            Desc {
                names: names,
                bools: bools,
                nums: nums,
                strings: strings,
                def_str: Vec::new(),
                def_num: 0xffff,
                def_bool: false,
            },
        )
    }

    pub fn names(&self) -> slice::Iter<String> {
        self.names.iter()
    }

    // Only public for use in `desc!` macro.
    #[doc(hidden)]
    pub fn new(names: &[String], pairs: &[cap::DPair]) -> Desc {
        use self::cap::Data::*;

        let mut bools: Vec<bool> = vec![false; cap::NUM_BOOLS];
        let mut nums: Vec<u16> = vec![0xffff; cap::NUM_INTS];
        let mut strings: Vec<Vec<u8>> = vec![Vec::new(); cap::NUM_STRINGS];
        let mut maxb: usize = 0;
        let mut maxn: usize = 0;
        let mut maxs: usize = 0;

        for pair in pairs {
            match *pair {
                (i, Bool(v)) if v => {
                    if i >= maxb {
                        maxb = i + 1;
                    }
                    bools[i] = v;
                }
                (i, Num(v)) if v != 0xffff => {
                    if i >= maxn {
                        maxn = i + 1;
                    }
                    nums[i] = v;
                }
                (i, Str(ref v)) if v.len() > 0 => {
                    if i >= maxs {
                        maxs = i + 1;
                    }
                    strings[i] = v.clone();
                }
                _ => (),
            }
        }
        bools.truncate(maxb);
        bools.shrink_to_fit();
        nums.truncate(maxn);
        nums.shrink_to_fit();
        strings.truncate(maxs);
        strings.shrink_to_fit();

        Desc {
            names: Vec::from(names),
            bools: bools,
            nums: nums,
            strings: strings,
            def_str: Vec::new(),
            def_num: 0xffff,
            def_bool: false,
        }
    }
}


impl Index<cap::Boolean> for Desc {
    type Output = bool;
    fn index(&self, index: cap::Boolean) -> &bool {
        let idx = usize::from(&index);
        if self.bools.len() > idx {
            &self.bools[idx]
        } else {
            &self.def_bool
        }
    }
}

impl Index<cap::Number> for Desc {
    type Output = u16;
    fn index(&self, index: cap::Number) -> &u16 {
        let idx = usize::from(&index);
        if self.nums.len() > idx {
            &self.nums[idx]
        } else {
            &self.def_num
        }
    }
}

impl Index<cap::String> for Desc {
    type Output = [u8];
    fn index(&self, index: cap::String) -> &[u8] {
        let idx = usize::from(&index);
        if self.strings.len() > idx {
            &self.strings[idx]
        } else {
            &self.def_str
        }
    }
}


#[macro_export]
macro_rules! desc {
    // Finish processing.
    (@acc [$($ns:expr),*] [$($ps:expr),*]) => {
        Desc::new(
            &[$(::std::string::String::from($ns)),*][..],
            &[$($ps),*][..]
        );
    };

    // Add a name.
    (@acc [$($ns:expr),*] [$($ps:expr),*] $n:expr) => {
        desc!(@acc [$($ns,)* $n] [$($ps),*]);
    };
    (@acc [$($ns:expr),*] [$($ps:expr),*] $n:expr, $($xs:tt)*) => {
        desc!(@acc [$($ns,)* $n] [$($ps),*] $($xs)*);
    };

    // Add a cap => val pair.
    (@acc [$($ns:expr),*] [$($ps:expr),*] $k:expr => $v:expr) => {
        desc!(@acc [$($ns),*] [$($ps,)* $k.data($v)]);
    };
    (@acc [$($ns:expr),*] [$($ps:expr),*] $k:expr => $v:expr, $($xs:tt)*) => {
        desc!(@acc [$($ns),*] [$($ps,)* $k.data($v)] $($xs)*);
    };

    // Start, with a name.
    ($n:expr) => { desc!(@acc [$n] []); };
    ($n:expr, $($xs:tt)*) => { desc!(@acc [$n] [] $($xs)*); };

    // Start, with a cap => val pair.
    ($k:expr => $v:expr) => { desc!(@acc [] [$k.data($v)]); };
    ($k:expr => $v:expr, $($xs:tt)*) => {
        desc!(@acc [] [$k.data($v)] $($xs)*);
    };
}


#[derive(Clone, Debug)]
pub enum Param {
    Absent,
    Int(i32),
    Str(Vec<u8>),
}

impl Default for Param {
    fn default() -> Param {
        Param::Absent
    }
}

impl From<i8> for Param {
    fn from(int: i8) -> Param {
        Param::Int(int as i32)
    }
}

impl From<u8> for Param {
    fn from(int: u8) -> Param {
        Param::Int(int as i32)
    }
}

impl From<i16> for Param {
    fn from(int: i16) -> Param {
        Param::Int(int as i32)
    }
}

impl From<u16> for Param {
    fn from(int: u16) -> Param {
        Param::Int(int as i32)
    }
}

impl From<i32> for Param {
    fn from(int: i32) -> Param {
        Param::Int(int)
    }
}

impl<'a> From<&'a str> for Param {
    fn from(str: &'a str) -> Param {
        Param::Str(str.into())
    }
}

impl From<String> for Param {
    fn from(str: String) -> Param {
        Param::Str(str.into())
    }
}

impl<'a> From<&'a [u8]> for Param {
    fn from(str: &'a [u8]) -> Param {
        Param::Str(str.into())
    }
}

impl From<Vec<u8>> for Param {
    fn from(str: Vec<u8>) -> Param {
        Param::Str(str)
    }
}

#[derive(Debug)]
pub struct Params([Param; 9]);

impl Params {
    pub fn new(params: [Param; 9]) -> Params {
        Params(params)
    }

    fn get(&self, idx: usize) -> Result<Param, CapError> {
        if idx < 1 || idx > 9 {
            return Err(stx_error("param index must be 1-9"));
        }
        match self.0[idx - 1] {
            Param::Absent => {
                panic!("unspecified parameter");
            }
            ref p => Ok(p.clone()),
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

#[macro_export]
macro_rules! params {
    () => {
        $crate::terminfo::Params::new(
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr, $p5:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), $p5.into(), Default::default(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr, $p5:expr, $p6:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), $p5.into(), $p6.into(),
             Default::default(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr, $p5:expr, $p6:expr,
     $p7:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), $p5.into(), $p6.into(),
             $p7.into(), Default::default(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr, $p5:expr, $p6:expr,
     $p7:expr, $p8:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), $p5.into(), $p6.into(),
             $p7.into(), $p8.into(), Default::default(),
            ])
    };
    ($p1:expr, $p2:expr, $p3:expr,
     $p4:expr, $p5:expr, $p6:expr,
     $p7:expr, $p8:expr, $p9:expr) => {
        $crate::terminfo::Params::new(
            [$p1.into(), $p2.into(), $p3.into(),
             $p4.into(), $p5.into(), $p6.into(),
             $p7.into(), $p8.into(), $p9.into(),
            ])
    };
    ($($arg:tt),*) => {too many args to params};
}

pub struct Vars(Vec<Param>);

impl Vars {
    pub fn new() -> Vars {
        Vars(Vec::new())
    }

    fn set(&mut self, name: char, param: Param) -> Result<(), CapError> {
        let idx = self.idx(name)?;
        if self.0.len() == 0 {
            self.0 = vec![Param::Absent; 52];
        }
        self.0[idx] = param;
        Ok(())
    }

    fn get(&self, name: char) -> Result<Param, CapError> {
        let idx = self.idx(name)?;
        if self.0.len() == 0 {
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
        self.0.pop().ok_or(stx_error("pop from empty stack"))
    }

    fn pop_int(&mut self) -> Result<i32, CapError> {
        match self.pop()? {
            Param::Int(i) => Ok(i),
            Param::Str(_) => panic!("string passed for int parameter"),
            _ => unreachable!(),
        }
    }

    fn pop_str(&mut self) -> Result<Vec<u8>, CapError> {
        match self.pop()? {
            Param::Str(s) => Ok(s),
            Param::Int(_) => panic!("int passed for str parameter"),
            _ => unreachable!(),
        }
    }

    fn push(&mut self, param: Param) {
        self.0.push(param);
    }
}

struct CapIter<'a>(iter::Peekable<slice::Iter<'a, u8>>);

impl<'a> CapIter<'a> {
    fn new(cap: &[u8]) -> CapIter {
        CapIter(cap.iter().peekable())
    }

    fn read(&mut self) -> Option<u8> {
        self.0.next().map(|c| *c)
    }

    fn peek(&mut self) -> Option<u8> {
        self.0.peek().map(|c| **c)
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
            num = num * 10 + d;
            found = true;
        }
        Ok(if found { Some(num) } else { None })
    }
}

pub fn tparm(
    output: &mut Vec<u8>,
    input: &[u8],
    params: &mut Params,
    vars: &mut Vars,
) -> Result<(), CapError> {

    use self::Param::*;

    output.clear();
    let mut cap = CapIter::new(input);
    let mut stack = ParamStack::new();

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
        if let Some(_) = ":# 0".find(cap.peek_char()?) {
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
        if fmt.specified() {
            if !"cdoxXs".find(cap.peek_char()?).is_some() {
                return Err(stx_error("unknown format specifier"));
            }
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
            '?' => (),
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
            ';' => (),
            // push integer constant
            '{' => {
                let ic = cap.try_number()?;
                if ic.is_some() && cap.read_char()? == '}' {
                    stack.push(Int(ic.unwrap() as i32));
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
                    '+' => v1 + v2,
                    '-' => v1 - v2,
                    '*' => v1 * v2,
                    '/' => v1 / v2,
                    'm' => v1 % v2,
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
                output.push('%' as u8);
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


#[derive(Debug)]
pub struct CapError {
    inner: CapErrorImpl,
}

fn stx_error(msg: &str) -> CapError {
    CapError { inner: CapErrorImpl::Stx(msg.to_owned()) }
}

fn var_error(c: char) -> CapError {
    CapError { inner: CapErrorImpl::Var(c) }
}

#[derive(Debug)]
enum CapErrorImpl {
    Stx(String),
    Var(char),
}

impl fmt::Display for CapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CapErrorImpl::*;
        match self.inner {
            Stx(ref msg) => write!(f, "{}", msg),
            Var(ref name) => write!(f, "variable {} not set", name),
        }
    }
}

impl error::Error for CapError {
    fn description(&self) -> &str {
        use self::CapErrorImpl::*;
        match self.inner {
            Stx(..) => "capability syntax error",
            Var(..) => "capability variable error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


#[derive(Debug)]
pub struct DescError {
    inner: DescErrorImpl,
}

fn parse_error(msg: &str) -> DescError {
    DescError { inner: DescErrorImpl::Parse(msg.to_owned()) }
}

fn absent_error(name: &str) -> DescError {
    DescError { inner: DescErrorImpl::Absent(name.to_owned()) }
}

fn name_error(name: &str) -> DescError {
    DescError { inner: DescErrorImpl::Name(name.to_owned()) }
}

#[derive(Debug)]
enum DescErrorImpl {
    Io(io::Error),
    Parse(String),
    Absent(String),
    Name(String),
}

impl fmt::Display for DescError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DescErrorImpl::*;
        match self.inner {
            Io(ref err) => err.fmt(f),
            Parse(ref msg) => write!(f, "{}", msg),
            Absent(ref name) => write!(f, "no description found for {}", name),
            Name(ref name) => write!(f, "invalid terminal name '{}'", name),
        }
    }
}

impl error::Error for DescError {
    fn description(&self) -> &str {
        use self::DescErrorImpl::*;
        match self.inner {
            Io(ref err) => err.description(),
            Parse(..) => "invalid terminfo description",
            Absent(..) => "missing terminfo description",
            Name(..) => "invalid terminal name",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::DescErrorImpl::*;
        match self.inner {
            Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for DescError {
    fn from(err: io::Error) -> DescError {
        DescError { inner: DescErrorImpl::Io(err) }
    }
}

impl From<str::Utf8Error> for DescError {
    fn from(err: str::Utf8Error) -> DescError {
        parse_error(&err.to_string())
    }
}

fn to_path<T: Into<PathBuf>>(var: Option<T>) -> Option<PathBuf> {
    match var {
        Some(d) => {
            let d = d.into();
            if d.as_os_str().is_empty() {
                None
            } else {
                Some(d)
            }
        }
        None => None,
    }
}

fn to_paths(var: Option<String>) -> Vec<PathBuf> {
    match var {
        None => Vec::new(),
        Some(ref d) if d.is_empty() => Vec::new(),
        Some(d) => env::split_paths(&d).collect(),
    }
}


fn read_bytes(r: &mut Read, n: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(n);
    if r.take(n as u64).read_to_end(&mut buf)? < n {
        return Err(
            io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ),
        );
    }
    Ok(buf)
}

fn read_words(r: &mut Read, n: usize) -> io::Result<Vec<u16>> {
    let mut buf_16: Vec<u16> = vec![0; n];
    let mut buf_8 = unsafe {
        slice::from_raw_parts_mut(buf_16.as_mut_ptr() as *mut u8, n * 2)
    };
    r.read_exact(&mut buf_8)?;
    if cfg!(target_endian = "big") {
        for w in &mut buf_16 {
            *w = w.swap_bytes();
        }
    }
    Ok(buf_16)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_literal() {
        use super::cap::*;
        let desc = desc![
            "dumb", "80-column dumb tty",
            am => true,
            cols => 80,
            bel => &[7u8][..]
        ];
        assert_eq!(desc[bw], false);
        assert_eq!(desc[am], true);
        assert_eq!(desc[xsb], false);
        assert_eq!(desc[cols], 80);
        assert_eq!(&desc[bel], &[7u8]);
    }

    #[test]
    fn tparm_basic_setabf() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"\\E[48;5;%p1%dm",
            &mut params!(1),
            &mut Vars::new(),
        );
        assert_eq!(output, b"\\E[48;5;1m");
    }

    #[test]
    fn tparm_multiple_int_constants() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"%{1}%{2}%d%d",
            &mut params!(),
            &mut Vars::new(),
        );
        assert_eq!(output, b"21");
    }

    #[test]
    fn tparm_op_i() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d",
            &mut params!(1, 2, 3),
            &mut Vars::new(),
        );
        assert_eq!(output, b"123233");
    }

    #[test]
    fn tparm_conditionals() {
        let mut output = Vec::new();
        let cap =
            b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
        let _ = tparm(&mut output, cap, &mut params!(1), &mut Vars::new());
        assert_eq!(output, b"\\E[31m");
        let _ = tparm(&mut output, cap, &mut params!(8), &mut Vars::new());
        assert_eq!(output, b"\\E[90m");
        let _ = tparm(&mut output, cap, &mut params!(42), &mut Vars::new());
        assert_eq!(output, b"\\E[38;5;42m");
    }

    #[test]
    fn tparm_format() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"%p1%s%p2%2s%p3%2s%p4%.2s",
            &mut params!(b"foo"[..], b"foo"[..], b"f"[..], b"foo"[..]),
            &mut Vars::new(),
        );
        assert_eq!(output, b"foofoo ffo");

        let _ = tparm(
            &mut output,
            b"%p1%:-4.2s",
            &mut params!(b"foo"[..]),
            &mut Vars::new(),
        );
        assert_eq!(output, b"fo  ");

        let _ = tparm(
            &mut output,
            b"%p1%d%p1%.3d%p1%5d%p1%:d",
            &mut params!(1),
            &mut Vars::new(),
        );
        assert_eq!(output, b"1001    11");

        let _ = tparm(
            &mut output,
            b"%p1%o%p1%#o%p2%6.4x%p2%#6.4X",
            &mut params!(15, 27),
            &mut Vars::new(),
        );
        assert_eq!(output, b"17017  001b0X001B");
    }

    #[test]
    fn tparm_vars() {
        let mut output = Vec::new();
        let mut vars = Vars::new();
        let cap = b"%?%p1%{1}%=%t%'h'%Pa%e%'l'%Pa%;\
              \\E[?1000%ga%c\\E[?1003%ga%c\\E[?1006%ga%c";

        let _ = tparm(&mut output, cap, &mut params!(1), &mut vars);
        assert_eq!(output, b"\\E[?1000h\\E[?1003h\\E[?1006h");

        let _ = tparm(&mut output, cap, &mut params!(0), &mut vars);
        assert_eq!(output, b"\\E[?1000l\\E[?1003l\\E[?1006l");
    }
}
