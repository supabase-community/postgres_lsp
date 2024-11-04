use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PgLspPath {
    path: PathBuf,
}

impl Deref for PgLspPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl PgLspPath {
    pub fn new(path_to_file: impl Into<PathBuf>) -> Self {
        Self {
            path: path_to_file.into(),
        }
    }
}

