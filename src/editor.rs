use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

// we want this to be public to main.rs
// hence pub
pub struct Editor {}

impl Editor {
    // clippy says unused self
    // removing self as per https://rust-lang.github.io/rust-clippy/master/index.html#unused_self
    // results in errors :( 
    pub fn run(&self) {
        
        let _stdout = stdout().into_raw_mode().unwrap();
       
        loop {
            if let Err(error) = self.process_keypresses() {
                die(error);
            }
        }
    }

    fn process_keypresses(&self) -> Result<(), std::io::Error> {
        
        let pressed_key = read_key()?;
        match pressed_key  {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }

    // this is essentially an init function 
    // for the struct
    // with default values (but none for now)
    pub fn default() -> Self {
        Self {}
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
   std::panic::panic_any(e);
}


