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
    let _ = &screen;

    let size = screen.get_size().unwrap();
    screen.log(&format!("SIZE: {} x {}", size.cols, size.rows));
    let colors = screen.max_colors().0;
    screen.log(&format!("COLORS: {}", colors));
    let styles = screen.supported_styles();
    screen.log(&format!("STYLES: {:?}", styles));

    for evt in rx.iter() {
        if let Some(evt) = evt.as_any().downcast_ref::<InputEvent>() {
            match *evt {
                InputEvent::Key(k, m) => {
                    if k == Key::Char([96, 0, 0, 0], 1) {
                        let _ = screen.write("BYE!");
                        let _ = screen.write("\r\n");
                        return;
                    }
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
