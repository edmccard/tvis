extern crate tvis;

use std::sync::mpsc::channel;

use tvis::term::{self, BoldOrBright, UseTruecolor};
use tvis::input::{InputEvent, Key};

fn main() {
    let (tx, rx) = channel();
    let mut screen =
        term::connect_with_input(tx, UseTruecolor::Auto, BoldOrBright::Bold)
            .unwrap();
    if !screen.is_tty_input() || !screen.is_tty_output() {
        screen.log("input or output is not a terminal");
        return;
    }
    screen.start_input().unwrap();

    for evt in rx.iter() {
        if let Some(evt) = evt.as_any().downcast_ref::<InputEvent>() {
            match *evt {
                InputEvent::Key(Key::Char('`', _, _), _) => return,
                InputEvent::Key(k, m) => {
                    screen.log(&format!("KEY {}{}\r", m, k,));
                }
                _ => {
                    screen.log(&format!("EVENT: {:?}\r", evt));
                }
            }
        }
    }
    screen.log("SHUTTING DOWN\r");
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
}
