pub struct Formatter {
    width: u32,
    prec: i32,
    align: Align,
    alt: bool,
    space: bool,
    spec: bool,
}

#[derive(PartialEq)]
#[repr(u8)]
enum Align {
    None = 0,
    LeftJust = 1,
    ZeroPad = 2,
}

impl Default for Formatter {
    fn default() -> Formatter {
        Formatter {
            width: 0,
            prec: -1,
            align: Align::None,
            alt: false,
            space: false,
            spec: false,
        }
    }
}

impl Formatter {
    pub fn add_flag(&mut self, flag: char) {
        use self::Align::*;
        self.spec = true;
        match flag {
            ':' => (),
            '#' => self.alt = true,
            ' ' => self.space = true,
            '-' => self.align = LeftJust,
            '0' => {
                if self.align != LeftJust {
                    self.align = ZeroPad;
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.spec = true;
    }

    pub fn set_prec(&mut self, prec: u32) {
        self.prec = prec as i32;
        self.spec = true;
    }

    pub fn specified(&self) -> bool {
        self.spec
    }

    pub fn printf_int(&self, w: &mut Vec<u8>, fs: char, val: i32) {
        if self.prec == 0 && val == 0 {
            return;
        }
        let mut output: Vec<u8> = Vec::new();
        if self.alt && (fs == 'o' || fs == 'x' || fs == 'X') {
            output.push(b'0');
            if fs == 'x' {
                output.push(b'x');
            } else if fs == 'X' {
                output.push(b'X');
            }
        }
        let num = match fs {
            'd' => format!("{}", val),
            'o' => format!("{:o}", val),
            'x' => format!("{:x}", val),
            'X' => format!("{:X}", val),
            _ => unreachable!(),
        }.into_bytes();
        let mut prec = self.prec;
        if prec != -1 {
            if fs == 'o' && self.alt {
                prec -= 1;
            }
            for _ in 0..(prec - num.len() as i32) {
                output.push(b'0')
            }
        }
        output.extend(num);
        self.printf(w, output);
    }

    pub fn printf_str(&self, w: &mut Vec<u8>, mut val: Vec<u8>) {
        if self.prec != -1 {
            val.truncate(self.prec as usize);
        }
        self.printf(w, val);
    }

    fn printf(&self, w: &mut Vec<u8>, val: Vec<u8>) {
        if self.align == Align::LeftJust {
            w.extend(val.iter());
        }
        for _ in 0..(self.width as i32 - val.len() as i32) {
            w.push(b' ');
        }
        if self.align != Align::LeftJust {
            w.extend(val.iter());
        }
    }
}
