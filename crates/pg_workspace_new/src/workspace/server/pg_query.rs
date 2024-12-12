use std::sync::Arc;

use dashmap::DashMap;
use pg_diagnostics::{serde::Diagnostic as SDiagnostic, Diagnostic, MessageAndDescription};
use text_size::TextRange;

use super::{
    change::ChangedStatement,
    document::{Statement, StatementRef},
    store::Store,
};

/// A specialized diagnostic for the libpg_query parser.
///
/// Parser diagnostics are always **errors**.
#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(category = "syntax", severity = Error)]
pub struct SyntaxDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

pub struct PgQueryStore {
    ast_db: DashMap<StatementRef, Arc<pg_query_ext::NodeEnum>>,
    diagnostics: DashMap<StatementRef, pg_query_ext::Error>,
}

impl From<&pg_query_ext::Error> for SyntaxDiagnostic {
    fn from(err: &pg_query_ext::Error) -> Self {
        SyntaxDiagnostic {
            span: None,
            message: MessageAndDescription::from(err.to_string()),
        }
    }
}

impl PgQueryStore {
    pub fn new() -> PgQueryStore {
        PgQueryStore {
            ast_db: DashMap::new(),
            diagnostics: DashMap::new(),
        }
    }

    pub fn pull_diagnostics(&self, ref_: &StatementRef) -> Vec<SDiagnostic> {
        self.diagnostics.get(ref_).map_or_else(Vec::new, |err| {
            vec![SDiagnostic::new(SyntaxDiagnostic::from(err.value()))]
        })
    }
}

impl Store<pg_query_ext::NodeEnum> for PgQueryStore {
    fn fetch(&self, statement: &StatementRef) -> Option<Arc<pg_query_ext::NodeEnum>> {
        self.ast_db.get(statement).map(|x| x.clone())
    }

    fn add_statement(&self, statement: &Statement) {
        let r = pg_query_ext::parse(statement.text.as_str());
        if let Ok(ast) = r {
            self.ast_db.insert(statement.ref_.clone(), Arc::new(ast));
        } else {
            self.diagnostics
                .insert(statement.ref_.clone(), r.unwrap_err());
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
}
