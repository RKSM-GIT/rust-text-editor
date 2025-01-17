use super::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn default() -> Self {
        View {
            buffer: Buffer::default(),
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;

            if row == height / 3 {
                Self::display_welcome_message(row, width)?;
            } else if let Some(line) = self.buffer.lines.get(row) {
                Self::display_text(line)?;
            } else {
                Self::display_empty_row()?;
            }

            if row.saturating_add(1) != height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn display_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }

    fn display_text(text: &str) -> Result<(), Error> {
        Terminal::print(text)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn display_welcome_message(row: usize, width: usize) -> Result<(), Error> {
        let hecto_info = format!("{NAME} {VERSION}");
        let info_len = hecto_info.len();
        let col = if width / 2 >= (info_len - 1) / 2 {
            width / 2 - (info_len - 1) / 2
        } else {
            0
        };
        let center_pos = Position { row, col };

        Self::display_empty_row()?;
        Terminal::move_caret_to(center_pos)?;
        Terminal::print(&hecto_info)
    }
}
