use crate::editor::Editor;
use crossterm::{event, Result, terminal};
mod editor;
mod reader;
mod output;
mod editor_contents;
mod cursor_controller;
mod editor_rows;
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("unable to disable raw mode");
    }
}

fn main() -> crate::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
