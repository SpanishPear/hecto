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

fn calc_line_width(editor: &Editor, y: usize) -> usize {
    if let Some(row) = editor.document().row(y) {
        row.len()
    } else {
        0
    }
 
}

fn navigate_line_end(editor: &Editor, position: &Position) -> Position { 
    let (_, y) = position.as_tuple();
    let last_col = calc_line_width(editor, y);
    debug!("line end to: {} {}", last_col, y);
    Position {x: last_col, y}
}

fn navigate_line_start(_: &Editor, position: &Position) -> Position {
    let (_, y) = position.as_tuple();
    
    Position {x: 0, y}
}

fn navigate_document_end(editor: &Editor, position: &Position) -> Position { 
    let (mut x, y) = position.as_tuple();
    let size = editor.document().len();
    let height = size.saturating_sub(1) as usize;
    let width = calc_line_width(editor, y);

    if x > width {
        x = width;
    }

    Position {x, y: height}
}

fn navigate_document_start(editor: &Editor, position: &Position) -> Position {
    let (mut x, _) = position.as_tuple();
    let width = calc_line_width(editor, 0);

    if x > width {
        x = width;
    }
    Position {x, y:0}
}


fn navigate_up(editor: &Editor, position: &Position) -> Position {
    let (mut x, y) = position.as_tuple();



    if y > 0 {
        info!("Navigating up    to ({} {})", x, y.saturating_sub(1));
        let prev_line_width = calc_line_width(editor, y.saturating_sub(1));
        if x > prev_line_width {
            x = prev_line_width;
        }   
        Position {x, y: y.saturating_sub(1)}
    } else {
        Position {x, y}
    }


}

fn navigate_down(editor: &Editor, position: &Position) -> Position {
    let (mut x, y) = position.as_tuple();
    let height = editor.document().len();

   if y < height {
        info!("Navigating down  to ({} {})", x, y.saturating_add(1));
        let next_line_width = calc_line_width(editor, y.saturating_add(1));
        if x > next_line_width {
            x = next_line_width;
        }
 
        Position {x, y: y.saturating_add(1)}
    } else {
        Position {x, y}
    }

}

fn navigate_left(editor: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    if x > 0 {
        Position {x: x - 1, y}
    } else if y > 0 {
        if let Some(row) = editor.document().row(y) {
            Position {x: row.len(), y: y - 1}
        } else {
            Position {x: 0, y: y - 1}
        }
    } 
    else {
        Position {x, y}
    }
}

fn navigate_right(editor: &Editor, position: &Position) -> Position {
    let (x, y) = position.as_tuple();
    let width = calc_line_width(editor, y);
    // is this suppsoed to be len - 1?
    let height = editor.document().len();
    
    if x < width {
        info!("Navigating right to ({} {})", x + 1, y);
        Position {x: x + 1 , y}
    } else if y < height {
        Position {x: 0, y: y + 1}
    } else {
        Position {x, y}
    }
}


