use std::{fmt::Display, path::PathBuf};

pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            path: None,
        }
    }
}

impl From<&str> for FileInfo {
    fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name))
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.path
            .as_ref()
            .and_then(|x| x.file_name())
            .and_then(|x| x.to_str())
            .unwrap_or("[No Name]");

        write!(f, "{name}")
    }
}

