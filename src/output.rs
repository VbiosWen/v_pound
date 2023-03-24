use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use std::io::stdout;

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    }

    pub fn refresh_screen() -> crossterm::Result<()>{
        Self::clear_screen()
    }
}
