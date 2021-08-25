use crossterm::event::{self, Event, KeyEvent, KeyCode};
use crate::Direction;
use crate::Mode;
use crate::Prompt;

pub enum TableEvent {
    MoveCursor(Direction),
    EnterPrompt(Prompt),
    EnterInsertMode,
    ClearCell,
    NewRowBelow,
    NewRowAbove,
    DeleteRow,
    Save,
    Quit,
}

pub enum PromptEvent {
    Literal(char),
    Backspace,
    Submit,
    Exit,
}

pub enum InsertEvent {
    Literal(char),
    NextCol,
    NextRow,
    Backspace,
    Exit,
}

pub fn read_table_event() -> Option<TableEvent> {
    use TableEvent::*;
    use Direction::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent {
            KeyEvent{ code: KeyCode::Char('k') | KeyCode::Up, .. } => Some(MoveCursor(Up)),
            KeyEvent{ code: KeyCode::Char('j') | KeyCode::Down, .. } => Some(MoveCursor(Down)),
            KeyEvent{ code: KeyCode::Char('h') | KeyCode::Left, .. } => Some(MoveCursor(Left)),
            KeyEvent{ code: KeyCode::Char('l') | KeyCode::Right, .. } => Some(MoveCursor(Right)),
            KeyEvent{ code: KeyCode::Char('q') | KeyCode::Esc, .. } => Some(Quit),
            KeyEvent{ code: KeyCode::Char('S'), .. } => Some(ClearCell),
            KeyEvent{ code: KeyCode::Char('o'), .. } => Some(NewRowBelow),
            KeyEvent{ code: KeyCode::Char('O'), .. } => Some(NewRowAbove),
            KeyEvent{ code: KeyCode::Char('D'), .. } => Some(DeleteRow),
            KeyEvent{ code: KeyCode::Char('I'), .. } => Some(EnterInsertMode),
            KeyEvent{ code: KeyCode::Char('r'), .. } => Some(EnterPrompt(Prompt::EditReplace)),
            KeyEvent{ code: KeyCode::Char('a'), .. } => Some(EnterPrompt(Prompt::EditAppend)),
            KeyEvent{ code: KeyCode::Char(':'), .. } => Some(EnterPrompt(Prompt::Command)),
            _ => None,
        },
        _ => None,
    }
}

pub fn read_prompt_event() -> Option<PromptEvent> {
    use PromptEvent::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent {
            KeyEvent{ code: KeyCode::Esc, .. } => Some(Exit),
            KeyEvent{ code: KeyCode::Enter, .. } => Some(Submit),
            KeyEvent{ code: KeyCode::Backspace, .. } => Some(Backspace),
            _ => if let KeyCode::Char(c) = keyevent.code {
                Some(Literal(c))
            } else {
                None
            },
        }
        _ => None,
    }
}

pub fn read_insert_event() -> Option<InsertEvent> {
    use InsertEvent::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent {
            KeyEvent{ code: KeyCode::Tab, .. } => Some(NextCol),
            KeyEvent{ code: KeyCode::Enter, .. } => Some(NextRow),
            KeyEvent{ code: KeyCode::Backspace, .. } => Some(Backspace),
            KeyEvent{ code: KeyCode::Esc, .. } => Some(Exit),
            _ => if let KeyCode::Char(c) = keyevent.code {
                Some(Literal(c))
            } else {
                None
            },
        }
        _ => None,
    }
}

/// Returns a Command from a string typed at the command prompt
pub fn from_prompt(s: String) -> Option<TableEvent> {
    use TableEvent::*;
    match s.as_str() {
        "w" | "write" => Some(Save),
        _ => None,
    }
}
