use crate::Navigable;
use crate::{terminal::Size, Document, Row, Terminal};
use log::{debug, info};
use std::env;
use termion::{color, event::Key};

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

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
        let width = self.terminal.size().width;
        let height = self.terminal.size().height;
        info!("Width: {}, Height: {}", width, height);
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

    pub fn document(&self) -> &Document {
        &self.document
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
            self.draw_status_bar();
            self.draw_message_bar();
            // after drawing rows, reset cursor
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;

        if let Some(navigation) = pressed_key.navigation_func() {
            self.cursor_position = navigation(&self, &self.cursor_position);
            self.scroll();
            return Ok(());
        }
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        debug!(
            "Cursor:  ({}, {}) - Offset: ({}, {})",
            x, y, offset.x, offset.y
        );
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn render_welcome(&self) {
        let mut welcome_msg = format!("Milli Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_msg.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_msg.truncate(width);
        println!("~{}{}\r", spaces, welcome_msg);
    }

    pub fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.terminal().size().width as usize + self.offset.x;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.render_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        };
        status = format!("{} - {} lines", file_name, self.document.len());
        if width > status.len() {
            status.push_str(&" ".repeat(width - status.len()));
        }

        Terminal::set_bg_color(STATUS_BG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
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
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
