use std::sync::Arc;

use base_db::{ChangedStatement, StatementRef};
use dashmap::DashMap;
use diagnostics::{Diagnostic, Severity};
use text_size::TextRange;

pub struct PgQueryParser {
    ast_db: DashMap<StatementRef, Arc<sql_parser::AstNode>>,
    native_diagnostics: DashMap<StatementRef, Arc<sql_parser::NativeError>>,
    enriched_ast_db: DashMap<StatementRef, Arc<sql_parser::EnrichedAst>>,
    cst_db: DashMap<StatementRef, Arc<sql_parser::Cst>>,
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

    pub fn ast(&self, statement: &StatementRef) -> Option<Arc<sql_parser::AstNode>> {
        self.ast_db.get(statement).map(|x| x.clone())
    }

    pub fn enriched_ast(&self, statement: &StatementRef) -> Option<Arc<sql_parser::EnrichedAst>> {
        self.enriched_ast_db.get(statement).map(|x| x.clone())
    }

    pub fn cst(&self, statement: &StatementRef) -> Option<Arc<sql_parser::Cst>> {
        self.cst_db.get(statement).map(|x| x.clone())
    }

    pub fn compute_cst(&self, statement: &StatementRef) {
        if self.cst_db.contains_key(statement) {
            return;
        }

        if let Some(ast) = self.ast_db.get(statement) {
            let r = sql_parser::parse_ast(&statement.text, &ast);
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
        let r = sql_parser::parse_sql_statement(statement.text.as_str());
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
