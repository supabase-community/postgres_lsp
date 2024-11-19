use std::{panic::RefUnwindSafe, sync::RwLock};

use dashmap::DashMap;
use pg_fs::PgLspPath;
use store::Document;

use crate::{settings::{Settings, SettingsHandleMut}, WorkspaceError};

use super::{OpenFileParams, ServerInfo, UpdateSettingsParams, Workspace};

mod store;

pub(super) struct WorkspaceServer {
    /// global settings object for this workspace
    settings: RwLock<Settings>,
    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<PgLspPath, Document>,
}


/// The `Workspace` object is long-lived, so we want it to be able to cross
/// unwind boundaries.
/// In return, we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead too hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

impl WorkspaceServer {
    /// Create a new [Workspace]
    ///
    /// This is implemented as a crate-private method instead of using
    /// [Default] to disallow instances of [Workspace] from being created
    /// outside a [crate::App]
    pub(crate) fn new() -> Self {
        Self {
            settings: RwLock::default(),
            documents: DashMap::default(),
        }
    }

    fn settings_mut(&self) -> SettingsHandleMut {
        SettingsHandleMut::new(&self.settings)
    }
}

impl Workspace for WorkspaceServer {
    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "trace", skip(self))]
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        let mut settings = self.settings_mut();
       settings
            .as_mut()
            .merge_with_configuration(
                params.configuration,
                params.workspace_directory
            )?;

        Ok(())
    }

    /// Add a new file to the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.documents.insert(
            params.path.clone(),
            Document::new(params.content, params.version)
        );

        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: super::CloseFileParams) -> Result<(), crate::WorkspaceError> {
        self.documents
            .remove(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        Ok(())
    }

    /// Change the content of an open file
    fn change_file(&self, params: super::ChangeFileParams) -> Result<(), WorkspaceError> {
        // get statement ids from document and apply changes and update the store for parse results
        todo!()
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }
}
