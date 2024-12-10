use std::sync::Arc;

use crate::{Diagnostic, Severity};
use dashmap::DashMap;
use pg_base_db::StatementRef;
use pg_typecheck::{check_sql, PgSeverity, TypeError, TypecheckerParams};
use text_size::TextRange;

pub struct Typechecker {
    errors: DashMap<StatementRef, Arc<Vec<TypeError>>>,
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
                    PgSeverity::Error => Severity::Error,
                    PgSeverity::Fatal => Severity::Error,
                    PgSeverity::Panic => Severity::Error,
                    PgSeverity::Warning => Severity::Warning,
                    PgSeverity::Notice => Severity::Information,
                    PgSeverity::Debug => Severity::Information,
                    PgSeverity::Info => Severity::Information,
                    PgSeverity::Log => Severity::Information,
                },
                message: e.message.to_owned(),
            }));
        }
        diagnostics
    }

    pub fn run_typecheck(&self, statement: &StatementRef, params: TypecheckerParams<'_>) {
        self.errors.insert(
            statement.clone(),
            Arc::new(async_std::task::block_on(check_sql(params))),
        );
    }

    pub fn clear_statement_errors(&self, statement: &StatementRef) {
        self.errors.remove(statement);
    }
}
