// pub conn: &'a PgPool,
// pub sql: &'a str,
// pub ast: &'a pg_query_ext::NodeEnum,
// pub tree: Option<&'a tree_sitter::Tree>,
use dashmap::DashMap;
use pg_diagnostics::serde::Diagnostic as SDiagnostic;
use pg_typecheck::TypecheckDiagnostic;

use super::document::Statement;

pub struct TypecheckStore {
    diagnostics: DashMap<Statement, TypecheckDiagnostic>,
}

impl TypecheckStore {
    pub fn new() -> TypecheckStore {
        TypecheckStore {
            diagnostics: DashMap::new(),
        }
    }

    pub fn get_diagnostics(&self, stmt: &Statement) -> Vec<pg_diagnostics::serde::Diagnostic> {
        self.diagnostics
            .get(stmt)
            .map_or_else(Vec::new, |err| vec![SDiagnostic::new(err.value().clone())])
    }
}
