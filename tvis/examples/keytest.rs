extern crate tvis;

use std::sync::mpsc::channel;

use tvis::term::Term;
use tvis::input::{InputEvent, Key};

fn main() {
    let (tx, rx) = channel();
    let screen = match Term::init(tx) {
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
    screen.log("SHUTTING DOWN\r");
    ::std::thread::sleep(::std::time::Duration::from_secs(3));

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
