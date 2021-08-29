use crossterm::{cursor, execute, QueueableCommand};
use crossterm::style::{Print, Stylize, Attribute, SetAttribute};
use crossterm::terminal::{self, ClearType, Clear, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use crate::{Mode, Table, PromptType, Cursor};

use std::io::{self, Write};
use std::ops::Drop;

pub struct Renderer {
    stdout: io::Stdout,
}

impl Renderer {
    
    /// Enables raw mode and enter alternate screen
    pub fn new() -> Renderer {
        let mut stdout = io::stdout();
        enable_raw_mode().unwrap();
        execute!(stdout, cursor::Hide, EnterAlternateScreen).unwrap();
        Renderer{ stdout }
    }

    /// Draws the table to the screen, highlights the cell the cursor is located
    pub fn draw_table(&mut self, table: &Table, cur: &Cursor, anchor: &Cursor) {
        self.stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        let (w, h) = table.dims();

        let pretty = table.fmt_iter();
        for (r, row) in pretty.iter().enumerate().skip(anchor.y) {
            self.queue_row_index(r, h);
            for (c, cell) in row.iter().enumerate() {
                self.stdout
                    .queue(
                        if cur.y == r && cur.x == c {
                            Print(format!("{} ", cell).negative())
                        } else {
                            Print(format!("{} ", cell).stylize())
                        }
                    )
                    .unwrap()
                    .queue(Print(" "))
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

    fn queue_row_index(&mut self, i: usize, total_rows: usize) {
        let w = format!("{}", total_rows).len();
        self.stdout
            .queue(Print(format!("{: <1$}  ", i, w)))
            .unwrap();
    }

    /// Draws at bottom of screen, exits raw mode, waits for input
    pub fn draw_prompt(&mut self, prompt: PromptType, buffer: &String) {
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
        self.flush();
    }

    pub fn clear_screen(&mut self) {
        self.stdout
            .queue(Clear(ClearType::All))
            .unwrap();
        self.flush();
    }

    pub fn draw_status(&mut self, mode: Mode) {
        let msg = text_full_width(format!("{}", mode));
        let (_, last) = terminal::size().unwrap();

        self.stdout
            .queue(cursor::MoveTo(0, last-2))
            .unwrap()
            .queue(SetAttribute(Attribute::Reverse))
            .unwrap()
            .queue(Print(msg))
            .unwrap()
            .queue(Clear(ClearType::UntilNewLine))
            .unwrap()
            .queue(SetAttribute(Attribute::Reset))
            .unwrap();
        self.flush();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.stdout, cursor::Show, LeaveAlternateScreen).unwrap();
    }
}

/// Returns a string padded to be the width of the terminal
fn text_full_width(s: String) -> String {
    let (w, _) = terminal::size().unwrap();
    format!(" {: <1$}", s, w as usize - 1)
}
