extern crate tinf;

use tinf::{Desc, cap};

fn main() {
    let desc = Desc::current();
    let names = desc.names();
    if names.len() > 0 {
        println!("{}", names[0]);
    }
    for b in cap::Boolean::iter() {
        if desc[b] {
            println!("{}", b.short_name());
        }
    }
    for n in cap::Number::iter() {
        if desc[n] != 0xffff {
            println!("{}#{}", n.short_name(), desc[n]);
        }
    }
    for s in cap::String::iter() {
        if &desc[s] != b"" {
            println!("{}={}", s.short_name(), show(&desc[s]));
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
