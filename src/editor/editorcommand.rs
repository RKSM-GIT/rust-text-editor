use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    PageUp,
    Home,
    PageDown,
    End,
}

#[derive(Clone, Copy)]
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Insert(char),
    Backspace,
    Delete,
    Quit,
    Enter,
    Tab,
    Save,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                (KeyCode::Tab, _) => Ok(Self::Tab),
                _ => Err(format!("Key code not supported: {code:?}")),
            },
            Event::Resize(width, height) => Ok(EditorCommand::Resize(Size {
                width: width as usize,
                height: height as usize,
            })),
            _ => Err(format!("Unsupported Event: {event:?}")),
        }
    }
}
