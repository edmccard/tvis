use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::slice;
use std::str;


pub mod cap;

pub struct Desc {
    pub names: Vec<String>,
    pub bools: Vec<bool>,
    pub nums: Vec<u16>,
    pub strings: Vec<String>,
}

impl Desc {
    pub fn file(name: &str) -> Option<File> {
        if name.is_empty() {
            return None;
        }

        let first_char = name.chars().next().unwrap();
        let first_hex = format!("{:x}", first_char as usize);
        let first_char = first_char.to_string();

        let d1 = to_path(env::var("TERMINFO").ok());
        let d2 = to_path(env::home_dir()).map(|d| d.join(".terminfo"));
        let d3 = to_paths(env::var("TERMINFO_DIRS").ok());
        let d3 = d3.into_iter().map(|d| if d.as_os_str().is_empty() {
                                        PathBuf::from("/usr/share/terminfo")
                                    } else {
                                        d
                                    });
        let d4 = vec![PathBuf::from("/etc/terminfo"),
                      PathBuf::from("/lib/terminfo"),
                      PathBuf::from("/usr/share/terminfo")];
        let ds = d1.into_iter()
            .chain(d2.into_iter())
            .chain(d3)
            .chain(d4.into_iter());

        for d in ds {
            if let Ok(f) = File::open(d.join(&first_char).join(name)) {
                return Some(f);
            }
            if let Ok(f) = File::open(d.join(&first_hex).join(name)) {
                return Some(f);
            }
        }
        return None;
    }

    pub fn parse(r: &mut Read) -> Result<Desc, ParseError> {
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
        if names.len() < 2 {
            return Err(parse_error("too few items in name section"));
        }

        let bools_num = header[2] as usize;
        if bools_num > cap::NUM_BOOLS {
            return Err(parse_error("too many boolean flags"));
        }
        let bool_buf = read_bytes(r, bools_num)?;
        let bools = bool_buf.into_iter()
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
            if pos == 0xffff {
                strings.push(String::new());
            } else if pos >= string_sz {
                return Err(parse_error("invalid string offset"));
            } else {
                match str_table[pos..].iter().position(|&b| b == 0) {
                    None => return Err(parse_error("unterminated string")),
                    Some(end) => {
                        let s = str::from_utf8(&str_table[pos..pos + end])?;
                        strings.push(s.to_owned());
                    }
                }
            }
        }

        Ok(Desc {
               names: names,
               bools: bools,
               nums: nums,
               strings: strings,
           })
    }
}


#[derive(Debug)]
pub struct ParseError {
    inner: ParseErrorImpl,
}

fn parse_error(msg: &str) -> ParseError {
    ParseError { inner: ParseErrorImpl::Other(msg.to_owned()) }
}

#[derive(Debug)]
enum ParseErrorImpl {
    Io(io::Error),
    Other(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ParseErrorImpl::Io(ref err) => err.fmt(f),
            ParseErrorImpl::Other(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match self.inner {
            ParseErrorImpl::Io(ref err) => err.description(),
            _ => "invalid terminfo description",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.inner {
            ParseErrorImpl::Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError { inner: ParseErrorImpl::Io(err) }
    }
}

impl From<str::Utf8Error> for ParseError {
    fn from(err: str::Utf8Error) -> ParseError {
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
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof,
                                  "failed to fill whole buffer"));
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
