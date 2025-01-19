use super::{
    buffer::Buffer,
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
use std::fs;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    caret_location: Position,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            caret_location: Position::default(),
            scroll_offset: Position::default(),
        }
    }
}

impl View {
    pub fn get_caret_location(&self) -> Position {
        return self.caret_location;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(content) = fs::read_to_string(file_name) {
            self.buffer.load(content);
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;

        for row in 0..height {
            if let Some(line) = self.buffer.get_line(row, self.scroll_offset) {
                let line = &line[..width.min(line.len())];
                Self::render_text(row, line);
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::display_welcome_message(row, width);
            } else {
                Self::render_text(row, "~");
            }
        }

        self.needs_redraw = false;
    }

    fn render_text(row: usize, text: &str) {
        let result = Terminal::print_row(row, text);
        debug_assert!(result.is_ok(), "Error rendering text on line");
    }

    #[allow(clippy::cast_possible_truncation)]
    fn display_welcome_message(row: usize, width: usize) {
        let hecto_info = format!("{NAME} {VERSION}");
        let info_len = hecto_info.len();
        let col = if width / 2 >= (info_len - 1) / 2 {
            width / 2 - (info_len - 1) / 2
        } else {
            0
        };
        let center_pos = Position { row, col };

        let res1 = Terminal::move_caret_to(center_pos);
        let res2 = Terminal::print(&hecto_info);

        debug_assert!(res1.is_ok(), "Error Moving Carent");
        debug_assert!(res2.is_ok(), "Error Printing to terminal");
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.handle_move(direction),
            EditorCommand::Resize(size) => self.handle_resize(size),
            EditorCommand::Quit => {}
        }
    }

    fn handle_resize(&mut self, size: Size) {
        self.size = size;
        self.needs_redraw = true;
    }

    fn handle_move(&mut self, direction: Direction) {
        let Size { height, width } = Terminal::size().unwrap_or_default();
        let Position { mut row, mut col } = self.caret_location;

        match direction {
            Direction::Left => {
                if col == 0 {
                    self.scroll_offset.col = self.scroll_offset.col.saturating_sub(1);
                    self.needs_redraw = true;
                }
                col = col.saturating_sub(1);
            }
            Direction::Right => {
                if col == width.saturating_sub(1) {
                    self.scroll_offset.col = self.scroll_offset.col.saturating_add(1);
                    self.needs_redraw = true;
                }
                col = col.saturating_add(1).min(width.saturating_sub(1));
            }
            Direction::Up => {
                if row == 0 {
                    self.scroll_offset.row = self.scroll_offset.row.saturating_sub(1);
                    self.needs_redraw = true;
                }
                row = row.saturating_sub(1);
            }
            Direction::Down => {
                if row == height.saturating_sub(1) {
                    self.scroll_offset.row = self.scroll_offset.row.saturating_add(1);
                    self.needs_redraw = true;
                }
                row = row.saturating_add(1).min(height.saturating_sub(1));
            }
            Direction::Home => {
                col = 0;
            }
            Direction::End => {
                col = width.saturating_sub(1);
            }
            Direction::PageUp => {
                row = 0;
            }
            Direction::PageDown => {
                row = height.saturating_sub(1);
            }
        }

        self.caret_location.row = row;
        self.caret_location.col = col;
    }
}
