mod buffer;
mod line;

use super::{
    editorcommand::{Direction, EditorCommand},
    position::{Location, Position},
    terminal::{Size, Terminal},
};
use buffer::Buffer;
use std::fs;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    max_grapheme_ind: usize,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
            max_grapheme_ind: 0,
        }
    }
}

impl View {
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
        let top = self.scroll_offset.row;
        let left = self.scroll_offset.col;
        let right = self.scroll_offset.col.saturating_add(width);

        for row in 0..height {
            if let Some(line) = self.buffer.get_line(row.saturating_add(top), left..right) {
                Self::render_text(row, &line);
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

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(&self.scroll_offset)
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.update_pos(direction),
            EditorCommand::Resize(size) => self.handle_resize(size),
            EditorCommand::Quit => {}
        }
    }

    fn handle_resize(&mut self, size: Size) {
        self.size = size;
        self.scroll_into_view();
        self.needs_redraw = true;
    }

    fn scroll_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.row;
        let col = self
            .buffer
            .row_width_until(row, self.text_location.grapheme_index);
        Position { row, col }
    }

    fn scroll_vertically(&mut self, row: usize) {
        let height = self.size.height;
        let mut s_row = self.scroll_offset.row;
        let mut offset_changed = false;

        if row < s_row {
            s_row = row;
            offset_changed = true;
        } else if row >= s_row.saturating_add(height) {
            s_row = row.saturating_add(1).saturating_sub(height);
            offset_changed = true;
        }

        self.needs_redraw = self.needs_redraw || offset_changed;
        self.scroll_offset.row = s_row;
    }

    fn scroll_horizontally(&mut self, col: usize) {
        let width = self.size.width;
        let mut s_col = self.scroll_offset.col;
        let mut offset_changed = false;

        if col < s_col {
            s_col = col;
            offset_changed = true;
        } else if col >= s_col.saturating_add(width) {
            s_col = col.saturating_add(1).saturating_sub(width);
            offset_changed = true;
        }

        self.needs_redraw = self.needs_redraw || offset_changed;
        self.scroll_offset.col = s_col;
    }

    fn update_pos(&mut self, dir: Direction) {
        let Size { height, .. } = self.size;

        match dir {
            Direction::Up => self.move_up(1),
            Direction::Left => self.move_left(),
            Direction::Down => self.move_down(1),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::End => self.move_to_end_of_line(),
        }

        self.scroll_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.row = self.text_location.row.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.row = self.text_location.row.saturating_add(step);
        self.snap_to_valid_line();
        self.snap_to_valid_grapheme();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index == 0 && self.text_location.row == 0 {
            return;
        }

        if self.text_location.grapheme_index == 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        } else {
            self.text_location.grapheme_index -= 1;
        }

        self.max_grapheme_ind = self.text_location.grapheme_index;
    }

    fn move_right(&mut self) {
        if self.text_location.row == self.buffer.height() {
            return;
        }

        let line_width = self.buffer.grapheme_count(self.text_location.row);
        if self.text_location.grapheme_index == line_width {
            self.move_down(1);
            self.move_to_start_of_line();
        } else {
            self.text_location.grapheme_index += 1;
        }

        self.max_grapheme_ind = self.text_location.grapheme_index;
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer.grapheme_count(self.text_location.row)
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.row = self.text_location.row.min(self.buffer.height());
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .get_valid_grapheme_ind(self.text_location.row, self.max_grapheme_ind);
    }
}
