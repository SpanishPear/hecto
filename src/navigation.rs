use termion::event::Key;
use log::{info, debug};
// these have been linked already in main.rs, so we are simply saying to use them in this file
use crate::Position;
use crate::Editor;

pub type Navigation = dyn Fn(&Editor, &Position) -> Position;

// If something is defined as something to be "Navigable"
// then it has access to the navigation_func function
pub trait Navigable {
    fn navigation_func(&self) -> Option<&'static Navigation>;
} 

// Keys are something that are navigable
// so lets implement the navigation_func for it!
impl Navigable for Key {
    
    fn navigation_func(&self) -> Option<&'static Navigation> {
        match *self {
            Key::Up => Some(&navigate_up),
            Key::Down => Some(&navigate_down),
            Key::Left => Some(&navigate_left),
            Key::Right => Some(&navigate_right),
            Key::PageUp => Some(&navigate_document_start),
            Key::PageDown =>Some(&navigate_document_end),
            Key::Home => Some(&navigate_line_start),
            Key::End => Some(&navigate_line_end),
            _ => None, 
        }

    }

}

fn navigate_line_end(editor: &Editor, position: &Position) -> Position { 
    let (_, y) = position.as_tuple();
    // this is the entire terminal size
    // need to replace this with the last column 
    let size = editor.terminal().size();
    let width = size.width.saturating_sub(1) as usize;
    debug!("line end to: {} {}", width, y);
    Position {x: width, y}
}

fn navigate_line_start(_: &Editor, position: &Position) -> Position {
    let (_, y) = position.as_tuple();
    
    Position {x: 0, y}
}

fn navigate_document_end(editor: &Editor, position: &Position) -> Position { 
    let (x, _) = position.as_tuple();
    // this is the entire terminal size
    // need to replace this with the last column 
    let size = editor.document().len();
    // height is n - line num is n - 1 
    let height = size.saturating_sub(1) as usize;
    Position {x, y: height}
}
fn navigate_document_start(_: &Editor, position: &Position) -> Position {
    let (x, _) = position.as_tuple();
    
    Position {x, y:0}
}


fn navigate_up(_: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    if y > 0 {
        info!("Navigating up    to ({} {})", x, y.saturating_sub(1));
        Position {x, y: y.saturating_sub(1)}
    } else {
        Position {x, y}
    }


}

fn navigate_down(editor: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    let height = editor.document().len();
    
    if y < height {
        info!("Navigating down  to ({} {})", x, y.saturating_add(1));
        Position {x, y: y.saturating_add(1)}
    } else {
        Position {x, y}
    }

}

fn navigate_left(_: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    
    if x > 0 {
        info!("Navigating left  to ({} {})", x.saturating_sub(1), y);
        Position {x: x.saturating_sub(1), y}
    } else {
        Position {x, y}
    }
}

fn navigate_right(editor: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    let width = editor.terminal().size().width.saturating_sub(1) as usize;
    
    if x < width {
        info!("Navigating right to ({} {})", x.saturating_add(1), y);
        Position  {x: x.saturating_add(1), y}
    } else {
        Position {x, y}
    }
}


