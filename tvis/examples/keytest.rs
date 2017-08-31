extern crate tinf;
extern crate tvis;
extern crate tvis_util;

use std::sync::mpsc::{channel, Sender};

#[cfg(windows)]
use tvis::screen::ConsoleScreen;
#[cfg(not(windows))]
use tvis::screen::TerminalScreen;

use tvis::{Event, InputEvent, Key, Result, Screen};

#[cfg(windows)]
fn init(tx: Sender<Box<Event>>) -> Result<Box<Screen>> {
    ConsoleScreen::init(tx)
}

#[cfg(not(windows))]
fn init(tx: Sender<Box<Event>>) -> Result<Box<Screen>> {
    TerminalScreen::init(tx)
}

fn main() {
    let (tx, rx) = channel();
    let screen = match init(tx) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };
    let _ = &screen;

    for evt in rx.iter() {
        if let Some(evt) = evt.as_any().downcast_ref::<InputEvent>() {
            match *evt {
                InputEvent::Interrupt => {
                    screen.log("BYE!");
                    break;
                }
                InputEvent::Key(k, m) => {
                    if k == Key::Char([96, 0, 0, 0], 1) {
                        screen.log("BYE!");
                        return;
                    }
                    screen.log(&format!("KEY {}{}\r", m, k,));
                }
                _ => screen.log(&format!("EVENT: {:?}\r", evt)),
            }
        }
    }

    // should make sure not redirected or cygwin
    // loop {
    //     for i in 0..read_count as usize {
    //         match buffer[i].event_type {
    //             win32::KEY_EVENT => {
    //                 // alt-key and ctrl-key can be either uchar 0 or not
    //                 for j in 0..key.repeat_count {
    //                     println!(
    //                         "{} code char: {} {}",
    //                         j,
    //                         key.virtual_key_code,
    //                         key.uchar
    //                     );
    //                 }
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    // }
}
