use std::io;

use {Color, Result, Stream};

/// A [`Stream`](trait.Stream.html) that records writes and style
/// changes.
#[derive(Default)]
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
    /// Create a new, empty `BufStream`.
    pub fn new() -> BufStream {
        BufStream {
            actions: Vec::new(),
            text: Vec::new(),
        }
    }

    /// Perform the actions recorded on this `BufStream` onto another
    /// `Stream`.
    pub fn playback(&self, other: &mut Stream) -> Result<()> {
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
    /// Record the call to `write`.
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.text.extend(data);
        self.actions.push(Action::Text(self.text.len()));
        Ok(data.len())
    }

    /// Record the call to `flush`.
    fn flush(&mut self) -> io::Result<()> {
        self.actions.push(Action::Flush);
        Ok(())
    }
}

/// The `Stream` methods of this implementation always succeed with
/// `Ok(())`.
impl Stream for BufStream {
    /// Record the call to `reset`.
    fn reset(&mut self) -> Result<()> {
        self.actions.push(Action::Reset);
        Ok(())
    }

    /// Record the call to `fg`.
    fn fg(&mut self, fg: Color) -> Result<()> {
        self.actions.push(Action::Fg(fg));
        Ok(())
    }

    /// Record the call to `em`.
    fn em(&mut self) -> Result<()> {
        self.actions.push(Action::Em);
        Ok(())
    }

    /// Always false.
    fn is_cli(&self) -> bool {
        false
    }
}
