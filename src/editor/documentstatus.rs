#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub curr_line_ind: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DocumentStatus {
    pub fn get_modified_string(&self) -> String {
        if self.is_modified {
            "(modified)".to_string()
        } else {
            String::new()
        }
    }

    pub fn get_line_count_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    pub fn get_position_indicator_string(&self) -> String {
        format!("{}/{}", self.curr_line_ind.saturating_add(1), self.total_lines)
    }
}