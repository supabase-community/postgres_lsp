use crate::workspace::ServerInfo;
use crate::{TransportError, Workspace, WorkspaceError};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicU64, Ordering},
};

use super::{CloseFileParams, GetFileContentParams, IsPathIgnoredParams, OpenFileParams};

pub struct WorkspaceClient<T> {
    transport: T,
    request_id: AtomicU64,
    server_info: Option<ServerInfo>,
}

pub trait WorkspaceTransport {
    fn request<P, R>(&self, request: TransportRequest<P>) -> Result<R, TransportError>
    where
        P: Serialize,
        R: DeserializeOwned;
}

#[derive(Debug)]
pub struct TransportRequest<P> {
    pub id: u64,
    pub method: &'static str,
    pub params: P,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct InitializeResult {
    /// Information about the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
}

impl<T> WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    pub fn new(transport: T) -> Result<Self, WorkspaceError> {
        let mut client = Self {
            transport,
            request_id: AtomicU64::new(0),
            server_info: None,
        };

        // TODO: The current implementation of the JSON-RPC protocol in
        // tower_lsp doesn't allow any request to be sent before a call to
        // initialize, this is something we could be able to lift by using our
        // own RPC protocol implementation
        let value: InitializeResult = client.request(
            "initialize",
            json!({
                "capabilities": {},
                "clientInfo": {
                    "name": env!("CARGO_PKG_NAME"),
                    "version": pgt_configuration::VERSION
                },
            }),
        )?;

        client.server_info = value.server_info;

        Ok(client)
    }

    fn request<P, R>(&self, method: &'static str, params: P) -> Result<R, WorkspaceError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = TransportRequest { id, method, params };

        let response = self.transport.request(request)?;

        Ok(response)
    }

    pub fn shutdown(self) -> Result<(), WorkspaceError> {
        self.request("pgt/shutdown", ())
    }
}

impl<T> Workspace for WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.request("pgt/open_file", params)
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.request("pgt/close_file", params)
    }

    fn change_file(&self, params: super::ChangeFileParams) -> Result<(), WorkspaceError> {
        self.request("pgt/change_file", params)
    }

    fn update_settings(&self, params: super::UpdateSettingsParams) -> Result<(), WorkspaceError> {
        self.request("pgt/update_settings", params)
    }

    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        self.request("pgt/is_path_ignored", params)
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.request("pgt/get_file_content", params)
    }

    fn pull_diagnostics(
        &self,
        params: super::PullDiagnosticsParams,
    ) -> Result<super::PullDiagnosticsResult, WorkspaceError> {
        self.request("pgt/pull_diagnostics", params)
    }

    fn get_completions(
        &self,
        params: super::GetCompletionsParams,
    ) -> Result<super::CompletionResult, WorkspaceError> {
        self.request("pgt/get_completions", params)
    }
}
