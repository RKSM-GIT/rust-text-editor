use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

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

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(EditorCommand::Quit),
                (KeyCode::Up, _) => Ok(EditorCommand::Move(Direction::Up)),
                (KeyCode::Right, _) => Ok(EditorCommand::Move(Direction::Right)),
                (KeyCode::Down, _) => Ok(EditorCommand::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(EditorCommand::Move(Direction::Left)),
                (KeyCode::End, _) => Ok(EditorCommand::Move(Direction::End)),
                (KeyCode::Home, _) => Ok(EditorCommand::Move(Direction::Home)),
                (KeyCode::PageDown, _) => Ok(EditorCommand::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(EditorCommand::Move(Direction::PageUp)),
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
