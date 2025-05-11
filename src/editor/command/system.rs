use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::size::Size;

pub enum SystemCommand {
    Save,
    Resize(Size),
    Quit,
}

impl TryFrom<KeyEvent> for SystemCommand {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {code, modifiers, ..} = event;
 
        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Char('s') => Ok(Self::Save),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}