use crate::{Row, Document, Terminal, terminal::Size};
use termion::event::Key;
use crate::Navigable;
use std::env;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

impl Position {
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
// we want this to be public to main.rs
// struct contains fields for the "class"
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
}


impl Editor {
    // clippy says unused self
    // removing self as per https://rust-lang.github.io/rust-clippy/master/index.html#unused_self
    // results in errors :( 
    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypresses() {
                die(error);
            }
        }
    }
    pub fn terminal(&self) -> &Terminal {
        &self.terminal
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide(); 
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            // after drawing rows, reset cursor
	          Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        
        let pressed_key = Terminal::read_key()?;
        
        
        if let Some(navigation) = pressed_key.navigation_func() {
            self.cursor_position = navigation(&self, &self.cursor_position); 
            return Ok(());
        }

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
			      _ => (),
        }  
        
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let Size {width, height} = self.terminal.size();
        let mut offset =&mut self.offset;
    }

    fn render_welcome(&self) {
        let mut welcome_msg = format!("Milli Editor -- version {}", VERSION);
		    let width = self.terminal.size().width as usize;
		    let len = welcome_msg.len();
		    let padding = width.saturating_sub(len) / 2; 
		    let spaces = " ".repeat(padding.saturating_sub(1));
		    welcome_msg.truncate(width);
		    println!("~{}{}\r",spaces, welcome_msg);
    }
    
    pub fn draw_row(&self, row: &Row) {
        
        let width = self.terminal().size().width;
        let start = self.offset.x;
        let end =  start + width as usize;
        let row = row.render(start, end);
        println!("{}\r",row);
    }


    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            
            let current_row = terminal_row  as usize+ self.offset.y ;
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(current_row) {
                self.draw_row(row);
            }
            else if self.document.is_empty() && terminal_row == height / 3 {
				        self.render_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    // this is essentially an init function 
    // for the struct
    // with default values (but none for now)
    pub fn default() -> Self {
        
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit:        false,
            terminal:           Terminal::default().expect("Failed to initialize terminal"),
            document,
			      cursor_position:    Position::default(),
            offset:             Position::default(), 
        }
    }

}


fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}


