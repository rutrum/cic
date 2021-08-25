use std::env;

use cic::{Direction, Prompt, Mode, Cursor, Table, Renderer, input::{self, TableEvent, PromptEvent}};

struct State {
    pub table: Table,
    pub r: Renderer,
    pub c: Cursor,
    pub m: Mode,
    pub buf: String,
    pub path: String,
}

impl State {
    fn from_path(path: &String) -> Self {
        Self {
            table: Table::from_path(&path),
            r: Renderer::new(),
            c: Cursor::new(),
            m: Mode::Table,
            buf: String::new(),
            path: path.to_string(),
        }
    }

    fn take_buf(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }

    fn set_buf(&mut self, s: &String) {
        self.buf = s.clone();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Enter a csv to edit");
        return
    }

    let filename = &args[1];

    let mut s = State::from_path(filename);

    loop {
        match s.m {
            Mode::Table => {
                s.r.draw_table(&s.table, &s.c);
                table_mode_update(&mut s);
            }
            Mode::Prompt(p) => prompt_mode_update(&mut s, p),
            Mode::Insert => {
                insert_mode_update(&mut s);
            }
            Mode::Exit => break,
        }
    }
}

fn insert_mode_update(s: &mut State) {
    if let Some(event) = input::read_insert_event() {

    }
}

fn table_mode_update(s: &mut State) {
    if let Some(event) = input::read_table_event() {
        perform_table_event(s, event);
    }
}

fn perform_table_event(s: &mut State, ev: TableEvent) {
    use TableEvent::*;
    match ev { 
        Quit => s.m = Mode::Exit,
        Save => s.table.save_to_path(s.path.to_string()),
        ClearCell => s.table.clear(s.c),
        NewRowBelow => {
            s.table.add_row(s.c.y + 1);
            s.c.move_dir(Direction::Down, &s.table);
        }
        NewRowAbove => s.table.add_row(s.c.y),
        EnterInsertMode => s.m = Mode::Insert,
        DeleteRow => {
            s.table.delete_row(&mut s.c);
        }
        EnterPrompt(p) => {
            match p {
                Prompt::EditAppend => {
                    let v = s.table.get(s.c.y, s.c.x);
                    s.set_buf(&v);
                    s.r.draw_prompt(p, &v);
                }
                _ => {
                    s.r.draw_prompt(p, &String::new());
                }
            }
            s.m = Mode::Prompt(p);
        }
        MoveCursor(dir) => s.c.move_dir(dir, &s.table),
    }
}

fn prompt_mode_update(s: &mut State, prompt: Prompt) {
    if let Some(event) = input::read_prompt_event() {
        use PromptEvent::*;
        match event {
            Literal(c) => {
                s.buf.push(c);
                s.r.draw_prompt(prompt, &s.buf);
            }
            Backspace => {
                s.buf.pop();
                s.r.draw_prompt(prompt, &s.buf);
            }
            Submit => {
                if let Prompt::EditReplace = prompt {
                    let b = s.take_buf();
                    s.table.update(s.c, b);
                } else if let Prompt::EditAppend = prompt {
                    let b = s.take_buf();
                    s.table.update(s.c, b);
                } else if let Prompt::Command = prompt {
                    let maybe_ev = input::from_prompt(s.take_buf());
                    if let Some(ev) = maybe_ev {
                        return perform_table_event(s, ev);
                    }
                }
                s.r.clear_prompt();
                s.m = Mode::Table;
            }
            Exit => {
                // wipe out prompt
                s.r.clear_prompt();
                s.m = Mode::Table;
            }
        }
    }
}
