use std::sync::Arc;

use base_db::StatementRef;
use dashmap::DashMap;
use diagnostics::{Diagnostic, Severity};
use text_size::TextRange;

pub struct Typechecker {
    errors: DashMap<StatementRef, Arc<Vec<typecheck_sqlx::TypeError>>>,
}

impl Typechecker {
    pub fn new() -> Typechecker {
        Typechecker {
            errors: DashMap::new(),
        }
    }

    pub fn clear_errors(&self) {
        self.errors.clear();
    }

    pub fn diagnostics(&self, statement: &StatementRef, at_range: TextRange) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if let Some(errs) = self.errors.get(statement) {
            diagnostics.extend(errs.iter().map(|e| Diagnostic {
                description: None,
                source: "typecheck".to_string(),
                range: e.range.map(|r| r + at_range.start()).unwrap_or(at_range),
                severity: match e.severity {
                    typecheck_sqlx::PgSeverity::Error => Severity::Error,
                    typecheck_sqlx::PgSeverity::Fatal => Severity::Error,
                    typecheck_sqlx::PgSeverity::Panic => Severity::Error,
                    typecheck_sqlx::PgSeverity::Warning => Severity::Warning,
                    typecheck_sqlx::PgSeverity::Notice => Severity::Information,
                    typecheck_sqlx::PgSeverity::Debug => Severity::Information,
                    typecheck_sqlx::PgSeverity::Info => Severity::Information,
                    typecheck_sqlx::PgSeverity::Log => Severity::Information,
                },
                message: e.message.to_owned(),
            }));
        }
        diagnostics
    }

    pub fn run_typecheck(
        &self,
        statement: &StatementRef,
        params: typecheck_sqlx::TypecheckerParams<'_>,
    ) {
        self.errors.insert(
            statement.clone(),
            Arc::new(async_std::task::block_on(typecheck_sqlx::check_sql(params))),
        );
    }

    pub fn clear_statement_errors(&self, statement: &StatementRef) {
        self.errors.remove(statement);
    }
}
