use std::sync::Arc;

use base_db::StatementRef;
use dashmap::DashMap;
use diagnostics::{Diagnostic, Severity};
use text_size::TextRange;

pub struct Linter {
    violations: DashMap<StatementRef, Arc<Vec<lint::RuleViolation>>>,
}

impl Linter {
    pub fn new() -> Linter {
        Linter {
            violations: DashMap::new(),
        }
    }

    pub fn diagnostics(&self, statement: &StatementRef, at_range: TextRange) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if let Some(v) = self.violations.get(statement) {
            diagnostics.extend(v.iter().flat_map(|v| {
                v.messages.iter().map(|m| Diagnostic {
                    description: None,
                    source: "lint".to_string(),
                    range: v.range.map(|r| r + at_range.start()).unwrap_or(at_range),
                    severity: match m {
                        lint::ViolationMessage::Note(_) => Severity::Warning,
                        lint::ViolationMessage::Help(_) => Severity::Hint,
                    },
                    message: match m {
                        lint::ViolationMessage::Note(n) => n.to_owned(),
                        lint::ViolationMessage::Help(n) => n.to_owned(),
                    },
                })
            }));
        }
        diagnostics
    }

    pub fn compute_statement_violations(
        &self,
        statement: &StatementRef,
        params: lint::LinterParams<'_>,
    ) {
        self.violations
            .insert(statement.clone(), Arc::new(lint::check_sql(params)));
    }

    pub fn clear_statement_violations(&self, statement: &StatementRef) {
        self.violations.remove(statement);
    }
}
