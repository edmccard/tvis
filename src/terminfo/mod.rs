//! An interface to terminfo databases.
//!
//! # Usage
//!
//! For loading terminal descriptions, see [`Desc`](struct.Desc.html);
//! for sending commands to the terminal, see [`tparm`](fn.tparm.html)
//! and [`tputs`](fn.tputs.html).
//!
//! ## Platform Compatibility
//!
//! This requires the local terminfo database to be in the "directory
//! tree" format; it will not work with the "hashed database" format.
//! In other words, it should Just Work on Linux/OSX/Cygwin, but it
//! might not work out of the box on BSD operating systems.

pub mod cap;
#[macro_use] mod desc;
#[macro_use] mod print;

pub use self::desc::{Desc, DescError};
pub use self::print::{CapError, Param, Vars, tparm, tputs};

#[cfg(test)]
mod tests;
