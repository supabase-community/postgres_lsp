use std::{fs, future::Future, panic::RefUnwindSafe, path::Path, sync::RwLock};

use change::StatementChange;
use dashmap::{DashMap, DashSet};
use document::{Document, StatementRef};
use pg_diagnostics::{serde::Diagnostic as SDiagnostic, Diagnostic, DiagnosticExt, Severity};
use pg_fs::{ConfigName, PgLspPath};
use pg_query::PgQueryStore;
use pg_schema_cache::SchemaCache;
use sqlx::PgPool;
use std::sync::LazyLock;
use store::Store;
use tokio::runtime::Runtime;
use tracing::info;
use tree_sitter::TreeSitterStore;

use crate::{
    settings::{Settings, SettingsHandle, SettingsHandleMut},
    workspace::PullDiagnosticsResult,
    WorkspaceError,
};

use super::{
    GetFileContentParams, IsPathIgnoredParams, OpenFileParams, ServerInfo, UpdateSettingsParams,
    Workspace,
};

mod change;
mod document;
mod pg_query;
mod store;
mod tree_sitter;

/// Simple helper to manage the db connection and the associated connection string
#[derive(Default)]
struct DbConnection {
    pool: Option<PgPool>,
    connection_string: Option<String>,
}

// Global Tokio Runtime
static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("Failed to create Tokio runtime"));

impl DbConnection {
    pub(crate) fn get_pool(&self) -> Option<PgPool> {
        self.pool.clone()
    }

    pub(crate) fn set_connection(&mut self, connection_string: &str) -> Result<(), WorkspaceError> {
        if self.connection_string.is_none()
            || self.connection_string.as_ref().unwrap() != connection_string
        {
            self.connection_string = Some(connection_string.to_string());
            self.pool = Some(PgPool::connect_lazy(connection_string)?);
        }

        Ok(())
    }
}

pub(super) struct WorkspaceServer {
    /// global settings object for this workspace
    settings: RwLock<Settings>,

    /// Stores the schema cache for this workspace
    schema_cache: RwLock<SchemaCache>,

    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<PgLspPath, Document>,

    tree_sitter: TreeSitterStore,
    pg_query: PgQueryStore,

    /// Stores the statements that have changed since the last analysis
    changed_stmts: DashSet<StatementRef>,

    connection: RwLock<DbConnection>,
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
            tree_sitter: TreeSitterStore::new(),
            pg_query: PgQueryStore::new(),
            changed_stmts: DashSet::default(),
            schema_cache: RwLock::default(),
            connection: RwLock::default(),
        }
    }

    /// Provides a reference to the current settings
    fn settings(&self) -> SettingsHandle {
        SettingsHandle::new(&self.settings)
    }

    fn settings_mut(&self) -> SettingsHandleMut {
        SettingsHandleMut::new(&self.settings)
    }

    fn refresh_db_connection(&self) -> Result<(), WorkspaceError> {
        let s = self.settings();

        let connection_string = s.as_ref().db.to_connection_string();
        self.connection
            .write()
            .unwrap()
            .set_connection(&connection_string)?;

        self.reload_schema_cache()?;

        Ok(())
    }

    fn reload_schema_cache(&self) -> Result<(), WorkspaceError> {
        tracing::info!("Reloading schema cache");
        // TODO return error if db connection is not available
        if let Some(c) = self.connection.read().unwrap().get_pool() {
            let schema_cache = run_async(async move {
                // TODO load should return a Result
                SchemaCache::load(&c).await
            })?;

            let mut cache = self.schema_cache.write().unwrap();
            *cache = schema_cache;
        } else {
            let mut cache = self.schema_cache.write().unwrap();
            *cache = SchemaCache::default();
        }
        tracing::info!("Schema cache reloaded");

        Ok(())
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
    #[tracing::instrument(level = "trace", skip(self))]
    fn refresh_schema_cache(&self) -> Result<(), WorkspaceError> {
        self.reload_schema_cache()
    }

    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "trace", skip(self))]
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        tracing::info!("Updating settings in workspace");

        self.settings_mut().as_mut().merge_with_configuration(
            params.configuration,
            params.workspace_directory,
            params.vcs_base_path,
            params.gitignore_matches.as_slice(),
        )?;

        self.refresh_db_connection()?;

        tracing::info!("Updated settings in workspace");

        Ok(())
    }

    /// Add a new file to the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        tracing::info!("Opening file: {:?}", params.path);

        let doc = Document::new(params.path.clone(), params.content, params.version);

        doc.statements.iter().for_each(|s| {
            let stmt = doc.statement(s);
            self.tree_sitter.add_statement(&stmt);
            self.pg_query.add_statement(&stmt);
        });

        self.documents.insert(params.path, doc);

        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: super::CloseFileParams) -> Result<(), crate::WorkspaceError> {
        let (_, doc) = self
            .documents
            .remove(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        for stmt in doc.statement_refs() {
            self.tree_sitter.remove_statement(&stmt);
        }

        Ok(())
    }

    /// Change the content of an open file
    fn change_file(&self, params: super::ChangeFileParams) -> Result<(), WorkspaceError> {
        let mut doc = self
            .documents
            .entry(params.path.clone())
            .or_insert(Document::new(
                params.path.clone(),
                "".to_string(),
                params.version,
            ));

        tracing::info!("Changing file: {:?}", params.path);

        for c in &doc.apply_file_change(&params) {
            match c {
                StatementChange::Added(s) => {
                    tracing::info!("Adding statement: {:?}", s);
                    self.tree_sitter.add_statement(s);
                    self.pg_query.add_statement(s);

                    self.changed_stmts.insert(s.ref_.to_owned());
                }
                StatementChange::Deleted(s) => {
                    tracing::info!("Deleting statement: {:?}", s);
                    self.tree_sitter.remove_statement(s);
                    self.pg_query.remove_statement(s);

                    self.changed_stmts.remove(s);
                }
                StatementChange::Modified(s) => {
                    tracing::info!("Modifying statement: {:?}", s);
                    self.tree_sitter.modify_statement(s);
                    self.pg_query.modify_statement(s);

                    self.changed_stmts.remove(&s.old.ref_);
                    self.changed_stmts.insert(s.new_ref.to_owned());
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

    fn pull_diagnostics(
        &self,
        params: super::PullDiagnosticsParams,
    ) -> Result<super::PullDiagnosticsResult, WorkspaceError> {
        // get all statements form the requested document and pull diagnostics out of every
        // source
        let doc = self
            .documents
            .get(&params.path)
            .ok_or(WorkspaceError::not_found())?;

        let diagnostics: Vec<SDiagnostic> = doc
            .statement_refs_with_ranges()
            .iter()
            .flat_map(|(stmt, r)| {
                let mut stmt_diagnostics = vec![];

                stmt_diagnostics.extend(self.pg_query.pull_diagnostics(stmt));

                stmt_diagnostics
                    .into_iter()
                    .map(|d| {
                        SDiagnostic::new(
                            d.with_file_path(params.path.as_path().display().to_string())
                                .with_file_span(r),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let errors = diagnostics
            .iter()
            .filter(|d| d.severity() == Severity::Error)
            .count();

        info!("Pulled {:?} diagnostic(s)", diagnostics.len());
        Ok(PullDiagnosticsResult {
            diagnostics,
            errors,
            skipped_diagnostics: 0,
        })
    }
}

/// Returns `true` if `path` is a directory or
/// if it is a symlink that resolves to a directory.
fn is_dir(path: &Path) -> bool {
    path.is_dir() || (path.is_symlink() && fs::read_link(path).is_ok_and(|path| path.is_dir()))
}

/// Use this function to run async functions in the workspace, which is a sync trait called from an
/// async context.
///
/// Checkout https://greptime.com/blogs/2023-03-09-bridging-async-and-sync-rust for details.
fn run_async<F, R>(future: F) -> Result<R, WorkspaceError>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    futures::executor::block_on(async { RUNTIME.spawn(future).await.map_err(|e| e.into()) })
}
