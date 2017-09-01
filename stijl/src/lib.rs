//! Simple styled text streams.
//!
//! # Basic usage
//!
//! [`stdout()`](fn.stdout.html) and [`stderr()`](fn.stderr.html) are
//! drop-in replacements for `std::io::stdout()` and
//! `std::io::stderr()`, which wrap them in a
//! [`CLIStream`](trait.CLIStream.html) that supports eight foreground
//! colors and emphasized text.
//!
//! ### Example
//!
//! ```rust
//! use std::io::Write;
//! use stijl::{CLIStream, DoStyle, Red};
//!
//! # fn main() {
//! #     foo().unwrap();
//! # }
//! # fn foo() -> Result<(), Box<std::error::Error>> {
//! let stream = &mut stijl::stdout(DoStyle::Auto);
//! stream.fg(Red);
//! stream.em();
//! write!(stream, "Warning: ");
//! stream.reset();
//! writeln!(stream, " this text is red.");
//! # Ok(())
//! # }
//! ```
//!
//! # Animations
//!
//! The [`get_size`](trait.CLIStream.html#tymethod.rewind_lines) and
//! [`rewind_lines`](trait.CLIStream.html#tymethod.rewind_lines)
//! methods of [`CLIStream`](trait.CLIStream.html) can help make
//! progress bars, spinners, and other simple animations.
//!
//! ### Example
//!
//! ```no_run
//! use stijl::{CLIStream, DoStyle};
//! use std::{time, thread};
//!
//! # fn main() {
//! #     foo().unwrap();
//! # }
//! # use std::io::Write;
//! # fn draw_indicator(w: &mut Write, pos: usize, max: usize) {}
//! # fn foo() -> Result<(), Box<std::error::Error>> {
//! let delay = time::Duration::from_millis(100);
//! let stream = &mut stijl::stdout(DoStyle::Auto);
//! let max = stream.get_size().cols as usize;
//! let mut pos = 0;
//! for _ in 0..1000 {
//!     // draw_indicator is left as an exercise for the reader
//!     draw_indicator(stream, pos, max);
//!     thread::sleep(delay);
//!     if max != 0 {
//!         pos = (pos + 1) % max;
//!     }
//!     stream.rewind_lines(1);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Multithreading
//!
//! The objects returned by [`stdout()`](fn.stdout.html) and
//! [`stderr()`](fn.stderr.html) implement
//! [`LockableStream`](trait.LockableStream.html), with a
//! [`lock`](trait.LockableStream.html#tymethod.lock) method for
//! synchronizing the stream.
//!
//! To reduce contention, multiple threads can write to their own
//! [`BufStream`](struct.BufStream.html) objects and when finished,
//! print them to a `LockableStream`.
//!
//! ### Example
//!
//! ```rust
//! use stijl::{BufStream, DoStyle};
//!
//! # fn main() {
//! #     foo().unwrap();
//! # }
//! # fn foo() -> Result<(), Box<std::error::Error>> {
//! let mut buf = BufStream::new();
//! // ...
//! // Do work
//! // Write to buf
//! // ...
//! let stream = &mut stijl::stdout(DoStyle::Auto);
//! let stream = &mut stream.lock();
//! buf.playback(stream)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Platform notes
//!
//! [`stdout`](fn.stdout.html) and [`stderr`](fn.stderr.html) return a
//! `TermStream` object for terminals that understand terminfo-style
//! escape sequences, including the Cygwin and MSYS terminals, and the
//! Windows 10 console (Anniversary Update or newer). They return a
//! `ConStream` struct for consoles in earlier versions of Windows.
//!
//! On Windows, the same binary will produce equivalent styled output
//! in either the console or a Cygwin terminal. However, in Cygwin,
//! [`CLIStream::get_size()`](trait.CLIStream.html#tymethod.get_size)
//! currently always returns the default size (80x24).

#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#![cfg_attr(feature = "cargo-clippy", allow(match_bool))]

extern crate libc;
extern crate tvis_util;
#[macro_use]
extern crate tinf;
#[cfg(windows)]
#[macro_use]
extern crate lazy_static;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

use std::{error, fmt, io};
use tvis_util::Handle;
pub use tvis_util::TerminalMode;
pub use tvis_util::size::WinSize;

#[cfg(windows)]
mod console;
#[cfg(windows)]
pub use self::console::ConStream;

mod buf;
pub use self::buf::BufStream;
mod term;
pub use self::term::TermStream;


/// A [`LockableStream`](trait.LockableStream.html) wrapping `stdout`.
pub fn stdout(do_style: DoStyle) -> Box<LockableStream> {
    match Handle::Stdout.terminal_mode() {
        #[cfg(windows)]
        TerminalMode::Console => Box::new(ConStream::stdout(do_style)),
        mode => Box::new(TermStream::std(mode, io::stdout(), do_style)),
    }
}

/// A [`LockableStream`](trait.LockableStream.html) wrapping `stderr`.
pub fn stderr(do_style: DoStyle) -> Box<LockableStream> {
    match Handle::Stderr.terminal_mode() {
        #[cfg(windows)]
        TerminalMode::Console => Box::new(ConStream::stderr(do_style)),
        mode => Box::new(TermStream::std(mode, io::stderr(), do_style)),
    }
}


/// Strategies for applying styles to text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoStyle {
    /// Always apply style.
    Always,
    /// Apply style if stdout/stderr write to a terminal, but not if
    /// they are redirected.
    Auto,
    /// Never apply style.
    Never,
}

/// A terminal color.
///
/// Values represent indexes in a terminal palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(i32, u16);

/// Color 0.
pub const Black: Color = Color(0, 0);
/// Color 1 (color 4 in Windows console)
pub const Red: Color = Color(1, 4);
/// Color 2
pub const Green: Color = Color(2, 2);
/// Color 3 (color 6 in Windows console)
pub const Yellow: Color = Color(3, 6);
/// Color 4 (color 1 in Windows console)
pub const Blue: Color = Color(4, 1);
/// Color 5
pub const Magenta: Color = Color(5, 5);
/// Color 6 (color 3 in Windows console)
pub const Cyan: Color = Color(6, 3);
/// Color 7
pub const White: Color = Color(7, 7);


/// An output stream with simple styling.
pub trait Stream: io::Write {
    /// Return color and emphasis to the default.
    fn reset(&mut self) -> Result<()>;
    /// Change the foreground color.
    fn fg(&mut self, fg: Color) -> Result<()>;
    /// Begin emphasized text.
    ///
    /// Emphasis stays in effect until `reset()` is called.
    fn em(&mut self) -> Result<()>;
    /// True if the stream is connected to a command-line interface.
    fn is_cli(&self) -> bool;
}

/// A [`Stream`](trait.CLIStream.html) connected to a command-line
/// interface.
pub trait CLIStream: Stream {
    /// "Rewind" the stream so that the last `count` lines can be
    /// overwritten.
    ///
    /// Panics if the `Stream` is not connected to a command-line
    /// window, that is, if
    /// [`Stream::is_cli()`](trait.Stream.html#tymethod.is_cli) would
    /// return false.
    fn rewind_lines(&mut self, count: u16) -> Result<()>;
    /// The size of the command-line window.
    ///
    /// Panics if the `Stream` is not connected to a command-line
    /// window, that is, if
    /// [`Stream::is_cli()`](trait.Stream.html#tymethod.is_cli) would
    /// return false.
    fn get_size(&self) -> WinSize;
}

impl<'a> Stream for Box<CLIStream + 'a> {
    fn reset(&mut self) -> Result<()> {
        (**self).reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        (**self).fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        (**self).em()
    }

    fn is_cli(&self) -> bool {
        (**self).is_cli()
    }
}

impl<'a> CLIStream for Box<CLIStream + 'a> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        (**self).rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        (**self).get_size()
    }
}

/// A [`CLIStream`](trait.CLIStream.html) that can be locked for
/// synchronized access.
pub trait LockableStream: CLIStream {
    /// Lock the stream, returning a writable guard.
    fn lock<'a>(&'a self) -> Box<CLIStream + 'a>;
}

impl Stream for Box<LockableStream> {
    fn reset(&mut self) -> Result<()> {
        (**self).reset()
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        (**self).fg(fg)
    }

    fn em(&mut self) -> Result<()> {
        (**self).em()
    }

    fn is_cli(&self) -> bool {
        (**self).is_cli()
    }
}

impl CLIStream for Box<LockableStream> {
    fn rewind_lines(&mut self, count: u16) -> Result<()> {
        (**self).rewind_lines(count)
    }

    fn get_size(&self) -> WinSize {
        (**self).get_size()
    }
}

/// An error that occurred writing to a `Stream`.
#[derive(Debug)]
pub struct Error {
    inner: ErrorImpl,
}

#[derive(Debug)]
enum ErrorImpl {
    Io(io::Error),
    Cap(::tinf::CapError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error { inner: ErrorImpl::Io(err) }
    }
}

impl From<::tinf::CapError> for Error {
    fn from(err: ::tinf::CapError) -> Error {
        Error { inner: ErrorImpl::Cap(err) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ErrorImpl::Io(ref err) => err.fmt(f),
            ErrorImpl::Cap(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.inner {
            ErrorImpl::Io(ref err) => err.description(),
            ErrorImpl::Cap(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.inner {
            ErrorImpl::Io(ref err) => Some(err),
            ErrorImpl::Cap(ref err) => Some(err),
        }
    }
}

/// Either success or failure.
pub type Result<T> = std::result::Result<T, Error>;
