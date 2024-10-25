use std::sync::Arc;

use lsp_types::Range;
use pg_base_db::PgLspPath;
use pg_diagnostics::Diagnostic;
use pg_workspace::Workspace;
use text_size::TextRange;
use tokio::sync::RwLock;

use crate::{db_connection::DbConnection, utils::line_index_ext::LineIndexExt};

pub struct WorkspaceHandler {
    db: RwLock<Option<DbConnection>>,
    ide: Arc<RwLock<Workspace>>,
}

impl WorkspaceHandler {
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

    pub async fn run_stmt(&self, stmt: String) -> anyhow::Result<u64> {
        let db = self.db.read().await;
        db.as_ref()
            .expect("No Db Connection")
            .run_stmt(stmt)
            .await
            .map(|pg_query_result| pg_query_result.rows_affected())
    }

    pub async fn get_diagnostics(&self, path: PgLspPath) -> Vec<(Diagnostic, Range)> {
        let ide = self.ide.read().await;

        // make sure there are documents at the provided path before
        // trying to collect diagnostics.
        let doc = ide.documents.get(&path);
        if doc.is_none() {
            return vec![];
        }

        self.ide
            .read()
            .await
            .diagnostics(&path)
            .into_iter()
            .map(|d| {
                let range = doc.as_ref().unwrap().line_index.line_col_lsp_range(d.range).unwrap();
                (d, range)
            })
            .collect()
    }
}
