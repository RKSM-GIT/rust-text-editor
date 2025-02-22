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
    
    pub fn insert_char(&mut self, c: char, row: usize, grapheme_index: usize, has_len_increased: &mut bool) {
        let old_len = self.lines.get(row).map_or(0, |line| line.grapheme_count());

        match self.lines.get_mut(row) {
            Some(line) => line.insert_char(c, grapheme_index),
            None => {
                let mut s = String::new();
                s.push(c);
                self.lines.push(Line::from(s.as_str()));
            },
        }

        let new_len = self.lines.get(row).map_or(0, |line| line.grapheme_count());

        *has_len_increased = new_len > old_len;
    }

    pub fn delete_grapheme_at(&mut self, row: usize, grapheme_index: usize) {
        self.lines.get_mut(row).map_or_else(|| {}, |line| line.delete_grapheme_at(grapheme_index));
    }

    pub fn delete_and_merge(&mut self, row_del: usize, row_merge: usize) {
        let del_line_as_str = self.lines.get_mut(row_del).map_or(String::new(), |line| line.as_string());
        self.lines.get_mut(row_merge).unwrap().append_str(&del_line_as_str);
        self.lines.remove(row_del);
    }
}
