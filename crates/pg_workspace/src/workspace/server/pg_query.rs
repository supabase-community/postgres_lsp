use std::sync::Arc;

use dashmap::DashMap;
use pg_diagnostics::serde::Diagnostic as SDiagnostic;
use pg_query_ext::diagnostics::*;

use super::{change::ModifiedStatement, document::Statement};

pub struct PgQueryStore {
    ast_db: DashMap<Statement, Arc<pg_query_ext::NodeEnum>>,
    diagnostics: DashMap<Statement, SyntaxDiagnostic>,
}

impl PgQueryStore {
    pub fn new() -> PgQueryStore {
        PgQueryStore {
            ast_db: DashMap::new(),
            diagnostics: DashMap::new(),
        }
    }

    pub fn get_ast(&self, statement: &Statement) -> Option<Arc<pg_query_ext::NodeEnum>> {
        self.ast_db.get(statement).map(|x| x.clone())
    }

    pub fn add_statement(&self, statement: &Statement, content: &str) {
        let r = pg_query_ext::parse(content);
        if let Ok(ast) = r {
            self.ast_db.insert(statement.clone(), Arc::new(ast));
        } else {
            tracing::info!("adding diagnostics");
            self.diagnostics
                .insert(statement.clone(), SyntaxDiagnostic::from(r.unwrap_err()));
        }
    }

    pub fn remove_statement(&self, statement: &Statement) {
        self.ast_db.remove(statement);
        self.diagnostics.remove(statement);
    }

    pub fn modify_statement(&self, change: &ModifiedStatement) {
        self.remove_statement(&change.old_stmt);
        self.add_statement(&change.new_stmt, &change.new_stmt_text);
    }

    pub fn get_diagnostics(&self, stmt: &Statement) -> Vec<pg_diagnostics::serde::Diagnostic> {
        self.diagnostics
            .get(stmt)
            .map_or_else(Vec::new, |err| vec![SDiagnostic::new(err.value().clone())])
    }
}
