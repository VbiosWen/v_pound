use std::time::Duration;

use crossterm::event::{KeyEvent, self, Event};

pub struct Reader;

impl Reader {
    pub fn read(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()?{
                    println!("{:?}",event);
                    return Ok(event)
                }
            }
        }
    }
}
