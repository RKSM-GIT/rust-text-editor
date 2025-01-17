pub struct Buffer {
    pub lines: Vec<String>,
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
}
