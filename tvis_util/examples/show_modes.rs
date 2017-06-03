extern crate tvis_util;

use tvis_util::*;

#[cfg(windows)]
fn main() {
    console_modes();
    terminal_modes();
}

#[cfg(not(windows))]
fn main() {
    terminal_modes();
}

#[cfg(windows)]
fn console_modes() {
    println!("Console modes:");
    println!(" stdin: {:?}", Handle::Stdin.console_mode());
    println!("stdout: {:?}", Handle::Stdout.console_mode());
    println!("stderr: {:?}", Handle::Stderr.console_mode());
    println!("");
}

fn terminal_modes() {
    println!("Terminal modes:");
    println!(" stdin: {:?}", Handle::Stdin.terminal_mode());
    println!("stdout: {:?}", Handle::Stdout.terminal_mode());
    println!("stderr: {:?}", Handle::Stderr.terminal_mode());
}
