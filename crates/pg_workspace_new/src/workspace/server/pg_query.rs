use std::sync::Arc;

use dashmap::DashMap;
use pg_diagnostics::{serde::Diagnostic as SDiagnostic, Diagnostic, MessageAndDescription};
use pg_query_ext::diagnostics::*;
use text_size::TextRange;

use super::{
    change::ChangedStatement,
    document::{Statement, StatementRef},
    store::Store,
};

pub struct PgQueryStore {
    ast_db: DashMap<StatementRef, Arc<pg_query_ext::NodeEnum>>,
    diagnostics: DashMap<StatementRef, SyntaxDiagnostic>,
}

impl PgQueryStore {
    pub fn new() -> PgQueryStore {
        PgQueryStore {
            ast_db: DashMap::new(),
            diagnostics: DashMap::new(),
        }
    }
}

impl Store<pg_query_ext::NodeEnum> for PgQueryStore {
    fn load(&self, statement: &StatementRef) -> Option<Arc<pg_query_ext::NodeEnum>> {
        self.ast_db.get(statement).map(|x| x.clone())
    }

    fn add_statement(&self, statement: &Statement) {
        let r = pg_query_ext::parse(statement.text.as_str());
        if let Ok(ast) = r {
            self.ast_db.insert(statement.ref_.clone(), Arc::new(ast));
        } else {
            tracing::info!("adding diagnostics");
            self.diagnostics.insert(
                statement.ref_.clone(),
                SyntaxDiagnostic::from(r.unwrap_err()),
            );
        }
    }

    fn remove_statement(&self, statement: &StatementRef) {
        self.ast_db.remove(statement);
        self.diagnostics.remove(statement);
    }

    fn modify_statement(&self, change: &ChangedStatement) {
        self.remove_statement(&change.old.ref_);
        self.add_statement(&change.new_statement());
    }

    fn diagnostics(&self, stmt: &StatementRef) -> Vec<pg_diagnostics::serde::Diagnostic> {
        self.diagnostics
            .get(stmt)
            .map_or_else(Vec::new, |err| vec![SDiagnostic::new(err.value().clone())])
    }
}
