use std::ops::Range;

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn default() -> Self {
        Buffer { lines: vec![] }
    }

    pub fn load(&mut self, content: String) {
        self.lines = content.lines().map(|x| x.to_string()).collect();
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_line(&self, row: usize, range: Range<usize>) -> Option<&str> {
        if row >= self.len() {
            return None;
        }

        let row_len = self.row_len(row);
        if range.start >= row_len {
            return Some("");
        }

        return Some(&self.lines[row][range.start..range.end.min(row_len)]);
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn row_len(&self, row: usize) -> usize {
        if row == self.len() {
            return 0;
        }

        self.lines[row].len()
    }
}
