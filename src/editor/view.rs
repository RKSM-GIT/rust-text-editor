use super::{
    buffer::Buffer,
    editorcommand::{Direction, EditorCommand},
    position::Position,
    terminal::{Size, Terminal},
};
use std::fs;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    pos: Position,
    offset: Position,
    max_col: usize,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            pos: Position::default(),
            offset: Position::default(),
            max_col: 0,
        }
    }
}

impl View {
    pub fn get_position(&self) -> Position {
        return self.pos.subtract(&self.offset);
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
        let top = self.offset.row;
        let left = self.offset.col;
        let right = self.offset.col.saturating_add(width);

        for row in 0..height {
            if let Some(line) = self.buffer.get_line(row.saturating_add(top), left..right) {
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
        self.scroll_into_view();
        self.needs_redraw = true;
    }

    fn handle_move(&mut self, dir: Direction) {
        self.update_pos(dir);
        self.scroll_into_view();
    }

    fn update_pos(&mut self, dir: Direction) {
        let Size { height, .. } = self.size;
        let Position { mut row, mut col } = self.pos;

        match dir {
            Direction::Up => {
                row = row.saturating_sub(1);
                col = self.max_col.min(self.buffer.row_len(row));
            }
            Direction::Left => {
                if col == 0 && row == 0 {
                    return;
                } else if col == 0 {
                    row -= 1;
                    col = self.buffer.row_len(row);
                } else {
                    col -= 1;
                }
                self.max_col = self.max_col.max(col);
            }
            Direction::Down => {
                row = self.buffer.len().min(row + 1);
                col = self.max_col.min(self.buffer.row_len(row));
            }
            Direction::Right => {
                if col == 0 && row == self.buffer.len() {
                    return;
                } else if col == self.buffer.row_len(row) {
                    row += 1;
                    col = 0;
                } else {
                    col += 1;
                }
                self.max_col = self.max_col.max(col);
            }
            Direction::PageUp => {
                row = row.saturating_add(1).saturating_sub(height);
                col = self.buffer.row_len(row).min(col);
            }
            Direction::Home => {
                col = 0;
                self.max_col = 0;
            }
            Direction::PageDown => {
                row = self
                    .buffer
                    .len()
                    .min(row.saturating_add(height).saturating_sub(1));
                col = self.buffer.row_len(row).min(col);
            }
            Direction::End => {
                col = self.buffer.row_len(row);
                self.max_col = col;
            }
        }

        self.pos.row = row;
        self.pos.col = col;
    }

    fn scroll_into_view(&mut self) {
        let Size { height, width } = self.size;
        let Position { row, col } = self.pos;
        let Position {
            row: mut s_row,
            col: mut s_col,
        } = self.offset;

        // Horizontal Fix
        if row < s_row {
            s_row = row;
        } else if row >= s_row.saturating_add(height) {
            s_row = row.saturating_add(1).saturating_sub(height);
        }

        // Vertical Fix
        if col < s_col {
            s_col = col;
        } else if col >= s_col.saturating_add(width) {
            s_col = col.saturating_add(1).saturating_sub(width);
        }

        self.needs_redraw = self.offset.row != s_row || self.offset.col != s_col;

        self.offset.row = s_row;
        self.offset.col = s_col;
    }
}
