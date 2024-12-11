use std::sync::Arc;

use super::{
    change::ChangedStatement,
    document::{Statement, StatementRef},
};

pub(crate) trait Store<T> {
    #[allow(dead_code)]
    fn fetch(&self, statement: &StatementRef) -> Option<Arc<T>>;

    fn add_statement(&self, statement: &Statement);

    fn remove_statement(&self, statement: &StatementRef);

    fn modify_statement(&self, change: &ChangedStatement);
}
