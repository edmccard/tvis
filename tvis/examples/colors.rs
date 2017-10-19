extern crate tvis;

use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use tvis::Coords;
use tvis::term::{self, BoldOrBright, Color, Style, Terminal, UseTruecolor};
use tvis::input::{ButtonMotion, Event, InputEvent, Key, MouseButton};

fn main() {
    let (tx, rx) = channel();
    let mut screen = term::connect_with_input(
        tx.clone(),
        UseTruecolor::Auto,
        BoldOrBright::Bright,
    ).unwrap();
    if !screen.is_tty_input() || !screen.is_tty_output() {
        screen.log("input or output is not a terminal");
        return;
    }

    let mut demo = ColorDemo::new(screen);
    demo.event_loop(tx, rx);
}

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq)]
enum DemoType {
    P16,
    P256,
    True,
}

#[derive(Debug)]
struct Tick {}

impl Event for Tick {
    fn as_any(&self) -> &Any {
        self
    }
}

struct ColorDemo {
    ty: DemoType,
    max: DemoType,
    offset: usize,
    screen: Box<Terminal>,
}

impl ColorDemo {
    fn new(screen: Box<Terminal>) -> ColorDemo {
        let mut demo = ColorDemo {
            ty: DemoType::P16,
            max: match screen.max_colors() {
                (_, _, true) => DemoType::True,
                (256, _, _) => DemoType::P256,
                _ => DemoType::P16,
            },
            offset: 0,
            screen,
        };
        demo.init();
        demo
    }

    fn init(&mut self) {
        self.screen.clear().unwrap();
        self.screen.cursor_visible(false).unwrap();
        self.paint();
        self.screen.start_input().unwrap();
    }

    fn repaint(&mut self) {
        self.screen.clear().unwrap();
        self.paint();
    }

    fn paint(&mut self) {
        self.draw_menu();
        match self.ty {
            DemoType::P16 => self.draw_16(),
            DemoType::P256 => self.draw_256(),
            DemoType::True => self.draw_true(),
        }
        self.screen
            .set_style(Style::empty(), Color::Default, Color::Default)
            .unwrap();
        self.draw_help();
        self.screen.flush_output().unwrap();
    }

    fn draw_16(&mut self) {
        for line in 0..16 {
            self.screen.set_cursor((5, line + 4)).unwrap();
            let c = ((line as usize + (self.offset / 2)) % 16) as u16;
            self.screen
                .set_style(
                    Style::empty(),
                    Color::Default,
                    Color::Palette(c as u8),
                )
                .unwrap();
            self.screen.write("          ").unwrap();
        }
    }

    fn draw_256(&mut self) {
        for c1 in 0..6 {
            for c2 in 0..6 {
                for c3 in 0..6 {
                    let x = (c1 * 6) + c3 + 5;
                    let y = c2 + 4;
                    self.screen.set_cursor((x, y)).unwrap();
                    let c3 = ((c3 as usize + (self.offset / 2)) % 6) as u16;
                    let c = c1 * 36 + c2 * 6 + c3 + 16;
                    self.screen
                        .set_style(
                            Style::empty(),
                            Color::Default,
                            Color::Palette(c as u8),
                        )
                        .unwrap();
                    self.screen.write(" ").unwrap();
                }
            }
        }
    }

    fn draw_true(&mut self) {
        let mut line = 4;
        // .step_by is nightly only, so...
        let mut green = 0u16;
        loop {
            if green > 255 {
                break;
            }
            self.screen.set_cursor((5, line)).unwrap();

            let red_idx = self.offset % 22;
            let red = 25 * if red_idx > 10 { 21 - red_idx } else { red_idx };
            let mut blue_ = 0.0;
            loop {
                let blue = blue_ as u16;
                if blue > 255 {
                    break;
                }
                self.screen
                    .set_style(
                        Style::empty(),
                        Color::Default,
                        Color::TrueColor(red as u8, blue as u8, green as u8),
                    )
                    .unwrap();
                self.screen.write(" ").unwrap();
                blue_ += 8.8;
            }

            green += 15;
            line += 1;
        }
    }

    fn draw_menu(&mut self) {
        let ty = self.ty;
        self.draw_button(0, "16", ty == DemoType::P16);
        if self.max > DemoType::P16 {
            self.draw_button(5, "256", ty == DemoType::P256);
        }
        if self.max > DemoType::P256 {
            self.draw_button(11, "Truecolor", ty == DemoType::True);
        }
    }

    fn draw_button(&mut self, x: u16, label: &str, selected: bool) {
        let mut edge = String::with_capacity(label.len());
        let mut top = String::with_capacity(label.len() + 2);
        let mut bot = String::with_capacity(label.len() + 2);
        let mut vert = String::with_capacity(1);
        if selected {
            edge.push_str(&"═".repeat(label.len()));
            top.push_str(&["╔", &edge, "╗"].concat());
            vert.push('║');
            bot.push_str(&["╚", &edge, "╝"].concat());
        } else {
            edge.push_str(&"─".repeat(label.len()));
            top.push_str(&["┌", &edge, "┐"].concat());
            vert.push('│');
            bot.push_str(&["└", &edge, "┘"].concat());
        }

        self.screen.set_cursor((x, 0)).unwrap();
        self.screen.write(&top).unwrap();

        self.screen.set_cursor((x, 1)).unwrap();
        self.screen.write(&[&vert, label, &vert].concat()).unwrap();
        self.screen.set_cursor((x, 2)).unwrap();
        self.screen.write(&bot).unwrap();
    }

    fn draw_help(&mut self) {
        let size = self.screen.get_size().unwrap();
        if self.max > DemoType::P16 {
            let mut msg = String::from("");
            msg.push_str("Select a mode with the mouse or press 1");
            if self.max == DemoType::P256 {
                msg.push_str(" or 2.");
            } else {
                msg.push_str(", 2, or t.");
            }
            self.screen.set_cursor((0, size.rows - 2)).unwrap();
            self.screen.write(&msg).unwrap();
        }
        self.screen.set_cursor((0, size.rows - 1)).unwrap();
        self.screen.write("Press Esc to exit").unwrap();
    }

    fn handle_key(&mut self, key: char) {
        match key {
            '1' => self.to_p16(),
            '2' => self.to_p256(),
            't' | 'T' => self.to_true(),
            _ => (),
        }
    }

    fn handle_click(&mut self, coords: Coords) {
        if coords.1 > 2 {
            return;
        }
        if coords.0 < 4 {
            self.to_p16();
        } else if coords.0 > 4 && coords.0 < 10 {
            self.to_p256();
        } else if coords.0 > 10 && coords.0 < 22 {
            self.to_true();
        }
    }

    fn to_p16(&mut self) {
        if self.ty != DemoType::P16 {
            self.ty = DemoType::P16;
            self.repaint();
        }
    }

    fn to_p256(&mut self) {
        if self.max > DemoType::P16 && self.ty != DemoType::P256 {
            self.ty = DemoType::P256;
            self.repaint();
        }
    }

    fn to_true(&mut self) {
        if self.max == DemoType::True && self.ty != DemoType::True {
            self.ty = DemoType::True;
            self.repaint();
        }
    }

    fn event_loop(&mut self, tx: Sender<Box<Event>>, rx: Receiver<Box<Event>>) {
        use std::{thread, time};
        let tick_duration = time::Duration::from_millis(100);
        thread::spawn(move || loop {
            thread::sleep(tick_duration);
            let _ = tx.send(Box::new(Tick {}));
        });

        for evt in rx.iter() {
            if let Some(evt) = evt.as_any().downcast_ref::<InputEvent>() {
                match *evt {
                    InputEvent::Key(key, _) => match key {
                        Key::Esc => return,
                        Key::Char(c, _, _) => self.handle_key(c),
                        _ => (),
                    },
                    InputEvent::Mouse(
                        ButtonMotion::Press,
                        MouseButton::Left,
                        _,
                        coords,
                    ) => self.handle_click(coords),
                    InputEvent::Repaint => self.repaint(),
                    _ => (),
                }
            }
            if let Some(_) = evt.as_any().downcast_ref::<Tick>() {
                self.offset += 1;
                self.paint();
            }
        }
    }
}
