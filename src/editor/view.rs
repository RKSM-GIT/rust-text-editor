use super::terminal::{Position, Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn draw_rows() -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello, World!")?;

        for row in 1..height {
            if row == height / 3 {
                Self::display_welcome_message(row, width)?;
            } else {
                Self::display_empty_row(row)?;
            }
        }

        Ok(())
    }

    fn display_empty_row(row: u16) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print("~")
    }

    #[allow(clippy::cast_possible_truncation)]
    fn display_welcome_message(row: u16, width: u16) -> Result<(), Error> {
        let hecto_info = format!("{NAME} {VERSION}");
        let info_len = hecto_info.len() as u16;
        let col = if width / 2 >= (info_len - 1) / 2 {
            width / 2 - (info_len - 1) / 2
        } else {
            0
        };
        let center_pos = Position { row, col };

        Self::display_empty_row(row)?;
        Terminal::move_caret_to(center_pos)?;
        Terminal::print(&hecto_info)
    }
}
