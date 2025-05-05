mod buffer;
mod line;

use super::{
    documentstatus::DocumentStatus, editorcommand::{Direction, EditorCommand}, position::{Location, Position}, terminal::{Size, Terminal}, uicomponent::UiComponent, NAME, VERSION
};
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    max_grapheme_ind: usize,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            curr_line_ind: self.text_location.row,
            is_modified: self.buffer.is_dirty(),
            file_name: format!("{}", self.buffer.file_info),
        }
    }

    pub fn load(&mut self, file_name: &str) {
        self.buffer.load(file_name);
        self.set_needs_redraw(true);
    }

    fn render_text(row: usize, text: &str) {
        let result = Terminal::print_row(row, text);
        debug_assert!(result.is_ok(), "Error rendering text on line");
    }

    #[allow(clippy::cast_possible_truncation)]
    fn generate_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);

        // hide the welcome message if it doesn't fit entirely.
        if remaining_width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remaining_width$}", "~", welcome_message)
    }

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(&self.scroll_offset)
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.update_pos(direction),
            EditorCommand::Resize(_) => {},
            EditorCommand::Quit => {}
            EditorCommand::Insert(c) => self.insert_char(c),
            EditorCommand::Backspace => self.perform_backspace(),
            EditorCommand::Delete => self.perform_delete(),
            EditorCommand::Enter => self.perform_newline(),
            EditorCommand::Tab => self.insert_char('\t'),
            EditorCommand::Save => self.save(),
        }
    }

    fn save(&mut self) {
        let _ = self.buffer.save();
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

        self.set_needs_redraw(self.needs_redraw() || offset_changed);
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

        self.set_needs_redraw(self.needs_redraw() || offset_changed);
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
            Direction::Home => self.home_action(),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::End => self.end_action(),
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

    fn home_action(&mut self) {
        self.move_to_start_of_line();
        self.max_grapheme_ind = self.text_location.grapheme_index;
    }

    fn end_action(&mut self) {
        self.move_to_end_of_line();
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

    fn insert_char(&mut self, c: char) {
        let mut has_len_increased = false;

        self.buffer.insert_char(c, self.text_location.row, self.text_location.grapheme_index, &mut has_len_increased);

        if has_len_increased {
            self.move_right();
        }
        self.set_needs_redraw(true);
        self.scroll_into_view();
    }

    fn perform_backspace(&mut self) {
        let Location { row, grapheme_index } = self.text_location;

        if grapheme_index != 0 {
            self.move_left();
            self.buffer.delete_grapheme_at(row, grapheme_index-1);
        } else {
            self.move_left();
            if row > 0 &&  row != self.buffer.height() {
                self.buffer.delete_and_merge(row, row-1);
            }
        }

        self.snap_to_valid_grapheme();
        self.set_needs_redraw(true);
        self.scroll_into_view();
    }

    fn perform_delete(&mut self) {
        let Location { row, grapheme_index } = self.text_location;

        if grapheme_index != self.buffer.grapheme_count(row) {
            self.buffer.delete_grapheme_at(row, grapheme_index);
        } else if row < self.buffer.height().saturating_sub(1) {
            self.buffer.delete_and_merge(row+1, row);
        }

        self.snap_to_valid_grapheme();
        self.set_needs_redraw(true);
        self.scroll_into_view();
    }

    fn perform_newline(&mut self) {
        let Location { row, grapheme_index } = self.text_location;
        let row_merge = self.buffer.height().min(row + 1);

        self.buffer.split_and_merge(row, grapheme_index, row_merge);

        self.move_right();
        self.set_needs_redraw(true);
        self.scroll_into_view();
    }
}

impl UiComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let Size { height, width } = self.size;
        let end_y = origin_y.saturating_add(height);

        let vertical_center = height / 3;
        let top = self.scroll_offset.row;
        let left = self.scroll_offset.col;
        let right = self.scroll_offset.col.saturating_add(width);

        for row in origin_y..end_y {
            let line_idx = row
                .saturating_sub(origin_y)
                .saturating_add(top);

            if let Some(line) = self.buffer.get_line(line_idx, left..right) {
                Self::render_text(row, &line);
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_text(row, &Self::generate_welcome_message(width));
            } else {
                Self::render_text(row, "~");
            }
        }

        Ok(())
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_into_view();
    }
}
