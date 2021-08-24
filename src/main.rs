use std::env;

use cic::{Cursor, Table, Renderer, input::{self, Command}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Enter a csv to edit");
        return
    }

    let filename = &args[1];
    let mut table = Table::from_path(filename);
    let mut r = Renderer::new();

    let mut c = Cursor::new();

    loop {
        r.draw_table(&table, &c);

        if let Some(command) = input::read() {
            match command { 
                Command::Quit => break,
                Command::ClearCell => table.clear(c),
                Command::MoveCursor(dir) => c.move_dir(dir, &table),
            }
        }
    }
}

fn wait() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
