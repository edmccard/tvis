//! Simple styled text streams.
//!
//! # Basic usage
//!
//! [`stdout()`](fn.stdout.html) and [`stderr()`](fn.stderr.html) are
//! drop-in replacements for `std::io::stdout()` and
//! `std::io::stderr()`, which wrap them in a
//! [`Stream`](trait.Stream.html) that supports eight foreground
//! colors and emphasized text.
//!
//! ### Example
//!
//! ```rust
//! use std::io::Write;
//! use stijl::{Stream, DoStyle, Red};
//!
//! # fn main() {
//! #     foo().unwrap();
//! # }
//! # fn foo() -> Result<(), Box<std::error::Error>> {
//! let str = &mut stijl::stdout(DoStyle::Auto);
//! str.fg(Red);
//! str.em();
//! write!(str, "Warning: ");
//! str.reset();
//! writeln!(str, " winter is coming.");
//! # Ok(())
//! # }
//! ```
//! # Platform notes
//!
//! [`stdout`](fn.stdout.html) and [`stderr`](fn.stderr.html) return a
//! `TermStream` object for terminals that understand terminfo-style
//! escape sequences, including the Cygwin and MSYS terminals, and the
//! Windows 10 console (Anniversary Update or newer). They return a
//! `ConStream` struct for consoles in earlier versions of Windows.
//!
//! # Multithreading
//!
//! The objects returned by [`stdout()`](fn.stdout.html) and
//! [`stderr()`](fn.stderr.html) implement
//! [`LockableStream`](trait.LockableStream.html).
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

#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#![cfg_attr(feature = "cargo-clippy", allow(match_bool))]

extern crate libc;
extern crate tvis_util;
#[macro_use]
extern crate tinf;
#[macro_use]
extern crate lazy_static;

use std::{error, fmt, io};
use tvis_util::Handle;
#[cfg(windows)]
use tvis_util::TerminalMode;

#[cfg(windows)]
mod win32;
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


/// Strategies for applying styles to standard output streams.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoStyle {
    /// Always apply styles.
    Always,
    /// Apply styles if stdout/stderr write to a terminal, but not if
    /// they are redirected.
    Auto,
    /// Never apply styles.
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
    fn em(&mut self) -> Result<()>;
    /// True if the stream is connected to a command-line interface.
    fn is_cli(&self) -> bool;
}

impl<'a> Stream for Box<Stream + 'a> {
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

/// A [`Stream`](trait.Stream.html) with synchronized access.
pub trait LockableStream: Stream {
    /// Lock the stream, returning a writable guard.
    fn lock<'a>(&'a self) -> Box<Stream + 'a>;
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

/// An error that occurred writing to a `Stream`.
#[derive(Debug)]
pub struct Error(());

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        // TODO
        Error(())
    }
}

impl From<::tinf::CapError> for Error {
    fn from(_: ::tinf::CapError) -> Error {
        // TODO
        Error(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO Error::fmt()")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "TODO Error::description()"
    }

    fn cause(&self) -> Option<&error::Error> {
        // TODO
        None
    }
}

/// Either success or failure.
pub type Result<T> = std::result::Result<T, Error>;


// Silences warning
lazy_static!{}
