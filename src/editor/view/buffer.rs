use std::{fs::{self, File}, io::{Error, Write}, ops::Range};

use crate::editor::file_info::FileInfo;

use super::line::Line;

pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer { lines: vec![], file_info: FileInfo::new(), dirty: false }
    }
}

impl Buffer {
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(content) = fs::read_to_string(file_name) {
            self.lines = content.lines().map(|x| x.into()).collect();
            self.file_info = FileInfo::from(file_name);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
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
        self.dirty = true;
    }

    pub fn delete_grapheme_at(&mut self, row: usize, grapheme_index: usize) {
        self.lines.get_mut(row).map_or_else(|| {}, |line| line.delete_grapheme_at(grapheme_index));
        self.dirty = true;
    }

    pub fn delete_and_merge(&mut self, row_del: usize, row_merge: usize) {
        let del_line_as_str = self.lines
            .get_mut(row_del)
            .map_or(
                String::new(), 
                |line| line.as_string()
            );

        self.lines.get_mut(row_merge).unwrap().append_str(&del_line_as_str);
        self.lines.remove(row_del);
        self.dirty = true;
    }

    pub fn split_and_merge(&mut self, row_split: usize, split_ind: usize, row_merge: usize) {
        let splitted_fragments = self.lines
            .get_mut(row_split)
            .map_or(
                Vec::new(), 
                |line| line.split(split_ind)
            );
        self.lines.insert(row_merge, Line::new(splitted_fragments));
        self.dirty = true;
    }
}
