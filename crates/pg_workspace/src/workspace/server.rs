use std::{fs, panic::RefUnwindSafe, path::Path, sync::RwLock};

use analyser::AnalyserVisitorBuilder;
use async_helper::run_async;
use change::StatementChange;
use dashmap::{DashMap, DashSet};
use db_connection::DbConnection;
use document::{Document, Statement};
use futures::{stream, StreamExt};
use pg_analyse::{AnalyserOptions, AnalysisFilter};
use pg_analyser::{Analyser, AnalyserConfig, AnalyserContext};
use pg_diagnostics::{serde::Diagnostic as SDiagnostic, Diagnostic, DiagnosticExt, Severity};
use pg_fs::{ConfigName, PgLspPath};
use pg_query::PgQueryStore;
use pg_typecheck::TypecheckParams;
use schema_cache_manager::SchemaCacheManager;
use tracing::info;
use tree_sitter::TreeSitterStore;

use crate::{
    configuration::to_analyser_rules,
    settings::{Settings, SettingsHandle, SettingsHandleMut},
    workspace::PullDiagnosticsResult,
    WorkspaceError,
};

use super::{
    GetFileContentParams, IsPathIgnoredParams, OpenFileParams, ServerInfo, UpdateSettingsParams,
    Workspace,
};

mod analyser;
mod async_helper;
mod change;
mod db_connection;
mod document;
mod migration;
mod pg_query;
mod schema_cache_manager;
mod tree_sitter;

pub(super) struct WorkspaceServer {
    /// global settings object for this workspace
    settings: RwLock<Settings>,

    /// Stores the schema cache for this workspace
    schema_cache: SchemaCacheManager,

    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<PgLspPath, Document>,

    tree_sitter: TreeSitterStore,
    pg_query: PgQueryStore,

    /// Stores the statements that have changed since the last analysis
    changed_stmts: DashSet<Statement>,

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
            schema_cache: SchemaCacheManager::default(),
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

    fn is_ignored_by_migration_config(&self, path: &Path) -> bool {
        let set = self.settings();
        set.as_ref()
            .migrations
            .as_ref()
            .and_then(|migration_settings| {
                let ignore_before = migration_settings.after.as_ref()?;
                let migrations_dir = migration_settings.path.as_ref()?;
                let migration = migration::get_migration(path, migrations_dir)?;

                Some(&migration.timestamp <= ignore_before)
            })
            .unwrap_or(false)
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored(&self, path: &Path) -> bool {
        let file_name = path.file_name().and_then(|s| s.to_str());
        // Never ignore PGLSP's config file regardless `include`/`ignore`
        (file_name != Some(ConfigName::pglsp_toml())) &&
            // Apply top-level `include`/`ignore
            (self.is_ignored_by_top_level_config(path) || self.is_ignored_by_migration_config(path))
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
                    // Because PGLSP passes a list of paths,
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
        tracing::info!("Updating settings in workspace");

        self.settings_mut().as_mut().merge_with_configuration(
            params.configuration,
            params.workspace_directory,
            params.vcs_base_path,
            params.gitignore_matches.as_slice(),
        )?;

        tracing::info!("Updated settings in workspace");

        if !params.skip_db {
            self.connection
                .write()
                .unwrap()
                .set_conn_settings(&self.settings().as_ref().db);
        }

        tracing::info!("Updated Db connection settings");

        Ok(())
    }

    /// Add a new file to the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        tracing::info!("Opening file: {:?}", params.path);

        let doc = Document::new(params.path.clone(), params.content, params.version);

        doc.iter_statements_with_text().for_each(|(stmt, content)| {
            self.tree_sitter.add_statement(&stmt, content);
            self.pg_query.add_statement(&stmt, content);
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

        for stmt in doc.iter_statements() {
            self.tree_sitter.remove_statement(&stmt);
            self.pg_query.remove_statement(&stmt);
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
                StatementChange::Added(added) => {
                    tracing::info!("Adding statement: {:?}", added);
                    self.tree_sitter.add_statement(&added.stmt, &added.text);
                    self.pg_query.add_statement(&added.stmt, &added.text);

                    self.changed_stmts.insert(added.stmt.clone());
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

                    self.changed_stmts.remove(&s.old_stmt);
                    self.changed_stmts.insert(s.new_stmt.clone());
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

        let settings = self.settings();

        // create analyser for this run
        // first, collect enabled and disabled rules from the workspace settings
        let (enabled_rules, disabled_rules) = AnalyserVisitorBuilder::new(settings.as_ref())
            .with_linter_rules(&params.only, &params.skip)
            .finish();
        // then, build a map that contains all options
        let options = AnalyserOptions {
            rules: to_analyser_rules(settings.as_ref()),
        };
        // next, build the analysis filter which will be used to match rules
        let filter = AnalysisFilter {
            categories: params.categories,
            enabled_rules: Some(enabled_rules.as_slice()),
            disabled_rules: &disabled_rules,
        };
        // finally, create the analyser that will be used during this run
        let analyser = Analyser::new(AnalyserConfig {
            options: &options,
            filter,
        });

        let mut diagnostics: Vec<SDiagnostic> = vec![];

        let maybe_pool = self
            .connection
            .read()
            .expect("DbConnection RwLock panicked")
            .get_pool();

        let skipped_db_checks = maybe_pool.is_none();
        if let Some(pool) = maybe_pool {
            let typecheck_params: Vec<_> = doc
                .iter_statements_with_text_and_range()
                .map(|(stmt, range, text)| {
                    let ast = self.pg_query.get_ast(&stmt);
                    let tree = self.tree_sitter.get_parse_tree(&stmt);
                    (text.to_string(), ast, tree, *range)
                })
                .collect();

            // run diagnostics for each statement in parallel if its mostly i/o work
            let path_clone = params.path.clone();
            let async_results = run_async(async move {
                stream::iter(typecheck_params)
                    .map(|(text, ast, tree, range)| {
                        let pool = pool.clone();
                        let path = path_clone.clone();
                        async move {
                            if let Some(ast) = ast {
                                pg_typecheck::check_sql(TypecheckParams {
                                    conn: &pool,
                                    sql: &text,
                                    ast: &ast,
                                    tree: tree.as_deref(),
                                })
                                .await
                                .map(|d| {
                                    let r = d.location().span.map(|span| span + range.start());

                                    d.with_file_path(path.as_path().display().to_string())
                                        .with_file_span(r.unwrap_or(range))
                                })
                            } else {
                                None
                            }
                        }
                    })
                    .buffer_unordered(10)
                    .collect::<Vec<_>>()
                    .await
            })?;

            for result in async_results.into_iter().flatten() {
                diagnostics.push(SDiagnostic::new(result));
            }
        }

        diagnostics.extend(doc.iter_statements_with_range().flat_map(|(stmt, r)| {
            let mut stmt_diagnostics = self.pg_query.get_diagnostics(&stmt);

            let ast = self.pg_query.get_ast(&stmt);

            if let Some(ast) = ast {
                stmt_diagnostics.extend(
                    analyser
                        .run(AnalyserContext { root: &ast })
                        .into_iter()
                        .map(SDiagnostic::new)
                        .collect::<Vec<_>>(),
                );
            }

            stmt_diagnostics
                .into_iter()
                .map(|d| {
                    let severity = d
                        .category()
                        .filter(|category| category.name().starts_with("lint/"))
                        .map_or_else(
                            || d.severity(),
                            |category| {
                                settings
                                    .as_ref()
                                    .get_severity_from_rule_code(category)
                                    .unwrap_or(Severity::Warning)
                            },
                        );

                    SDiagnostic::new(
                        d.with_file_path(params.path.as_path().display().to_string())
                            .with_file_span(r)
                            .with_severity(severity),
                    )
                })
                .collect::<Vec<_>>()
        }));

        let errors = diagnostics
            .iter()
            .filter(|d| d.severity() == Severity::Error)
            .count();

        info!("Pulled {:?} diagnostic(s)", diagnostics.len());
        Ok(PullDiagnosticsResult {
            diagnostics,
            errors,
            skipped_diagnostics: 0,
            skipped_db_checks,
        })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn get_completions(
        &self,
        params: super::CompletionParams,
    ) -> Result<pg_completions::CompletionResult, WorkspaceError> {
        tracing::debug!(
            "Getting completions for file {:?} at position {:?}",
            &params.path,
            &params.position
        );

        let pool = match self.connection.read().unwrap().get_pool() {
            Some(pool) => pool,
            None => return Ok(pg_completions::CompletionResult::default()),
        };

        let doc = self
            .documents
            .get(&params.path)
            .ok_or(WorkspaceError::not_found())?;

        tracing::debug!(
            "Found the document. Looking for statement in file {:?} at position: {:?}",
            &params.path,
            &params.position
        );

        let (statement, stmt_range, text) = match doc
            .iter_statements_with_text_and_range()
            .find(|(_, r, _)| r.contains(params.position))
        {
            Some(s) => s,
            None => return Ok(pg_completions::CompletionResult::default()),
        };

        // `offset` is the position in the document,
        // but we need the position within the *statement*.
        let position = params.position - stmt_range.start();

        let tree = self.tree_sitter.get_parse_tree(&statement);

        tracing::debug!("Found the statement. We're looking for position {:?}. Statement Range {:?} to {:?}. Statement: {}", position, stmt_range.start(), stmt_range.end(), text);

        let schema_cache = self.schema_cache.load(pool)?;

        let result = pg_completions::complete(pg_completions::CompletionParams {
            position,
            schema: schema_cache.as_ref(),
            tree: tree.as_deref(),
            text: text.to_string(),
        });

        Ok(result)
    }
}

/// Returns `true` if `path` is a directory or
/// if it is a symlink that resolves to a directory.
fn is_dir(path: &Path) -> bool {
    path.is_dir() || (path.is_symlink() && fs::read_link(path).is_ok_and(|path| path.is_dir()))
}
