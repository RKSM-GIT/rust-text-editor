use crossterm::{
    cursor, queue, style,
    terminal::{self, ClearType},
    Command,
};
use std::io::{stdout, Error, Write};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { row: 0, col: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
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

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(cursor::MoveTo(position.col, position.row))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = terminal::size()?;
        Ok(Size { height, width })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
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
