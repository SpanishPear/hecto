use crate::Terminal;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

// we want this to be public to main.rs
// struct contains fields for the "class"
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            // after drawing rows, reset cursor
            Terminal::cursor_position(0, 0);
        }
        Terminal::flush()
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        
        let pressed_key = Terminal::read_key()?;
           
        if let Key::Ctrl('q') = pressed_key { self.should_quit = true }

        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            // ~ then newline
            println!("~\r");
        }
    }

    // this is essentially an init function 
    // for the struct
    // with default values (but none for now)
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}


fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    std::panic::panic_any(e);
}


