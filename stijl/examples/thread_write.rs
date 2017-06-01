#![allow(unused_must_use)]

extern crate stijl;
extern crate tinf;

use std::io::Write;
use std::thread;
use std::time;
use stijl::*;

// Spawns three threads and writes various styled messages that should
// not interleave.
fn main() {
    let t1 = thread::spawn(
        || {
            let five_millis = time::Duration::from_millis(5);
            let stream = stijl::stdout(DoStyle::Auto);
            let mut stream = stream.lock();
            stream.fg(Blue);
            write!(stream, "ONE ");
            thread::sleep(five_millis);
            write!(stream, "TWO ");
            thread::sleep(five_millis);
            writeln!(stream, "THREE");
            stream.reset();
        }
    );
    let t2 = thread::spawn(
        || {
            let five_millis = time::Duration::from_millis(5);
            let stream = stijl::stdout(DoStyle::Auto);
            let mut stream = stream.lock();
            stream.fg(Green);
            write!(stream, "1 ");
            thread::sleep(five_millis);
            write!(stream, "2 ");
            thread::sleep(five_millis);
            writeln!(stream, "3");
            stream.reset();
        }
    );
    let t3 = thread::spawn(
        || {
            let five_millis = time::Duration::from_millis(5);
            let stream = stijl::stdout(DoStyle::Auto);
            let mut stream = stream.lock();
            stream.fg(Red);
            write!(stream, "I ");
            thread::sleep(five_millis);
            write!(stream, "II ");
            thread::sleep(five_millis);
            writeln!(stream, "III");
            stream.reset();
        }
    );

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
