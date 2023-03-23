use crate::reader::Reader;
use crossterm::Result;

pub struct Editor {
    reader: Reader,
}

impl Editor {
    pub fn new() -> Self {
        Self { reader: Reader }
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        self.reader.read();
        Ok(true)
    }
}
