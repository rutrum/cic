use crossterm::{cursor, execute, QueueableCommand};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{self, ClearType, Clear, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use crate::{Table, Prompt, Cursor};

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

        let pretty = table.fmt_iter();
        for (r, row) in pretty.iter().enumerate() {
            self.stdout
                .queue(Print(r))
                .unwrap()
                .queue(Print("  "))
                .unwrap();
            for (c, cell) in row.iter().enumerate() {
                self.stdout
                    .queue(
                        if cur.y == r && cur.x == c {
                            Print(cell.clone().negative())
                        } else {
                            Print(cell.clone().stylize())
                        }
                    )
                    .unwrap()
                    .queue(Print("  "))
                    .unwrap();
            }
            self.stdout
                .queue(Clear(ClearType::UntilNewLine))
                .unwrap()
                .queue(cursor::MoveDown(1))
                .unwrap()
                .queue(cursor::MoveToColumn(0))
                .unwrap();
        }

        self.stdout
            .queue(Clear(ClearType::FromCursorDown))
            .unwrap()
            .flush().unwrap();
    }

    /// Draws at bottom of screen, exits raw mode, waits for input
    pub fn draw_prompt(&mut self, prompt: Prompt, buffer: &String) {
        let (_, last) = terminal::size().unwrap();
        self.stdout
            .queue(cursor::MoveTo(0, last))
            .unwrap()
            .queue(Print(prompt.ps2()))
            .unwrap()
            .queue(Print(buffer))
            .unwrap()
            .queue(Clear(ClearType::UntilNewLine))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    /// Draws at bottom of screen, exits raw mode, waits for input
    pub fn clear_prompt(&mut self) {
        let (_, last) = terminal::size().unwrap();
        self.stdout
            .queue(cursor::MoveTo(0, last-1))
            .unwrap()
            .queue(Clear(ClearType::CurrentLine))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_screen(&mut self) {
        self.stdout
            .queue(Clear(ClearType::All))
            .unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        disable_raw_mode();
        execute!(self.stdout, cursor::Show, LeaveAlternateScreen).unwrap();
    }
}
