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
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if let Key::Ctrl(c) = key {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    } 
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                },

                Err(err) => die(err),
            }
        }

    }

    // this is essentially an init function 
    // for the struct
    // with default values (but none for now)
    pub fn default() -> Self {
        Self {}
    }
}

fn die(e: std::io::Error) {
   std::panic::panic_any(e);
}


