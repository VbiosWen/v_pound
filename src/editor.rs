use crate::output::Output;
use crate::reader::Reader;
use crossterm::Result;

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        self.reader.read();
        Ok(true)
    }
}
