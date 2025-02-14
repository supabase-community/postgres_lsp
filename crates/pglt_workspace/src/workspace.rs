use std::{panic::RefUnwindSafe, path::PathBuf, sync::Arc};

pub use self::client::{TransportRequest, WorkspaceClient, WorkspaceTransport};
use pglt_analyse::RuleCategories;
use pglt_configuration::{PartialConfiguration, RuleSelector};
use pglt_fs::PgLTPath;
use serde::{Deserialize, Serialize};
use text_size::{TextRange, TextSize};

use crate::WorkspaceError;

mod client;
mod server;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenFileParams {
    pub path: PgLTPath,
    pub content: String,
    pub version: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CloseFileParams {
    pub path: PgLTPath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeFileParams {
    pub path: PgLTPath,
    pub version: i32,
    pub changes: Vec<ChangeParams>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PullDiagnosticsParams {
    pub path: PgLTPath,
    pub categories: RuleCategories,
    pub max_diagnostics: u64,
    pub only: Vec<RuleSelector>,
    pub skip: Vec<RuleSelector>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionParams {
    /// The File for which a completion is requested.
    pub path: PgLTPath,
    /// The Cursor position in the file for which a completion is requested.
    pub position: TextSize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PullDiagnosticsResult {
    pub diagnostics: Vec<pglt_diagnostics::serde::Diagnostic>,
    pub errors: usize,
    pub skipped_diagnostics: u64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
/// Which fixes should be applied during the analyzing phase
pub enum FixFileMode {
    /// Applies [safe](pglt_diagnostics::Applicability::Always) fixes
    SafeFixes,
    /// Applies [safe](pglt_diagnostics::Applicability::Always) and [unsafe](pglt_diagnostics::Applicability::MaybeIncorrect) fixes
    SafeAndUnsafeFixes,
    /// Applies suppression comments to existing diagnostics when using `--suppress`
    ApplySuppressions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeParams {
    /// The range of the file that changed. If `None`, the whole file changed.
    pub range: Option<TextRange>,
    pub text: String,
}

impl ChangeParams {
    pub fn overwrite(text: String) -> Self {
        Self { range: None, text }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IsPathIgnoredParams {
    pub pglt_path: PgLTPath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateSettingsParams {
    pub configuration: PartialConfiguration,
    pub vcs_base_path: Option<PathBuf>,
    pub gitignore_matches: Vec<String>,
    pub workspace_directory: Option<PathBuf>,
    pub skip_db: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetFileContentParams {
    pub path: PgLTPath,
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
    /// Retrieves the list of diagnostics associated to a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError>;

    fn get_completions(
        &self,
        params: CompletionParams,
    ) -> Result<pglt_completions::CompletionResult, WorkspaceError>;

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

    /// Return the content of a file
    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError>;

    /// Checks if the current path is ignored by the workspace.
    ///
    /// Takes as input the path of the file that workspace is currently processing and
    /// a list of paths to match against.
    ///
    /// If the file path matches, then `true` is returned, and it should be considered ignored.
    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError>;
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

/// [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
/// guard for an open file in a workspace, takes care of closing the file
/// automatically on drop
pub struct FileGuard<'app, W: Workspace + ?Sized> {
    workspace: &'app W,
    path: PgLTPath,
}

impl<'app, W: Workspace + ?Sized> FileGuard<'app, W> {
    pub fn open(workspace: &'app W, params: OpenFileParams) -> Result<Self, WorkspaceError> {
        let path = params.path.clone();
        workspace.open_file(params)?;
        Ok(Self { workspace, path })
    }

    pub fn change_file(
        &self,
        version: i32,
        changes: Vec<ChangeParams>,
    ) -> Result<(), WorkspaceError> {
        self.workspace.change_file(ChangeFileParams {
            path: self.path.clone(),
            version,
            changes,
        })
    }

    pub fn get_file_content(&self) -> Result<String, WorkspaceError> {
        self.workspace.get_file_content(GetFileContentParams {
            path: self.path.clone(),
        })
    }

    pub fn pull_diagnostics(
        &self,
        categories: RuleCategories,
        max_diagnostics: u32,
        only: Vec<RuleSelector>,
        skip: Vec<RuleSelector>,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            path: self.path.clone(),
            categories,
            max_diagnostics: max_diagnostics.into(),
            only,
            skip,
        })
    }
    //
    // pub fn pull_actions(
    //     &self,
    //     range: Option<TextRange>,
    //     only: Vec<RuleSelector>,
    //     skip: Vec<RuleSelector>,
    //     suppression_reason: Option<String>,
    // ) -> Result<PullActionsResult, WorkspaceError> {
    //     self.workspace.pull_actions(PullActionsParams {
    //         path: self.path.clone(),
    //         range,
    //         only,
    //         skip,
    //         suppression_reason,
    //     })
    // }
    //
    // pub fn format_file(&self) -> Result<Printed, WorkspaceError> {
    //     self.workspace.format_file(FormatFileParams {
    //         path: self.path.clone(),
    //     })
    // }
    //
    // pub fn format_range(&self, range: TextRange) -> Result<Printed, WorkspaceError> {
    //     self.workspace.format_range(FormatRangeParams {
    //         path: self.path.clone(),
    //         range,
    //     })
    // }
    //
    // pub fn format_on_type(&self, offset: TextSize) -> Result<Printed, WorkspaceError> {
    //     self.workspace.format_on_type(FormatOnTypeParams {
    //         path: self.path.clone(),
    //         offset,
    //     })
    // }
    //
    // pub fn fix_file(
    //     &self,
    //     fix_file_mode: FixFileMode,
    //     should_format: bool,
    //     rule_categories: RuleCategories,
    //     only: Vec<RuleSelector>,
    //     skip: Vec<RuleSelector>,
    //     suppression_reason: Option<String>,
    // ) -> Result<FixFileResult, WorkspaceError> {
    //     self.workspace.fix_file(FixFileParams {
    //         path: self.path.clone(),
    //         fix_file_mode,
    //         should_format,
    //         only,
    //         skip,
    //         rule_categories,
    //         suppression_reason,
    //     })
    // }
    //
    // pub fn organize_imports(&self) -> Result<OrganizeImportsResult, WorkspaceError> {
    //     self.workspace.organize_imports(OrganizeImportsParams {
    //         path: self.path.clone(),
    //     })
    // }
    //
    // pub fn search_pattern(&self, pattern: &PatternId) -> Result<SearchResults, WorkspaceError> {
    //     self.workspace.search_pattern(SearchPatternParams {
    //         path: self.path.clone(),
    //         pattern: pattern.clone(),
    //     })
    // }
}

impl<W: Workspace + ?Sized> Drop for FileGuard<'_, W> {
    fn drop(&mut self) {
        self.workspace
            .close_file(CloseFileParams {
                path: self.path.clone(),
            })
            // `close_file` can only error if the file was already closed, in
            // this case it's generally better to silently matcher the error
            // than panic (especially in a drop handler)
            .ok();
    }
}
