use std::{
    fmt::Display, 
    path::{Path, PathBuf}
};

pub struct FileInfo {
    path: Option<PathBuf>,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            path: None,
        }
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool {
        self.path.is_some()
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
        let name = self.get_path()
            .and_then(|x| x.file_name())
            .and_then(|x| x.to_str())
            .unwrap_or("[No Name]");

        write!(f, "{name}")
    }
}

