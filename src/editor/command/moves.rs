use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum MoveCommand {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

impl TryFrom<KeyEvent> for MoveCommand {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {code, modifiers, ..} = event;

        if modifiers == KeyModifiers::NONE {
            return match code {
                KeyCode::Left => Ok(MoveCommand::Left),
                KeyCode::Right => Ok(MoveCommand::Right),
                KeyCode::Up => Ok(MoveCommand::Up),
                KeyCode::Down => Ok(MoveCommand::Down),
                KeyCode::Home => Ok(MoveCommand::Home),
                KeyCode::End => Ok(MoveCommand::End),
                KeyCode::PageUp => Ok(MoveCommand::PageUp),
                KeyCode::PageDown => Ok(MoveCommand::PageDown),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            return Err(format!("Unsupported key code {code:?} or modifier {modifiers:?}"));
        }
    }
}