use std::sync::Arc;

use dashmap::DashMap;

use super::{
    change::ChangedStatement,
    document::{Statement, StatementRef},
    store::Store,
};

pub struct PgQueryStore {
    ast_db: DashMap<StatementRef, Arc<pg_query_ext::NodeEnum>>,
    native_diagnostics: DashMap<StatementRef, Arc<pg_query_ext::Error>>,
}

impl PgQueryStore {
    pub fn new() -> PgQueryStore {
        PgQueryStore {
            ast_db: DashMap::new(),
            native_diagnostics: DashMap::new(),
        }
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
            self.native_diagnostics
                .insert(statement.ref_.clone(), Arc::new(r.unwrap_err()));
        }
    }

    fn remove_statement(&self, statement: &StatementRef) {
        self.ast_db.remove(statement);
        self.native_diagnostics.remove(statement);
    }

    fn modify_statement(&self, change: &ChangedStatement) {
        self.remove_statement(&change.old.ref_);
        self.add_statement(&change.new_statement());
    }
}
