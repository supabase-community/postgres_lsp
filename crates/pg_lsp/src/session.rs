use std::{collections::HashSet, sync::Arc};

use pg_base_db::{Change, DocumentChange};
use pg_commands::{Command, ExecuteStatementCommand};
use pg_completions::CompletionParams;
use pg_fs::PgLspPath;
use pg_hover::HoverParams;
use pg_workspace::diagnostics::Diagnostic;
use pg_workspace::Workspace;
use text_size::TextSize;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{
    CodeActionOrCommand, CompletionItem, CompletionList, Hover, HoverContents, InlayHint,
    InlayHintKind, InlayHintLabel, MarkedString, Position, Range,
};

use crate::{
    db_connection::DbConnection,
    utils::{line_index_ext::LineIndexExt, to_lsp_types::to_completion_kind},
};

pub struct Session {
    db: RwLock<Option<DbConnection>>,
    ide: Arc<RwLock<Workspace>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            db: RwLock::new(None),
            ide: Arc::new(RwLock::new(Workspace::new())),
        }
    }

    #[tracing::instrument(name = "Shutting down Session", skip(self))]
    pub async fn shutdown(&self) {
        let mut db = self.db.write().await;
        let db = db.take();

        if db.is_some() {
            db.unwrap().close().await;
        }
    }

    /// `update_db_connection` will update `Self`'s database connection.
    /// If the passed-in connection string is the same that we're already connected to, it's a noop.
    /// Otherwise, it'll first open a new connection, replace `Self`'s connection, and then close
    /// the old one.
    #[tracing::instrument(name = "Updating DB Connection", skip(self))]
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

        tracing::info!("Setting up new Database connection");
        let new_db = DbConnection::new(connection_string, Arc::clone(&self.ide)).await?;
        tracing::info!("Set up new connection, trying to acquire write lock…");

        let mut current_db = self.db.write().await;
        let old_db = current_db.replace(new_db);

        if old_db.is_some() {
            tracing::info!("Dropping previous Database Connection.");
            let old_db = old_db.unwrap();
            old_db.close().await;
        }

        tracing::info!("Successfully set up new connection.");
        Ok(())
    }

    /// Runs the passed-in statement against the underlying database.
    pub async fn run_stmt(&self, stmt: String) -> anyhow::Result<u64> {
        let db = self.db.read().await;
        let pool = db.as_ref().map(|d| d.get_pool());

        let cmd = ExecuteStatementCommand::new(stmt);

        match cmd.run(pool).await {
            Err(e) => Err(e),
            Ok(res) => Ok(res.rows_affected()),
        }
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
            tracing::info!("Doc not found, path: {:?}", &path);
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
    ) -> HashSet<PgLspPath> {
        {
            let ide = self.ide.read().await;

            ide.apply_change(
                path,
                DocumentChange::new(version, vec![Change { range: None, text }]),
            );
        }

        self.recompute_and_get_changed_files().await
    }

    pub async fn recompute_and_get_changed_files(&self) -> HashSet<PgLspPath> {
        let ide = self.ide.read().await;

        let db = self.db.read().await;
        let pool = db.as_ref().map(|d| d.get_pool());

        let changed_files = ide.compute(pool);

        changed_files.into_iter().map(|p| p.document_url).collect()
    }

    pub async fn get_available_code_actions_or_commands(
        &self,
        path: PgLspPath,
        range: Range,
    ) -> Option<Vec<CodeActionOrCommand>> {
        let ide = self.ide.read().await;
        let doc = ide.documents.get(&path)?;

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
                CodeActionOrCommand::Command(tower_lsp::lsp_types::Command {
                    title,
                    command: format!("pglsp.{}", cmd.id()),
                    arguments: Some(vec![serde_json::to_value(stmt.text.clone()).unwrap()]),
                })
            })
            .collect();

        Some(actions)
    }

    pub async fn get_inlay_hints(&self, path: PgLspPath, range: Range) -> Option<Vec<InlayHint>> {
        let ide = self.ide.read().await;
        let doc = ide.documents.get(&path)?;

        let range = doc.line_index.offset_lsp_range(range)?;

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

    pub async fn get_available_completions(
        &self,
        path: PgLspPath,
        position: Position,
    ) -> Option<CompletionList> {
        let ide = self.ide.read().await;

        let doc = ide.documents.get(&path)?;
        let offset = doc.line_index.offset_lsp(position)?;
        let (range, stmt) = doc.statement_at_offset_with_range(&offset)?;

        let schema_cache = ide.schema_cache.read().expect("No Schema Cache");

        let completion_items: Vec<CompletionItem> = pg_completions::complete(CompletionParams {
            position: offset - range.start() - TextSize::from(1),
            text: stmt.text.clone(),
            tree: ide
                .tree_sitter
                .tree(&stmt)
                .as_ref().map(|t| t.as_ref()),
            schema: &schema_cache,
        })
        .into_iter()
        .map(|item| CompletionItem {
            label: item.label,
            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                description: Some(item.description),
                detail: None,
            }),
            kind: Some(to_completion_kind(item.kind)),
            detail: None,
            documentation: None,
            deprecated: None,
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: None,
            insert_text_format: None,
            insert_text_mode: None,
            text_edit: None,
            additional_text_edits: None,
            commit_characters: None,
            data: None,
            tags: None,
            command: None,
        })
        .collect();

        Some(CompletionList {
            is_incomplete: false,
            items: completion_items,
        })
    }

    pub async fn get_available_hover_diagnostics(
        &self,
        path: PgLspPath,
        position: Position,
    ) -> Option<Hover> {
        let ide = self.ide.read().await;
        let doc = ide.documents.get(&path)?;

        let offset = doc.line_index.offset_lsp(position)?;

        let (range, stmt) = doc.statement_at_offset_with_range(&offset)?;
        let range_start = range.start();
        let hover_range = doc.line_index.line_col_lsp_range(range);

        let schema_cache = ide.schema_cache.read().expect("No Schema Cache");

        ::pg_hover::hover(HoverParams {
            position: offset - range_start,
            source: stmt.text.as_str(),
            enriched_ast: ide
                .pg_query
                .enriched_ast(&stmt)
                .as_ref()
                .map(|x| x.as_ref()),
            tree: ide.tree_sitter.tree(&stmt).as_ref().map(|x| x.as_ref()),
            schema_cache: schema_cache.clone(),
        })
        .map(|hover| Hover {
            contents: HoverContents::Scalar(MarkedString::String(hover.content)),
            range: hover_range,
        })
    }
}
