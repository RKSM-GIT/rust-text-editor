#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn saturating_sub(&self, other: &Self) -> Position {
        Position {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}

#[derive(Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}
