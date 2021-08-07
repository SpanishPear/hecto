use crate::Terminal;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};


const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        Terminal::cursor_hide(); 
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            // after drawing rows, reset cursor
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        
        let pressed_key = Terminal::read_key()?;
           
        if let Key::Ctrl('q') = pressed_key { self.should_quit = true }

        Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {

            Terminal::clear_current_line();
            if row == height / 3 {
                println!("Tera - a text editor in rust -- version {}", VERSION);
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
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}


fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!(e);
}


