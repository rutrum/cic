use std::env;

use cic::{align_anchor, Action, PromptAction, Dir, PromptType, Mode, Cursor, Table, Renderer, input};

struct State {
    table: Table,
    r: Renderer,
    c: Cursor,
    anchor: Cursor,
    m: Mode,
    buf: String,
    path: String,
}

impl State {
    fn from_path(path: &String) -> Self {
        Self {
            table: Table::from_path(&path),
            r: Renderer::new(),
            c: Cursor::new(),
            anchor: Cursor::new(),
            m: Mode::Table,
            buf: String::new(),
            path: path.to_string(),
        }
    }

    fn new() -> Self {
        Self {
            table: Table::new(),
            r: Renderer::new(),
            c: Cursor::new(),
            anchor: Cursor::new(),
            m: Mode::Table,
            buf: String::new(),
            path: String::new()
        }
    }

    fn take_buf(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }

    fn set_buf(&mut self, s: &String) {
        self.buf = s.clone();
    }

    fn draw_table(&mut self) {
        align_anchor(&mut self.anchor, self.c);
        self.r.draw_table(&self.table, &self.c, &self.anchor);
    }

    fn draw_prompt(&mut self, p: PromptType) {
        self.r.draw_prompt(p, &self.buf);
    }

    fn do_action(&mut self, action: Action) {
        use Action::*;
        match action {
            MoveCursor(dir) => {
                self.c.move_dir(dir, &self.table);
                self.draw_table();
            }
            EnterPrompt(p) => {
                match p {
                    PromptType::EditAppend => {
                        let v = self.table.get(self.c);
                        self.set_buf(&v);
                        self.draw_prompt(p);
                    }
                    _ => {
                        self.set_buf(&String::new());
                        self.draw_prompt(p);
                    }
                }
                self.m = Mode::Prompt(p);
            }

            Prompt(p_type, p_action) => match p_action {
                PromptAction::Push(c) => {
                    self.buf.push(c);
                    self.draw_prompt(p_type);
                }
                PromptAction::Backspace => { 
                    self.buf.pop(); 
                    self.draw_prompt(p_type);
                },
                PromptAction::Submit => {
                    if let PromptType::EditReplace = p_type {
                        let b = self.take_buf();
                        self.table.update(self.c, b);

                    } else if let PromptType::EditAppend = p_type {
                        let b = self.take_buf();
                        self.table.update(self.c, b);

                    } else if let PromptType::Command = p_type {
                        for action in input::from_prompt(self.take_buf()) {
                            self.do_action(action);
                        }
                    }
                    self.r.clear_prompt();
                    if let Mode::Prompt(_) = self.m {
                        self.m = Mode::Table;
                    }
                    self.draw_table();
                }
                PromptAction::Exit => {
                    self.r.clear_prompt();
                    self.m = Mode::Table;
                }
            }
            ClearCell => self.table.clear(self.c),

            AddRowBelow => {
                self.table.add_row_after(self.c);
                self.c.move_dir(Dir::Down, &self.table);
                self.draw_table();
            }
            AddRowAbove => {
                self.table.add_row_before(self.c);
                self.draw_table();
            }
            DeleteRow => {
                self.table.delete_row(&mut self.c);
                self.draw_table();
            }

            AddColLeft => {
                self.table.add_col_before(self.c);
                self.c.move_dir(Dir::Left, &self.table);
                self.draw_table();
            }
            AddColRight => {
                self.table.add_col_after(self.c);
                self.draw_table();
            }
            DeleteCol => {
                self.table.delete_col(&mut self.c);
                self.draw_table();
            }

            EnterMode(m) => self.m = m,

            Append(c) => {
                let mut val = self.table.get(self.c);
                val.push(c);
                self.table.update(self.c, val);
                self.draw_table();
            }
            Pop => {
                let mut val = self.table.get(self.c);
                val.pop();
                self.table.update(self.c, val);
                self.draw_table();
            }

            CarriageReturn => {
                // last row? add new row
                if self.c.y == self.table.dims().1 - 1 {
                    self.do_action(Action::AddRowBelow);
                }
                self.do_action(Action::MoveCursor(Dir::Start));
                self.do_action(Action::MoveCursor(Dir::Down));
                self.draw_table();
            }

            Save => self.table.save_to_path(self.path.to_string()),
            Quit => self.m = Mode::Exit,
        }
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
    s.draw_table();
    s.r.draw_status(s.m);

    loop {
        let actions = input::get_actions(s.m);
        for action in actions {
            s.do_action(action);
        }

        s.r.draw_status(s.m);

        if let Mode::Exit = s.m {
            break
        }
    }
}
