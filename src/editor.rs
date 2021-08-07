use crate::Terminal;
use std::io::{self, stdout, Write};
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
        
        let _stdout = stdout().into_raw_mode().unwrap();
       
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
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            // after drawing rows, reset cursor
            print!("{}", termion::cursor::Goto(1,1));
        }
        io::stdout().flush()
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        
        let pressed_key = read_key()?;
       
        if let Key::Ctrl('q') = pressed_key { self.should_quit = true }

        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
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

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
        
    }
}


fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    std::panic::panic_any(e);
}


