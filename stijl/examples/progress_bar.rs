#![allow(unused_must_use)]

extern crate stijl;
use stijl::*;

use std::{thread, time};
use std::io::{self, Cursor, Read, Write};

fn main() {
    let quarter_sec = time::Duration::from_millis(250);
    let mut completion = 0.0f32;
    while completion <= 1.0 {
        draw_progress(completion);
        thread::sleep(quarter_sec);
        completion += 0.03125 // 1/32
    }
    println!();
}

fn draw_progress(completion: f32) {
    let mut stream = stijl::stdout(DoStyle::Auto);
    let sz = stream.get_size();
    // Use (cols - 1) for Windows 7 compatibility.
    stream.write_all(&progress_str((sz.cols - 1) as usize, completion));
    stream.rewind_lines(1);
}

fn progress_str(cols: usize, completion: f32) -> Vec<u8> {
    let mut pstr = Cursor::new(Vec::with_capacity(cols));
    // minimum updatable: "Processing: 100%"
    if cols < 16 {
        write!(
            &mut pstr,
            "{}",
            "Processing...".chars().take(cols).collect::<String>()
        );
        return pstr.into_inner();
    }
    write!(&mut pstr, "Processing:{}", if cols > 16 { " " } else { "" });
    if cols > 17 {
        let cols = (cols - 17) as u64;
        let fill = (completion * (cols as f32)) as u64;
        io::copy(&mut io::repeat(b'#').take(fill), &mut pstr);
        io::copy(&mut io::repeat(b' ').take(cols - fill), &mut pstr);
    }
    write!(&mut pstr, "{: >4}%", (completion * 100.0) as u32);
    pstr.into_inner()
}
