use std::{panic::RefUnwindSafe, sync::Arc};

use client::WorkspaceTransport;
use pg_fs::PgLspPath;
use serde::{Deserialize, Serialize};

use crate::WorkspaceError;

mod client;
// mod server;

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

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    /// The server's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;
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
