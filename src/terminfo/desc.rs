use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::str;

use super::cap;


/// A terminal description.
///
/// The terminal's capabilities are read by indexing a `Desc` object
/// with a [`Boolean`](cap/struct.Boolean.html),
/// [`Number`](cap/struct.Number.html), or
/// [`String`](cap/struct.String.html) capability name. For example,
/// `desc[bw]` returns a `bool`, `desc[cols]` returns a `u16`, and
/// `&desc[setaf]` returns a `&[u8]`.
///
/// # Examples
///
/// ```ignore
/// # use tvis::terminfo::DescError;
/// # fn foo() -> Result<(), DescError> {
/// use tvis::terminfo::Desc;
/// use tvis::terminfo::cap::rs1;
///
/// let mut file = Desc::file("xterm-256color")?;
/// let desc = Desc::parse(&mut file)?;
/// assert_eq!(&desc[rs1], b"\x1bc\x1b]104\x07");
/// # Ok(())
/// # }
/// ```
///
/// The [`desc!` macro](../macro.desc.html) provides syntax for `Desc`
/// literals.
pub struct Desc {
    names: Vec<String>,
    bools: Vec<bool>,
    nums: Vec<u16>,
    strings: Vec<Vec<u8>>,
}

impl Desc {
    /// Finds and opens the compiled terminfo description for the
    /// terminal named by `term_name`.
    ///
    /// This assumes that the local terminfo database uses
    /// the "directory tree" format for storing compiled descriptions,
    /// and it searches in these directories:
    ///
    /// 1. The directory named by the `TERMINFO` environment variable.
    /// 2. `$HOME/.terminfo`
    /// 3. The list of directories named by the `TERMINFO_DIRS`
    /// environment variable (with empty entries replaced by
    /// `/usr/share/terminfo`).
    /// 4. `/etc/terminfo`
    /// 5. `/lib/terminfo`
    /// 6. `/usr/share/terminfo`
    ///
    /// For each directory, `file` checks for a description named
    /// `term_name` in a subdirectory named by the first letter of
    /// `term_name` as a character or hex representation, for example
    /// `/x/xterm` or `/78/xterm`. (Note that if `term_name` has more
    /// than one path component, only the last one is used).
    ///
    /// #Errors
    ///
    /// This returns an error if `file` could not find and open a
    /// description for `term_name`, or if `term_name` is invalid.
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

        Err(absent_error(term_name))
    }

    /// Creates a `Desc` from a compiled terminfo description.
    ///
    /// # Errors
    ///
    /// This returns an error if the input is not a valid terminfo
    /// description.
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
        let bools = bool_buf.into_iter().map(|b| !(b == 0)).collect();

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
                names,
                bools,
                nums,
                strings,
            },
        )
    }

    /// The terminal's names.
    pub fn names(&self) -> &[String] {
        &self.names
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
                (i, Str(ref v)) if !v.is_empty() => {
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
            bools,
            nums,
            strings,
        }
    }
}


static DEF_BOOL: bool = false;

impl Index<cap::Boolean> for Desc {
    #[doc(hidden)]
    type Output = bool;

    /// The value of the boolean capability named by `index`.
    fn index(&self, index: cap::Boolean) -> &bool {
        let idx = usize::from(index);
        if self.bools.len() > idx {
            &self.bools[idx]
        } else {
            &DEF_BOOL
        }
    }
}

static DEF_NUM: u16 = 0xffff;

impl Index<cap::Number> for Desc {
    #[doc(hidden)]
    type Output = u16;

    /// The value of the numeric capability named by `index`.
    fn index(&self, index: cap::Number) -> &u16 {
        let idx = usize::from(index);
        if self.nums.len() > idx {
            &self.nums[idx]
        } else {
            &DEF_NUM
        }
    }
}

static DEF_STR: &[u8] = &[];

impl Index<cap::String> for Desc {
    #[doc(hidden)]
    type Output = [u8];

    /// The value of the string capability named by `index`.
    fn index(&self, index: cap::String) -> &[u8] {
        let idx = usize::from(index);
        if self.strings.len() > idx {
            &self.strings[idx]
        } else {
            DEF_STR
        }
    }
}


/// A syntax for [`terminfo::Desc`](terminfo/struct.Desc.html) literals.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate tvis;
/// # fn main() {
/// use tvis::terminfo::cap::*;
///
/// let desc = desc![
///     "dumb", "80-column dumb terminal",
///     am => true,
///     cols => 80,
///     bel => "\x07",
///     cr => "\r",
///     cud1 => "\n",
///     ind => "\n",
/// ];
/// # }
/// ```
///
/// Terminal names are specified by strings; capabilities are
/// specified by `name => val` pairs, where `name` is a
/// [`Boolean`](../terminfo/cap/struct.Boolean.html),
/// [`Number`](../terminfo/cap/struct.Number.html), or
/// [`String`](../terminfo/cap/struct.String.html) capability name,
/// and `val` is a `bool`, `u16`, or `Into<Vec<u8>>` respectively.
#[macro_export]
macro_rules! desc {
    // Finish processing.
    (@acc [$($ns:expr),*] [$($ps:expr),*]) => {
        $crate::terminfo::Desc::new(
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
        ::std::slice::from_raw_parts_mut(buf_16.as_mut_ptr() as *mut u8, n * 2)
    };
    r.read_exact(&mut buf_8)?;
    if cfg!(target_endian = "big") {
        for w in &mut buf_16 {
            *w = w.swap_bytes();
        }
    }
    Ok(buf_16)
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


/// An error that occurred while finding or parsing a terminal
/// description.
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

impl ::std::fmt::Display for DescError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::DescErrorImpl::*;
        match self.inner {
            Io(ref err) => err.fmt(f),
            Parse(ref msg) => write!(f, "{}", msg),
            Absent(ref name) => write!(f, "no description found for {}", name),
            Name(ref name) => write!(f, "invalid terminal name '{}'", name),
        }
    }
}

impl ::std::error::Error for DescError {
    fn description(&self) -> &str {
        use self::DescErrorImpl::*;
        match self.inner {
            Io(ref err) => err.description(),
            Parse(..) => "invalid terminfo description",
            Absent(..) => "missing terminfo description",
            Name(..) => "invalid terminal name",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
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
