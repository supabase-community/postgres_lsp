use base_db::{StatementChange, StatementRef};
use dashmap::DashMap;

pub struct PgQueryParser {
    ast_db: DashMap<StatementRef, parser::pg_query::NodeEnum>,
    native_diagnostics: DashMap<StatementRef, parser::pg_query::Error>,
}

impl PgQueryParser {
    pub fn new() -> PgQueryParser {
        PgQueryParser {
            ast_db: DashMap::new(),
            native_diagnostics: DashMap::new(),
        }
    }

    pub fn process_changes(&self, changes: &Vec<StatementChange>) {
        for c in changes {
            if c.change.is_none() {
                // statement was removed
                self.ast_db.remove(&c.statement);
                self.native_diagnostics.remove(&c.statement);
                continue;
            }

            if c.change.as_ref().unwrap().range.is_none() {
                // statement was added
                let r = parser::parse_sql_statement(c.statement.text.as_str());
                if r.is_ok() {
                    self.ast_db.insert(c.statement.clone(), r.unwrap());
                } else {
                    self.native_diagnostics
                        .insert(c.statement.clone(), r.unwrap_err());
                }
                continue;
            }

            // statement was changed
            self.ast_db.remove(&c.statement);
            self.native_diagnostics.remove(&c.statement);
            let r = parser::parse_sql_statement(c.statement.text.as_str());
            if r.is_ok() {
                self.ast_db.insert(c.statement.clone(), r.unwrap());
            } else {
                self.native_diagnostics
                    .insert(c.statement.clone(), r.unwrap_err());
            }
        }
    }
}
