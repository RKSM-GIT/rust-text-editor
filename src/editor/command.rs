use crossterm::event::Event;
use edit::EditCommand;
use moves::MoveCommand;
use system::SystemCommand;

use super::terminal::Size;

pub mod moves;
pub mod edit;
pub mod system;

pub enum Command {
    Move(MoveCommand),
    Edit(EditCommand),
    System(SystemCommand),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => MoveCommand::try_from(key_event)
                .map(Command::Move)
                .or_else(|_| EditCommand::try_from(key_event).map(Command::Edit))
                .or_else(|_| SystemCommand::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}"))
            ,
            Event::Resize(width, height) => Ok(Command::System(SystemCommand::Resize(Size {
                height: height as usize,
                width: width as usize,
            }))),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}