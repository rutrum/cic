use crossterm::event::{self, Event, KeyEvent, KeyCode};
use crate::Direction;

pub enum Command {
    MoveCursor(Direction),
    ClearCell,
    Quit,
}

/// Returns a command from a keystroke
pub fn read() -> Option<Command> {
    use Command::*;
    use Direction::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent {
            KeyEvent{ code: KeyCode::Char('k') | KeyCode::Up, .. } => Some(MoveCursor(Up)),
            KeyEvent{ code: KeyCode::Char('j') | KeyCode::Down, .. } => Some(MoveCursor(Down)),
            KeyEvent{ code: KeyCode::Char('h') | KeyCode::Left, .. } => Some(MoveCursor(Left)),
            KeyEvent{ code: KeyCode::Char('l') | KeyCode::Right, .. } => Some(MoveCursor(Right)),
            KeyEvent{ code: KeyCode::Char('q') | KeyCode::Esc, .. } => Some(Quit),
            KeyEvent{ code: KeyCode::Char('S'), .. } => Some(ClearCell),
            _ => None,
        },
        _ => None,
    }
}

/// Returns a Command from a string typed at the command prompt
pub fn from_line(s: String) -> Option<Command> {
    None
}
