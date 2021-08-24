//mod term;
//pub use term::Editor;

pub mod input;
mod render;
pub use render::Renderer;
mod table;
pub use table::Table;

pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Clone, Copy)]
pub struct Location(pub usize, pub usize);

/// needs a table for context (also usize)
#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_dir(&mut self, dir: Direction, table: &Table) {
        use Direction::*;
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
        }
    }
}
