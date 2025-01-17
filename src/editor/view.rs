use super::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};
use std::{fs, io::Error};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.needs_redraw = true;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(content) = fs::read_to_string(file_name) {
            self.buffer.load(content);
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        let vertical_center = height / 3;

        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let line = &line[..width.min(line.len())];
                Self::render_text(row, line)?;
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::display_welcome_message(row, width)?;
            } else {
                Self::render_text(row, "~")?;
            }
        }

        self.needs_redraw = false;
        Ok(())
    }

    fn render_text(row: usize, text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row, col: 0 })?;
        Terminal::clear_line()?;
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
