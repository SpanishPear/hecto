use crate::Terminal;
use termion::event::Key;
use crate::Navigable;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        Terminal::cursor_position(&Position{x: 0, y: 0});
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

        Ok(())
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

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {

            Terminal::clear_current_line();
            if row == height / 3 {
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
        Self {
            should_quit:        false,
            terminal:           Terminal::default().expect("Failed to initialize terminal"),
			      cursor_position:    Position {x: 0, y: 0},
        }
    }

}


fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}


