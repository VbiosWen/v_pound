use crate::editor::Editor;
use crossterm::{event, Result};
mod editor;
mod reader;

fn main() -> crate::Result<()> {
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
