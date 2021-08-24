use crossterm::{cursor, execute, QueueableCommand};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use crate::{Table, Cursor};

use std::io::{self, Write};
use std::ops::Drop;

pub struct Renderer {
    stdout: io::Stdout,
}

impl Renderer {
    
    /// Enables raw mode and enter alternate screen
    pub fn new() -> Renderer {
        let mut stdout = io::stdout();
        enable_raw_mode();
        execute!(stdout, cursor::Hide, EnterAlternateScreen).unwrap();
        Renderer{ stdout }
    }

    /// Draws the table to the screen, highlights the cell the cursor is located
    pub fn draw_table(&mut self, table: &Table, cur: &Cursor) {
        self.stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        for (r, row) in table.rows().iter().enumerate() {
            for (c, (cell, width)) in row.iter().zip(table.col_widths().iter()).enumerate() {
                self.stdout
                    .queue(
                        if cur.y == r && cur.x == c {
                            Print(cell.clone().negative())
                        } else {
                            Print(cell.clone().stylize())
                        }
                    )
                    .unwrap()
                    .queue(cursor::MoveRight((*width - cell.len() as i32 + 2) as u16)) // need to write over with spaces
                    .unwrap();
            }
            self.stdout
                .queue(cursor::MoveDown(1))
                .unwrap()
                .queue(cursor::MoveToColumn(0))
                .unwrap();
        }

        self.stdout.flush().unwrap();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        disable_raw_mode();
        execute!(self.stdout, cursor::Show, LeaveAlternateScreen).unwrap();
    }
}
