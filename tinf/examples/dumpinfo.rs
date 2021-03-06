extern crate tinf;

use tinf::{cap, Desc};

fn main() {
    let desc = Desc::current();
    let names = desc.names();
    if !names.is_empty() {
        println!("{}", names[0]);
    }
    for b in cap::Boolean::iter() {
        if desc[b] {
            println!("{}", b.short_name());
        }
    }
    for b in desc.bool_exts() {
        if desc.get_bool_ext(b) {
            println!("* {}", b.name());
        }
    }
    for n in cap::Number::iter() {
        if desc[n] != 0xffff {
            println!("{}#{}", n.short_name(), desc[n]);
        }
    }
    for n in desc.num_exts() {
        if desc.get_num_ext(n) != 0xffff {
            println!("* {}#{}", n.name(), desc.get_num_ext(n));
        }
    }
    for s in cap::String::iter() {
        if &desc[s] != b"" {
            println!("{}={}", s.short_name(), show(&desc[s]));
        }
    }
    for s in desc.str_exts() {
        if desc.get_str_ext(s) != b"" {
            println!("* {}={}", s.name(), show(desc.get_str_ext(s)));
        }
    }
}

fn show(val: &[u8]) -> String {
    let mut s = String::new();
    for &b in val {
        if b == 27 {
            s.push_str("\\E");
            continue;
        }
        if b < 32 {
            s.push('^');
            s.push((b + 64) as char);
            continue;
        }
        if b > 126 {
            s.push('\\');
            s.push_str(&format!("{:03o}", b));
            continue;
        }
        s.push(b as char);
    }
    s
}
