pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn default() -> Self {
        let default_line = vec!["Hello, World".to_string(), "second line".to_string()];

        Buffer {
            lines: default_line,
        }
    }
}
