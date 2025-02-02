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
        self.lines
            .get(row)
            .map_or(None, |line| Some(line.get(range)))
    }

    pub fn row_width_until(&self, row: usize, grapheme_ind: usize) -> usize {
        self.lines
            .get(row)
            .map_or(0, |line| line.width_until(grapheme_ind))
    }

    pub fn get_valid_grapheme_ind(&self, row: usize, grapheme_ind: usize) -> usize {
        self.lines
            .get(row)
            .map_or(0, |line| line.grapheme_count().min(grapheme_ind))
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn grapheme_count(&self, row: usize) -> usize {
        self.lines.get(row).map_or(0, |line| line.grapheme_count())
    }
}
