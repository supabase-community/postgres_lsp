use std::sync::Arc;

use dashmap::DashMap;
use pg_base_db::{ChangedStatement, StatementRef};
use crate::{Diagnostic, Severity};
use text_size::TextRange;

pub struct PgQueryParser {
    ast_db: DashMap<StatementRef, Arc<pg_query_ext::NodeEnum>>,
    native_diagnostics: DashMap<StatementRef, Arc<pg_query_ext::Error>>,
    enriched_ast_db: DashMap<StatementRef, Arc<pg_syntax::AST>>,
    cst_db: DashMap<StatementRef, Arc<pg_syntax::CST>>,
}

impl PgQueryParser {
    pub fn new() -> PgQueryParser {
        PgQueryParser {
            ast_db: DashMap::new(),
            native_diagnostics: DashMap::new(),
            enriched_ast_db: DashMap::new(),
            cst_db: DashMap::new(),
        }
    }

    pub fn ast(&self, statement: &StatementRef) -> Option<Arc<pg_query_ext::NodeEnum>> {
        self.ast_db.get(statement).map(|x| x.clone())
    }

    pub fn enriched_ast(&self, statement: &StatementRef) -> Option<Arc<pg_syntax::AST>> {
        self.enriched_ast_db.get(statement).map(|x| x.clone())
    }

    pub fn cst(&self, statement: &StatementRef) -> Option<Arc<pg_syntax::CST>> {
        self.cst_db.get(statement).map(|x| x.clone())
    }

    pub fn compute_cst(&self, statement: &StatementRef) {
        if self.cst_db.contains_key(statement) {
            return;
        }

        if let Some(ast) = self.ast_db.get(statement) {
            let r = pg_syntax::parse_syntax(&statement.text, &ast);
            self.cst_db.insert(statement.clone(), Arc::new(r.cst));
            self.enriched_ast_db
                .insert(statement.clone(), Arc::new(r.ast));
        }
    }

    pub fn diagnostics(&self, statement: &StatementRef, at_range: TextRange) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if let Some(err) = self.native_diagnostics.get(statement) {
            diagnostics.push(Diagnostic {
                description: None,
                source: "pg_query".to_string(),
                range: at_range,
                severity: Severity::Error,
                message: err.to_string(),
            });
        }
        diagnostics
    }

    pub fn add_statement(&self, statement: &StatementRef) {
        let r = pg_query_ext::parse(statement.text.as_str());
        if r.is_ok() {
            self.ast_db.insert(statement.clone(), Arc::new(r.unwrap()));
        } else {
            self.native_diagnostics
                .insert(statement.clone(), Arc::new(r.unwrap_err()));
        }
    }

    pub fn remove_statement(&self, statement: &StatementRef) {
        self.ast_db.remove(statement);
        self.native_diagnostics.remove(statement);
        self.enriched_ast_db.remove(statement);
        self.cst_db.remove(statement);
    }

    pub fn modify_statement(&self, change: &ChangedStatement) {
        self.remove_statement(&change.statement);
        self.add_statement(&change.new_statement());
    }
}
