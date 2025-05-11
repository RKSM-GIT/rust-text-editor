use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum EditCommand {
    Insert(char),
    InsertNewline,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for EditCommand {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(EditCommand::Insert(c)),
            (KeyCode::Tab, KeyModifiers::NONE) => Ok(EditCommand::Insert('\t')),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(EditCommand::InsertNewline),
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(EditCommand::Delete),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(EditCommand::DeleteBackward),
            _ => Err(format!("Unsupported key code {:?} with modifiers {:?}", event.code, event.modifiers))
        }
    }
}