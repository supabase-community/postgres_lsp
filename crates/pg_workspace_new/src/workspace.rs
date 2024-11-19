use std::{panic::RefUnwindSafe, path::PathBuf, sync::Arc};

pub use self::client::{TransportRequest, WorkspaceClient, WorkspaceTransport};
use pg_configuration::PartialConfiguration;
use pg_fs::PgLspPath;
use serde::{Deserialize, Serialize};
use text_size::TextRange;

use crate::WorkspaceError;

mod client;
mod server;

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeFileParams {
    pub path: PgLspPath,
    pub version: i32,
    pub changes: Vec<ChangeParams>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeParams {
    /// The range of the file that changed. If `None`, the whole file changed.
    pub range: Option<TextRange>,
    pub text: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateSettingsParams {
    pub configuration: PartialConfiguration,
    pub workspace_directory: Option<PathBuf>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    /// The server's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Update the global settings for this workspace
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError>;

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError>;

    /// Returns information about the server this workspace is connected to or `None` if the workspace isn't connected to a server.
    fn server_info(&self) -> Option<&ServerInfo>;

}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server() -> Box<dyn Workspace> {
    Box::new(server::WorkspaceServer::new())
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server_sync() -> Arc<dyn Workspace> {
    Arc::new(server::WorkspaceServer::new())
}

// Convenience function for constructing a client instance of [Workspace]
pub fn client<T>(transport: T) -> Result<Box<dyn Workspace>, WorkspaceError>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync + 'static,
{
    Ok(Box::new(client::WorkspaceClient::new(transport)?))
}
