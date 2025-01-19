#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn subtract(&self, other: &Self) -> Position {
        Position {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}
