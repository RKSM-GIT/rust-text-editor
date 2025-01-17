use crossterm::{
    cursor, queue, style,
    terminal::{self, ClearType},
    Command,
};
use std::io::{stdout, Error, Write};

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(terminal::Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(terminal::Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(cursor::MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = terminal::size()?;
        Ok(Size {
            height: height as usize,
            width: width as usize,
        })
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(cursor::Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(cursor::Show)?;
        Ok(())
    }

    pub fn print(text: &str) -> Result<(), Error> {
        Self::queue_command(style::Print(text))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
