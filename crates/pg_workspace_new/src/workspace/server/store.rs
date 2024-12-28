use std::sync::Arc;

use super::{
    change::ChangedStatement,
    document::{Statement, StatementRef},
};

pub(crate) trait Store<T> {
    fn diagnostics(&self, _stmt: &StatementRef) -> Vec<pg_diagnostics::serde::Diagnostic> {
        Vec::new()
    }

    #[allow(dead_code)]
    fn load(&self, _stmt: &StatementRef) -> Option<Arc<T>> {
        None
    }

    fn add_statement(&self, _stmt: &Statement) {}

    fn remove_statement(&self, _stmt: &StatementRef) {}

    fn modify_statement(&self, _change: &ChangedStatement) {}
}

