#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod navigation;
mod row; 
mod document;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;
pub use navigation::Navigable;
use simple_logging;
use log::LevelFilter;

fn main() {
    simple_logging::log_to_file("logs/log.txt", LevelFilter::Debug );
    let mut editor = Editor::default();
    editor.run();
}
