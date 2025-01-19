use super::terminal::Position;

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

    pub fn get_line(&self, row: usize, offset: Position) -> Option<&str> {
        if row + offset.row >= self.lines.len() {
            return None;
        }

        if offset.col >= self.lines[row].len() {
            return Some("");
        }

        return Some(&self.lines[row][offset.col..]);
    }
}
