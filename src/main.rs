#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod navigation;

use editor::Editor;

pub use terminal::Terminal;
pub use editor::Position;
pub use navigation::Navigable;


fn main() {
    let mut editor = Editor::default();
    editor.run();
}
