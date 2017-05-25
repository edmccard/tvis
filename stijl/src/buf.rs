use std::io;

use {Color, Result, Stream};


pub struct BufStream {
    actions: Vec<Action>,
    text: Vec<u8>,
}

enum Action {
    Text(usize),
    Flush,
    Reset,
    Fg(Color),
    Em,
}

impl BufStream {
    pub fn new() -> BufStream {
        BufStream {
            actions: Vec::new(),
            text: Vec::new(),
        }
    }

    pub fn print(&self, other: &mut Stream) -> Result<()> {
        use self::Action::*;

        let mut pos = 0;
        for action in &self.actions {
            match *action {
                Text(end) => {
                    other.write_all(&self.text[pos..end])?;
                    pos = end;
                }
                Flush => other.flush()?,
                Reset => other.reset()?,
                Fg(c) => other.fg(c)?,
                Em => other.em()?,
            }
        }
        other.flush()?;
        Ok(())
    }
}

impl io::Write for BufStream {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.text.extend(data);
        self.actions.push(Action::Text(self.text.len()));
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.actions.push(Action::Flush);
        Ok(())
    }
}

impl Stream for BufStream {
    fn reset(&mut self) -> Result<()> {
        self.actions.push(Action::Reset);
        Ok(())
    }

    fn fg(&mut self, fg: Color) -> Result<()> {
        self.actions.push(Action::Fg(fg));
        Ok(())
    }

    fn em(&mut self) -> Result<()> {
        self.actions.push(Action::Em);
        Ok(())
    }
}
