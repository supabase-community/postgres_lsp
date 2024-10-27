use std::{collections::HashSet, sync::Arc};

use pg_base_db::{Change, DocumentChange, PgLspPath};
use pg_commands::ExecuteStatementCommand;
use pg_diagnostics::Diagnostic;
use pg_workspace::Workspace;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{CodeAction, InlayHint, Range};

use crate::{db_connection::DbConnection, utils::line_index_ext::LineIndexExt};

pub struct Session {
    db: RwLock<Option<DbConnection>>,
    ide: Arc<RwLock<Workspace>>,
}

impl Session {
    pub fn new() -> Self {
        let ide = Arc::new(RwLock::new(Workspace::new()));
        Self {
            db: RwLock::new(None),
            ide,
        }
    }

    /// `update_db_connection` will update `Self`'s database connection.
    /// If the passed-in connection string is the same that we're already connected to, it's a noop.
    /// Otherwise, it'll first open a new connection, replace `Self`'s connection, and then close
    /// the old one.
    pub async fn change_db(&self, connection_string: String) -> anyhow::Result<()> {
        if self
            .db
            .read()
            .await
            .as_ref()
            // if the connection is already connected to the same database, do nothing
            .is_some_and(|c| c.connected_to(&connection_string))
        {
            return Ok(());
        }

        let mut db = DbConnection::new(connection_string).await?;

        let ide = self.ide.clone();
        db.listen_for_schema_updates(move |schema| {
            let _guard = ide.blocking_write().set_schema_cache(schema);
        });

        let mut current_db = self.db.blocking_write();
        let old_db = current_db.replace(db);

        if old_db.is_some() {
            let old_db = old_db.unwrap();
            old_db.close().await;
        }

        Ok(())
    }

    /// Runs the passed-in statement against the underlying database.
    pub async fn run_stmt(&self, stmt: String) -> anyhow::Result<u64> {
        let db = self.db.read().await;
        let pool = db.map(|d| d.get_pool());

        let cmd = ExecuteStatementCommand::new(stmt);

        cmd.run(pool).await
    }

    pub async fn on_file_closed(&self, path: PgLspPath) {
        let ide = self.ide.read().await;
        ide.remove_document(path);
    }

    pub async fn get_diagnostics(&self, path: PgLspPath) -> Vec<(Diagnostic, Range)> {
        let ide = self.ide.read().await;

        // make sure there are documents at the provided path before
        // trying to collect diagnostics.
        let doc = ide.documents.get(&path);
        if doc.is_none() {
            return vec![];
        }

        ide.diagnostics(&path)
            .into_iter()
            .map(|d| {
                let range = doc
                    .as_ref()
                    .unwrap()
                    .line_index
                    .line_col_lsp_range(d.range)
                    .unwrap();
                (d, range)
            })
            .collect()
    }

    pub async fn apply_doc_changes(
        &self,
        path: PgLspPath,
        version: i32,
        text: String,
    ) -> HashSet<String> {
        {
            let ide = self.ide.read().await;

            let doc = ide.documents.get(&path);
            if doc.is_none() {
                return HashSet::new();
            }

            ide.apply_change(
                path,
                DocumentChange::new(version, vec![Change { range: None, text }]),
            );
        }

        self.recompute_and_get_changed_files()
    }

    pub async fn recompute_and_get_changed_files(&self) -> HashSet<String> {
        let ide = self.ide.read().await;

        let db = self.db.read().await;
        let pool = db.as_ref().map(|d| d.get_pool());

        let changed_files = ide.compute(pool);

        changed_files
            .into_iter()
            .map(|f| f.document_url.to_string_lossy().to_string())
            .collect()
    }

    pub async fn get_available_code_actions(
        &self,
        path: PgLspPath,
        range: Range,
    ) -> Option<Vec<CodeAction>> {
        let ide = self.ide.read().await;
        let doc = ide.documents.get(&path);
        if doc.is_none() {
            return None;
        }

        let db = self.db.read().await;
        if db.is_none() {
            return None;
        }

        let doc = doc.unwrap();
        let range = doc.line_index.offset_lsp_range(range).unwrap();

        // for now, we only provide `ExcecuteStatementCommand`s.
        let actions = doc
            .statements_at_range(&range)
            .into_iter()
            .map(|stmt| {
                let cmd = ExecuteStatementCommand::command_type();
                let title = format!(
                    "Execute '{}'",
                    ExecuteStatementCommand::trim_statement(stmt.text.clone(), 50)
                );
                CodeAction {
                    title: title.clone(),
                    kind: None,
                    edit: None,
                    command: Some(Command {
                        title,
                        command: format!("pglsp.{}", cmd.id()),
                        arguments: Some(vec![serde_json::to_value(stmt.text.clone()).unwrap()]),
                    }),
                    diagnostics: None,
                    is_preferred: None,
                    disabled: None,
                    data: None,
                }
            })
            .collect();

        Some(actions)
    }

    pub async fn get_inlay_hints(&self, path: PgLspPath, range: Range) -> Option<Vec<InlayHint>> {
        let ide = self.ide.read().await;
        let doc = ide.documents.get(&path);
        if doc.is_none() {
            return None;
        }

        let doc = doc.unwrap();
        let range = doc.line_index.offset_lsp_range(range).unwrap();

        let schema_cache = ide.schema_cache.read().expect("Unable to get Schema Cache");

        let hints = doc
            .statements_at_range(&range)
            .into_iter()
            .flat_map(|stmt| {
                ::pg_inlay_hints::inlay_hints(::pg_inlay_hints::InlayHintsParams {
                    ast: ide.pg_query.ast(&stmt).as_ref().map(|x| x.as_ref()),
                    enriched_ast: ide
                        .pg_query
                        .enriched_ast(&stmt)
                        .as_ref()
                        .map(|x| x.as_ref()),
                    tree: ide.tree_sitter.tree(&stmt).as_ref().map(|x| x.as_ref()),
                    cst: ide.pg_query.cst(&stmt).as_ref().map(|x| x.as_ref()),
                    schema_cache: &schema_cache,
                })
            })
            .map(|hint| InlayHint {
                position: doc.line_index.line_col_lsp(hint.offset).unwrap(),
                label: match hint.content {
                    pg_inlay_hints::InlayHintContent::FunctionArg(arg) => {
                        InlayHintLabel::String(match arg.name {
                            Some(name) => format!("{} ({})", name, arg.type_name),
                            None => arg.type_name.clone(),
                        })
                    }
                },
                kind: match hint.content {
                    pg_inlay_hints::InlayHintContent::FunctionArg(_) => {
                        Some(InlayHintKind::PARAMETER)
                    }
                },
                text_edits: None,
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            })
            .collect();

        Some(hints)
    }
}
