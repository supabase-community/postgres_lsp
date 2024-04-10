use base_db::{ChangedStatement, StatementRef};
use dashmap::DashMap;

pub struct PgQueryParser {
    ast_db: DashMap<StatementRef, sql_parser::AstNode>,
    native_diagnostics: DashMap<StatementRef, sql_parser::NativeError>,
    enriched_ast_db: DashMap<StatementRef, sql_parser::EnrichedAst>,
    cst_db: DashMap<StatementRef, sql_parser::Cst>,
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

    pub fn add_statement(&self, statement: &StatementRef) {
        let r = sql_parser::parse_sql_statement(statement.text.as_str());
        if r.is_ok() {
            self.ast_db.insert(statement.clone(), r.unwrap());
        } else {
            self.native_diagnostics
                .insert(statement.clone(), r.unwrap_err());
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
        self.add_statement(&change.statement);
    }
}
