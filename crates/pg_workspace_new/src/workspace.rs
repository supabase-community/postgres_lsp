use std::panic::RefUnwindSafe;

use pg_fs::PgLspPath;

use crate::WorkspaceError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenFileParams {
    pub path: PgLspPath,
    pub content: String,
    pub version: i32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CloseFileParams {
    pub path: PgLspPath,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;
}

