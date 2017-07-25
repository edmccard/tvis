//! A low-level interface to terminfo databases.
//!
//! ## Usage
//!
//! To find and read terminal descriptions, see
//! [`Desc`](struct.Desc.html); to send commands to a terminal, see
//! [`tparm`](fn.tparm.html) and [`tputs`](fn.tputs.html).
//!
//! ## Examples
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate tinf;
//! # fn main() {
//! #     foo().unwrap();
//! # }
//! # fn foo() -> Result<(), Box<std::error::Error>> {
//! # use std::io::Write;
//! use tinf::{Desc, tparm, Vars};
//!
//! // Find the description for "xterm" in the default locations.
//! let mut file = Desc::file("xterm")?;
//!
//! // Parse it into a `Desc` object.
//! let desc = Desc::parse(&mut file)?;
//!
//! // Send the escape sequence to set foreground to red.
//! use tinf::cap::setaf;
//! let stdout = &mut std::io::stdout();
//! let mut vars = Vars::new();
//! tparm(stdout, &desc[setaf], &mut params!(1), &mut vars)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Platform Compatibility
//!
//! This requires a local terminfo database in directory tree format;
//! it will not work with a hashed database. In other words, it should
//! Just Work on Linux/OSX/Cygwin, but it might not work out of the
//! box on BSD operating systems.

#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str;

pub mod cap;
#[macro_use]
mod print;

pub use self::print::{CapError, Param, ToParamFromInt, ToParamFromStr, Vars,
                      tparm, tputs};
use self::cap::{Cap, ICap, CapName, UserDef};


/// The names and capabilities that make up a terminal description.
///
/// Predefined capabilities are read by indexing a `Desc` object
/// with a [`Boolean`](cap/struct.Boolean.html),
/// [`Number`](cap/struct.Number.html), or
/// [`String`](cap/struct.String.html) capability name. For example,
/// `desc[bw]` returns a `bool`, `desc[cols]` returns a `u16`, and
/// `&desc[setaf]` returns a `&[u8]`. User-defined capabilities are
/// queried using the `get_*_ext()` methods.
///
/// An absent capability will be `false`, `0xffff`, or an empty
/// slice, for booleans, numbers, and strings respectively.
///
/// The [`desc!` macro](macro.desc.html) provides syntax for `Desc`
/// literals.
///
/// # Examples
///
/// Read the description for `xterm-256color` and look up the `rs1`
/// capability:
///
/// ```no_run
/// # use tinf::DescError;
/// # fn foo() -> Result<(), DescError> {
/// use tinf::Desc;
/// use tinf::cap::rs1;
///
/// let mut file = Desc::file("xterm-256color")?;
/// let desc = Desc::parse(&mut file)?;
/// assert_eq!(&desc[rs1], b"\x1bc\x1b]104\x07");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Desc {
    names: Vec<String>,
    bools: Vec<bool>,
    nums: Vec<u16>,
    strings: Vec<Vec<u8>>,
    ext: Vec<ICap>,
}

impl Desc {
    /// Finds and opens the compiled terminfo description for the
    /// terminal named by `term_name`.
    ///
    /// This assumes that the local terminfo database uses a directory
    /// tree format for storing compiled descriptions, and it searches
    /// in these directories:
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
        fn path_to_root(path: &str) -> PathBuf {
            FS_ROOT.join(path)
        }

        if term_name.is_empty() {
            return Err(name_error(term_name));
        }
        match Path::new(term_name).file_name() {
            Some(fname) if fname == Path::new(term_name).as_os_str() => (),
            _ => return Err(name_error(term_name)),
        }

        let first_char =
            term_name.chars().next().expected("non-empty term_name");
        let first_hex = format!("{:x}", first_char as usize);
        let first_char = first_char.to_string();

        let d1 = to_path(env::var("TERMINFO").ok());
        let d2 = to_path(env::home_dir()).map(|d| d.join(".terminfo"));
        let d3 = to_paths(env::var("TERMINFO_DIRS").ok());
        let d3 = d3.into_iter().map(|d| if d.as_os_str().is_empty() {
            path_to_root("usr/share/terminfo")
        } else {
            d
        });
        let d4 = vec![
            path_to_root("etc/terminfo"),
            path_to_root("lib/terminfo"),
            path_to_root("usr/share/terminfo"),
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
        let mut r = &mut AlignReader::new(r);

        let header = r.read_words(6)?;
        if header[0] != 282 {
            return Err(parse_error("wrong magic number"));
        }

        let name_sz = header[1] as usize;
        if name_sz == 0 {
            return Err(parse_error("zero-length names"));
        }
        let name_buf = r.read_bytes(name_sz)?;
        if name_buf[name_sz - 1] != 0 {
            return Err(parse_error("names are not null-terminated"));
        }
        let names = str::from_utf8(&name_buf[0..name_sz - 1])?;
        let names: Vec<_> = names.split('|').map(str::to_owned).collect();

        let bools_num = header[2] as usize;
        if bools_num > cap::NUM_BOOLS {
            return Err(parse_error("too many boolean flags"));
        }
        let bools = Desc::read_bools(r, bools_num)?;

        let ints_num = header[3] as usize;
        if ints_num > cap::NUM_INTS {
            return Err(parse_error("too many numbers"));
        }
        let nums = r.read_words(ints_num)?;

        let strings_num = header[4] as usize;
        let string_sz = header[5] as usize;
        if strings_num > cap::NUM_STRS {
            return Err(parse_error("too many strings"));
        }
        let offsets = r.read_words(strings_num)?;
        let table = r.read_bytes(string_sz)?;
        let strings = Desc::read_strs(&[&offsets], &table)?
            .pop()
            .expected("read_strs with length 1");

        Ok(Desc {
            names,
            bools,
            nums,
            strings,
            ext: Desc::parse_user(r)?,
        })
    }

    /// The description for the terminal type from the `TERM`
    /// environment variable, or the "dumb terminal" description if
    /// `TERM` is empty.
    pub fn current() -> &'static Desc {
        &*CURRENT
    }

    // Returns user-defined capabilities, or an empty vector if the
    // reader is exhausted, or an error if there is extra data that is
    // invalid.
    fn parse_user(r: &mut AlignReader) -> Result<Vec<ICap>, DescError> {
        let ext_header = r.read_words(5);
        if let Err(e) = ext_header {
            return match e.kind() {
                io::ErrorKind::UnexpectedEof => Ok(Vec::new()),
                _ => Err(DescError::from(e)),
            };
        }
        let ext_header = ext_header.expected("Ok() ext_header");

        let mut ext_bools = Desc::read_bools(r, ext_header[0] as usize)?;
        let mut ext_nums = r.read_words(ext_header[1] as usize)?;
        let ext_offs = r.read_words(ext_header[2] as usize)?;
        let ext_name_offs =
            r.read_words(ext_bools.len() + ext_nums.len() + ext_offs.len())?;
        let ext_table = r.read_bytes(ext_header[4] as usize)?;

        let mut ext_data =
            Desc::read_strs(&[&ext_offs, &ext_name_offs], &ext_table)?;
        let mut ext_names = ext_data.pop().expected("ext_data.len() == 2");
        // TODO: must be unique and not names of predefined.
        let mut ext_strs = ext_data.pop().expected("ext_data.len() == 2");

        let mut ext = Vec::new();
        ext_strs.reverse();
        for val in ext_strs {
            let name = &ext_names
                .pop()
                .expected("names.len == (strs + nums + bools).len");
            let name = UserDef(str::from_utf8(name)?.to_owned());
            ext.push(ICap::Str(CapName::U(name), val));
        }
        ext_nums.reverse();
        for val in ext_nums {
            let name = &ext_names
                .pop()
                .expected("names.len == (strs + nums + bools).len");
            let name = UserDef(str::from_utf8(name)?.to_owned());
            ext.push(ICap::Num(CapName::U(name), val));
        }
        ext_bools.reverse();
        for val in ext_bools {
            let name = &ext_names
                .pop()
                .expected("names.len == (strs + nums + bools).len");
            let name = UserDef(str::from_utf8(name)?.to_owned());
            ext.push(ICap::Bool(CapName::U(name), val));
        }

        Ok(ext)
    }

    // Turns a sequence of 0 or 1 bytes into a vector of bool.
    fn read_bools(r: &mut AlignReader, n: usize) -> io::Result<Vec<bool>> {
        let buf = r.read_bytes(n)?;
        Ok(buf.into_iter().map(|b| !(b == 0)).collect())
    }

    // Parses a string table using one or two offset tables. It either
    // returns an error, or a vector with the same size and shape as
    // `offsets`, so that `offsets.len()` calls to `pop()` on the
    // return value will always succeed.
    fn read_strs(
        offsets: &[&[u16]],
        table: &[u8],
    ) -> Result<Vec<Vec<Vec<u8>>>, DescError> {
        let mut strs = Vec::new();
        let mut start = 0;
        for &offs in offsets {
            let mut len = 0;
            let mut strings = Vec::new();
            for &pos in offs {
                let pos = pos as usize + start;
                if pos == 0xffff || pos == 0xfffe {
                    strings.push(Vec::new());
                } else if pos >= table.len() {
                    return Err(parse_error("invalid string offset"));
                } else {
                    match table[pos..].iter().position(|&b| b == 0) {
                        None => {
                            return Err(parse_error("unterminated string"));
                        }
                        Some(end) => {
                            len += end + 1;
                            strings.push(table[pos..pos + end].to_vec());
                        }
                    }
                }
            }
            start += len;
            strs.push(strings);
        }
        Ok(strs)
    }

    /// The terminal's names.
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Query a user-defined boolean capability.
    ///
    /// If the capability is absent, returns `false`.
    pub fn get_bool_ext(&self, name: &UserDef) -> bool {
        for ecap in self.ext.iter().rev() {
            match *ecap {
                ICap::Bool(CapName::U(ref n), v) if n == name => {
                    return v;
                }
                _ => (),
            }
        }
        false
    }

    /// Query a user-defined numeric capability.
    ///
    /// If the capability is absent, returns `0xffff`.
    pub fn get_num_ext(&self, name: &UserDef) -> u16 {
        for ecap in self.ext.iter().rev() {
            match *ecap {
                ICap::Num(CapName::U(ref n), v) if n == name => {
                    return v;
                }
                _ => (),
            }
        }
        0xffff
    }

    /// Query a user-defined string capability.
    ///
    /// If the capability is absent, returns an empty slice.
    pub fn get_str_ext(&self, name: &UserDef) -> &[u8] {
        for ecap in self.ext.iter().rev() {
            match *ecap {
                ICap::Str(CapName::U(ref n), ref v) if n == name => {
                    return v;
                }
                _ => (),
            }
        }
        &[]
    }

    fn update(&mut self, caps: &[Cap]) {
        fn add_val<T: Default>(vs: &mut Vec<T>, idx: usize, val: T) {
            if idx >= vs.len() {
                for _ in 0..(idx - vs.len()) {
                    vs.push(Default::default());
                }
                vs.push(val);
            } else {
                vs[idx] = val;
            }
        }

        use self::ICap::*;
        for cap in caps {
            match cap.0 {
                Bool(CapName::P(idx), v) => {
                    add_val(&mut self.bools, idx, v);
                }
                Num(CapName::P(idx), v) => {
                    add_val(&mut self.nums, idx, v);
                }
                Str(CapName::P(idx), ref v) => {
                    add_val(&mut self.strings, idx, v.to_vec());
                }
                ref udc => self.ext.push(udc.clone()),
            }
        }
    }

    // Only public for use in the `desc!` macro.
    #[doc(hidden)]
    pub fn from_literal(names: &[String], caps: &[Cap]) -> Desc {
        let mut desc = Desc {
            names: Vec::from(names),
            bools: Vec::new(),
            nums: Vec::new(),
            strings: Vec::new(),
            ext: Vec::new(),
        };

        desc.update(caps);
        desc
    }
}


/// A syntax for [`Desc`](struct.Desc.html) literals.
///
/// # Examples
///
/// A description for a "dumb terminal":
///
/// ```
/// #[macro_use]
/// extern crate tinf;
/// # fn main() {
/// use tinf::cap::*;
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
/// A terminal with user-defined capabilities:
///
/// ```
/// #[macro_use]
/// # extern crate tinf;
/// # fn main() {
/// use tinf::cap::UserDef;
/// let desc = desc![
///     // tmux can use this to indicate TrueColor support
///     UserDef::named("Tc") => true,
///
///     // emacs can use these for TrueColor
///     UserDef::named("setb24") => "\x1b[48;2;\
///                                  %p1%{65536}%/%d;\
///                                  %p1%{256}%/%{255}%&%d;\
///                                  %p1%{255}%&%dm",
///     UserDef::named("setf24") => "\x1b[38;2;\
///                                  %p1%{65536}%/%d;\
///                                  %p1%{256}%/%{255}%&%d;\
///                                  %p1%{255}%&%dm"
/// ];
/// # }
/// ```
///
/// Terminal names are specified by strings; predefined capabilities
/// are specified by `name => val` pairs, where `name` is a
/// [`Boolean`](../tinf/cap/struct.Boolean.html),
/// [`Number`](../tinf/cap/struct.Number.html), or
/// [`String`](../tinf/cap/struct.String.html) capability name,
/// and `val` is a `bool`, `u16`, or `AsRef<[u8]>` respectively; for
/// user-defined capabilities, `name` must be `Borrow<str>` and `val`
/// can be a `bool`, a `u16`, or a `&'static str`.
#[macro_export]
macro_rules! desc {
    // Finish processing.
    (@acc [$($ns:expr),*] [$($ps:expr),*]) => {
        $crate::Desc::from_literal(
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
        desc!(@acc [$($ns),*] [$($ps,)* ($k, $v).into()]);
    };
    (@acc [$($ns:expr),*] [$($ps:expr),*] $k:expr => $v:expr, $($xs:tt)*) => {
        desc!(@acc [$($ns),*] [$($ps,)* ($k, $v).into()] $($xs)*);
    };

    // Start, with a name.
    ($n:expr) => { desc!(@acc [$n] []); };
    ($n:expr, $($xs:tt)*) => { desc!(@acc [$n] [] $($xs)*); };

    // Start, with a cap => val pair.
    ($k:expr => $v:expr) => { desc!(@acc [] [($k, $v).into()]); };
    ($k:expr => $v:expr, $($xs:tt)*) => {
        desc!(@acc [] [($k, $v).into()] $($xs)*);
    };
}


// The terminfo binary format contains padding bytes as necessary to
// keep u16 data chunks at word-aligned file offsets. `AlignReader`
// handles this padding (as well as endianness) when reading bytes and
// words from a compiled description.
struct AlignReader<'a> {
    r: &'a mut Read,
    n: usize,
}

impl<'a> AlignReader<'a> {
    fn new(r: &mut Read) -> AlignReader {
        AlignReader { r, n: 0 }
    }

    fn align(&mut self) -> io::Result<()> {
        if self.n % 2 != 0 {
            self.read_bytes(1)?;
        }
        Ok(())
    }

    fn read_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(n);
        if self.r.take(n as u64).read_to_end(&mut buf)? < n {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }
        self.n += n;
        Ok(buf)
    }

    fn read_words(&mut self, n: usize) -> io::Result<Vec<u16>> {
        self.align()?;
        let mut buf_16: Vec<u16> = vec![0; n];
        let mut buf_8 = unsafe {
            ::std::slice::from_raw_parts_mut(
                buf_16.as_mut_ptr() as *mut u8,
                buf_16.len() * 2,
            )
        };
        self.r.read_exact(&mut buf_8)?;
        if cfg!(target_endian = "big") {
            for w in &mut buf_16 {
                *w = w.swap_bytes();
            }
        }
        self.n += n * 2;
        Ok(buf_16)
    }
}

// env::var distinguishes between empty and unset; `to_path` and
// `to_paths` treat them as the same for compatibility with ncurses.

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


lazy_static! {
    static ref CURRENT: Desc = {
        env::var("TERM")
            .ok()
            .and_then(|t| if t.is_empty() { None } else { Some(t) })
            .and_then(|t| Desc::file(&t).ok())
            .and_then(|mut f| Desc::parse(&mut f).ok())
            .unwrap_or(
                desc![
                    "dumb", "80-column dumb tty",
                    cap::am => true,
                    cap::cols => 80,
                    cap::bel => "\x07",
                    cap::cr => "\r",
                    cap::cud1 => "\n",
                    cap::ind => "\n",
                ]
            )
    };

    static ref FS_ROOT: PathBuf = {
        if cfg!(target_os = "windows") {
            let paths = to_paths(env::var("PATH").ok());
            for p in paths {
                let p = p.display().to_string();
                if p.ends_with("\\usr\\local\\bin") {
                    let (p1, _) = p.split_at(p.len() - 14);
                    return PathBuf::from(p1);
                }
            }
            PathBuf::new()
        } else {
            PathBuf::from("/")
        }
    };
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
            Io(ref err) => Some(err),
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

#[cfg(test)]
mod tests;

// Utility trait. `.expected("foo")` means that ... you expected
// "foo", and you panic if you didn't get it.
trait Expectation<T> {
    fn expected(self, msg: &str) -> T;
}

impl<T> Expectation<T> for Option<T> {
    fn expected(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            _ => expectation_failed(msg),
        }
    }
}

impl<T, E> Expectation<T> for Result<T, E> {
    fn expected(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            _ => expectation_failed(msg),
        }
    }
}

#[inline(never)]
#[cold]
fn expectation_failed(msg: &str) -> ! {
    panic!("expected {}", msg)
}
