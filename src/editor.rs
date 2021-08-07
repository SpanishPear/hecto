use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

// we want this to be public to main.rs
// hence pub
pub struct Editor {}

impl Editor {
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
        Editor {}
    }
}

fn die(e: std::io::Error) {
   panic!(e);
}


