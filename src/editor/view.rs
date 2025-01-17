use super::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};
use std::{fs, io::Error};

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

    pub fn load(&mut self, file_name: &str) {
        if let Ok(content) = fs::read_to_string(file_name) {
            self.buffer.load(content);
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        if !self.buffer.is_empty() {
            self.render_buffer()?;
        } else {
            self.render_welcome_screen()?;
        }

        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(row) {
                Self::display_text(line)?;
            } else {
                Self::display_text("~")?;
            }

            if row.saturating_add(1) != height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn render_welcome_screen(&self) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;
            Self::display_text("~")?;

            if row == height / 3 {
                Self::display_welcome_message(row, width)?;
            }

            if row.saturating_add(1) != height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
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

        Terminal::move_caret_to(center_pos)?;
        Terminal::print(&hecto_info)
    }
}
