use std::ops::Range;

use super::line::Line;

pub struct Buffer {
    lines: Vec<Line>,
}

impl Buffer {
    pub fn default() -> Self {
        Buffer { lines: vec![] }
    }

    pub fn load(&mut self, content: String) {
        self.lines = content.lines().map(|x| x.into()).collect();
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_line(&self, row: usize, range: Range<usize>) -> Option<String> {
        if row >= self.height() {
            return None;
        }

        return Some(self.lines[row].get(range));
    }

    pub fn row_width_until(&self, row: usize, grapheme_ind: usize) -> usize {
        self.lines[row].width_until(grapheme_ind)
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn row_width(&self, row: usize) -> usize {
        if row == self.height() {
            return 0;
        }

        self.lines[row].total_width()
    }
}
