use std::{fs, panic::RefUnwindSafe, path::Path, sync::RwLock};

use change::StatementChange;
use dashmap::DashMap;
use pg_fs::{ConfigName, PgLspPath};
use store::{Document, StatementRef};

use crate::{settings::{Settings, SettingsHandleMut, SettingsHandle}, WorkspaceError};

use super::{GetFileContentParams, IsPathIgnoredParams, OpenFileParams, ServerInfo, UpdateSettingsParams, Workspace};

mod store;
mod change;

pub(super) struct WorkspaceServer {
    /// global settings object for this workspace
    settings: RwLock<Settings>,
    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<PgLspPath, Document>,
    // ts: DashMap<StatementRef, tree_sitter::TreeSitter>
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

    /// Provides a reference to the current settings
    fn settings(&self) -> SettingsHandle {
        SettingsHandle::new(&self.settings)
    }

    fn settings_mut(&self) -> SettingsHandleMut {
        SettingsHandleMut::new(&self.settings)
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored(&self, path: &Path) -> bool {
        let file_name = path.file_name().and_then(|s| s.to_str());
        // Never ignore Biome's config file regardless `include`/`ignore`
        (file_name != Some(ConfigName::pglsp_toml())) &&
            // Apply top-level `include`/`ignore
            (self.is_ignored_by_top_level_config(path))
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored_by_top_level_config(&self, path: &Path) -> bool {
        let set = self.settings();
        let settings = set.as_ref();
        let is_included = settings.files.included_files.is_empty()
            || is_dir(path)
            || settings.files.included_files.matches_path(path);
        !is_included
            || settings.files.ignored_files.matches_path(path)
            || settings.files.git_ignore.as_ref().is_some_and(|ignore| {
                // `matched_path_or_any_parents` panics if `source` is not under the gitignore root.
                // This checks excludes absolute paths that are not a prefix of the base root.
                if !path.has_root() || path.starts_with(ignore.path()) {
                    // Because Biome passes a list of paths,
                    // we use `matched_path_or_any_parents` instead of `matched`.
                    ignore
                        .matched_path_or_any_parents(path, path.is_dir())
                        .is_ignore()
                } else {
                    false
                }
            })
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
                params.workspace_directory,
                params.vcs_base_path,
                params.gitignore_matches.as_slice()
            )?;

        Ok(())
    }

    /// Add a new file to the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        tracing::info!("Opening file: {:?}", params.path);
        self.documents.insert(
            params.path.clone(),
            Document::new(params.path, params.content, params.version)
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
        let mut doc = self
            .documents
            .entry(params.path.clone())
            .or_insert(Document::new(params.path.clone(), "".to_string(), params.version));

        for c in &doc.apply_file_change(&params) {
            match c {
                StatementChange::Added(s) => {
                    // self.tree_sitter.add_statement(s);
                    // self.pg_query.add_statement(s);
                    //
                    // self.changed_stmts.insert(s.to_owned());
                }
                StatementChange::Deleted(s) => {
                    // self.tree_sitter.remove_statement(s);
                    // self.pg_query.remove_statement(s);
                    // self.linter.clear_statement_violations(s);
                    // self.typechecker.clear_statement_errors(s);
                    //
                    // self.changed_stmts.insert(s.to_owned());
                }
                StatementChange::Modified(s) => {
                    // self.tree_sitter.modify_statement(s);
                    // self.pg_query.modify_statement(s);
                    // self.linter.clear_statement_violations(&s.statement);
                    // self.typechecker.clear_statement_errors(&s.statement);
                    //
                    // self.changed_stmts.remove(&s.statement);
                    // self.changed_stmts.insert(s.new_statement().to_owned());
                }
            }
        }

        Ok(())
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        let document = self
            .documents
            .get(&params.path)
            .ok_or(WorkspaceError::not_found())?;
        Ok(document.content.clone())
    }

    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        Ok(self.is_ignored(params.pglsp_path.as_path()))
    }
}

/// Returns `true` if `path` is a directory or
/// if it is a symlink that resolves to a directory.
fn is_dir(path: &Path) -> bool {
    path.is_dir() || (path.is_symlink() && fs::read_link(path).is_ok_and(|path| path.is_dir()))
}

