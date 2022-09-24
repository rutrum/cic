use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use crate::Mode;
use crate::PromptType;

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up, Down, Left, Right,
    Top, Bottom, Start, End
}

/// Actions are unique and have the same
/// behavior no matter the current mode.
pub enum Action {
    MoveCursor(Dir),

    EnterPrompt(PromptType),
    Prompt(PromptType, PromptAction),

    EnterMode(Mode),

    Append(char),
    Pop,

    CarriageReturn,

    ClearCell,

    AddRowAbove,
    AddRowBelow,
    DeleteRow,

    AddColLeft,
    AddColRight,
    DeleteCol,

    Save,
    Quit,
}

pub enum PromptAction {
    Push(char),
    Backspace,
    Submit,
    Exit,
}

pub fn get_actions(mode: Mode) -> Vec<Action> {
    match mode {
        Mode::Table => table_mode_actions(),
        Mode::Prompt(p) => prompt_mode_actions(p),
        Mode::Insert => insert_mode_actions(),
        _ => Vec::new(),
    }
}

pub fn prompt_mode_actions(p: PromptType) -> Vec<Action> {
    use Action::*;
    use PromptAction::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Esc => vec![Prompt(p, Exit)],
            KeyCode::Enter => vec![Prompt(p, Submit)],
            KeyCode::Backspace => vec![Prompt(p, Backspace)],
            KeyCode::Char(c) => vec![Prompt(p, Push(c))],
            _ => Vec::new(),
        }
        _ => Vec::new(),
    }
}

pub fn insert_mode_actions() -> Vec<Action> {
    use Action::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match (keyevent.code, keyevent.modifiers.contains(KeyModifiers::SHIFT)) {
            (KeyCode::Up, _) => vec![MoveCursor(Dir::Up)],
            (KeyCode::Down, _) => vec![MoveCursor(Dir::Down)],
            (KeyCode::Left, _) => vec![MoveCursor(Dir::Left)],
            (KeyCode::Right, _) => vec![MoveCursor(Dir::Right)],

            (KeyCode::Esc, _) => vec![EnterMode(Mode::Table)],
            (KeyCode::Backspace, _) => vec![Pop],
            (KeyCode::Tab, _) => vec![MoveCursor(Dir::Right)],
            (KeyCode::BackTab, _) => vec![MoveCursor(Dir::Left)],
            (KeyCode::Enter, _) => vec![CarriageReturn],
            (KeyCode::Char(c), _) => vec![Append(c)],
            _ => Vec::new(),
        }
        _ => Vec::new(),
    }
}

pub fn table_mode_actions() -> Vec<Action> {
    use Action::*;
    match event::read().unwrap() {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('k') | KeyCode::Up => vec![MoveCursor(Dir::Up)],
            KeyCode::Char('j') | KeyCode::Down => vec![MoveCursor(Dir::Down)],
            KeyCode::Char('h') | KeyCode::Left => vec![MoveCursor(Dir::Left)],
            KeyCode::Char('l') | KeyCode::Right => vec![MoveCursor(Dir::Right)],
            KeyCode::Char('g') => vec![MoveCursor(Dir::Top)],
            KeyCode::Char('G') => vec![MoveCursor(Dir::Bottom)],
            KeyCode::Char('0') => vec![MoveCursor(Dir::Start)],
            KeyCode::Char('$') => vec![MoveCursor(Dir::End)],

            KeyCode::Esc => vec![Quit],
            KeyCode::Char('S') => vec![ClearCell],
            KeyCode::Char('o') => vec![AddRowBelow],
            KeyCode::Char('O') => vec![AddRowAbove],
            KeyCode::Char('D') => vec![DeleteRow],

            KeyCode::Char('I') => vec![EnterMode(Mode::Insert)],

            KeyCode::Char('c') => vec![EnterPrompt(PromptType::EditReplace)],
            KeyCode::Char('a') => vec![EnterPrompt(PromptType::EditAppend)],
            KeyCode::Char(':') => vec![EnterPrompt(PromptType::Command)],
            _ => Vec::new(),
        }
        _ => Vec::new(),
    }
}

/// Returns a Command from a string typed at the command prompt
pub fn from_prompt(s: String) -> Vec<Action> {
    use Action::*;
    match s.to_lowercase().as_str() {
        "w" | "write" => vec![Save],
        "q" | "quit" => vec![Quit],
        "addcol" => vec![AddColRight],
        "delcol" => vec![DeleteCol],
        _ => Vec::new(),
    }
}
