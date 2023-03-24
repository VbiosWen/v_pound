use crate::editor::Editor;
use crossterm::{event, Result, terminal};
mod editor;
mod reader;
mod output;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("unable to disable raw mode");
    }
}

fn main() -> crate::Result<()> {
    let _clean_up = CleanUp;
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
