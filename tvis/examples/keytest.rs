extern crate tvis;

use std::sync::mpsc::channel;

use tvis::term;
use tvis::input::{InputEvent, Key};

fn main() {
    let (tx, rx) = channel();
    let mut screen = match term::connect_with_input(tx) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };
    match screen.start_input() {
        Ok(()) => (),
        Err(e) => {
            screen.log(&format!("ERROR: {}", e));
            return;
        }
    }
    let _ = &screen;

    let size = screen.get_size().unwrap();
    screen.log(&format!("SIZE: {} x {}", size.cols, size.rows));

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
}
