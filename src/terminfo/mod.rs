use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::slice;
use std::str;

pub mod cap;


pub struct Desc {
    names: Vec<String>,
    bools: Vec<bool>,
    nums: Vec<u16>,
    strings: Vec<String>,
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

    pub fn names(&self) -> slice::Iter<String> {
        self.names.iter()
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
        match self.inner {
            DescErrorImpl::Io(ref err) => err.fmt(f),
            DescErrorImpl::Parse(ref msg) => write!(f, "{}", msg),
            DescErrorImpl::Absent(ref name) => {
                write!(f, "no description found for {}", name)
            }
            DescErrorImpl::Name(ref name) => {
                write!(f, "invalid terminal name '{}'", name)
            }
        }
    }
}

impl error::Error for DescError {
    fn description(&self) -> &str {
        match self.inner {
            DescErrorImpl::Io(ref err) => err.description(),
            DescErrorImpl::Parse(..) => "invalid terminfo description",
            DescErrorImpl::Absent(..) => "missing terminfo description",
            DescErrorImpl::Name(..) => "invalid terminal name",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.inner {
            DescErrorImpl::Io(ref err) => err.cause(),
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
