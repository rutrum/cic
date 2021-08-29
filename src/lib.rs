use crossterm::terminal;

pub mod input;
mod render;
pub use render::Renderer;
mod table;
pub use table::Table;
pub use input::{Dir, PromptAction, Action};

use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Prompt(PromptType),
    Table,
    Insert,
    Exit,  // could remove and use option<mode> otherwise
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Mode::*;
        let s = match self {
            Insert => "Insert Mode",
            Table => "Movement Mode",
            Prompt(_) => "Prompt",
            _ => "",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PromptType {
    EditReplace,
    EditAppend,
    Command,
}

impl PromptType {
    fn ps2(self) -> String {
        use PromptType::*;
        match self {
            EditReplace => "edit: ",
            EditAppend => "edit: ",
            Command => ":",
        }.to_string()
    }
}

/// needs a table for context (also usize)
#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_dir(&mut self, dir: Dir, table: &Table) {
        use Dir::*;
        match dir {
            Up => if self.y > 0 {
                self.y -= 1;
            }
            Down => if self.y < table.dims().1 - 1 {
                self.y += 1;
            }
            Left => if self.x > 0 {
                self.x -= 1;
            }
            Right => if self.x < table.dims().0 - 1 {
                self.x += 1;
            }
            Top => {
                self.y = 0;
            }
            Bottom => {
                self.y = table.dims().1 - 1;
            }
            Start => {
                self.x = 0;
            }
            End => {
                self.x = table.dims().0 - 1;
            }
        }
    }
}

pub fn align_anchor(anchor: &mut Cursor, c: Cursor) {
    let (w, hs) = terminal::size().unwrap();
    let h = hs as usize;
    
    // is cursor above anchor?
    if anchor.y > c.y {
        anchor.y = c.y;
    }

    // is cursor below screen?
    // -2 is for padding
    if anchor.y + h - 3 < c.y {
        anchor.y = c.y + 3 - h; // order matters with usize
    }
}
