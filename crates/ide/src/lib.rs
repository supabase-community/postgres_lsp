mod pg_query;
mod tree_sitter;

use std::sync::{RwLock, RwLockWriteGuard};

use base_db::{Document, DocumentChange, DocumentParams, PgLspPath};
use dashmap::DashMap;
use pg_query::PgQueryParser;
use schema_cache::SchemaCache;
use tree_sitter::TreeSitterParser;

pub struct IDE {
    pub documents: DashMap<PgLspPath, Document>,
    schema_cache: RwLock<SchemaCache>,

    tree_sitter: TreeSitterParser,
    pg_query: PgQueryParser,
}

impl IDE {
    pub fn new() -> IDE {
        IDE {
            documents: DashMap::new(),
            schema_cache: RwLock::new(SchemaCache::new()),

            tree_sitter: TreeSitterParser::new(),
            pg_query: PgQueryParser::new(),
        }
    }

    /// Applies changes to the current state of the world
    pub fn apply_change(&self, url: PgLspPath, mut change: DocumentChange) {
        let mut doc = self
            .documents
            .entry(url.clone())
            .or_insert(Document::new_empty(url));
        change.apply(doc.value_mut());

        let changed_stmts = change.collect_statement_changes();

        for c in &changed_stmts {
            match c {
                base_db::StatementChange::Added(s) => {
                    self.tree_sitter.add_statement(s);
                    self.pg_query.add_statement(s);
                }
                base_db::StatementChange::Deleted(s) => {
                    self.tree_sitter.remove_statement(s);
                    self.pg_query.remove_statement(s);
                }
                base_db::StatementChange::Modified(c) => {
                    self.tree_sitter.modify_statement(c);
                    self.pg_query.modify_statement(c);
                }
            }
        }
    }

    /// Computes the set of diagnostics for a given document
    pub fn diagnostics(&self, url: PgLspPath) {
        // TODO: add diagnostics struct that all native diagnostics can be transformed into and
        // that is the glue between the ide and native diagnostics
        // let doc = self.documents.get(&url).unwrap();
        // let stmts = doc.stmts();
        // for stmt in stmts {
        //     let tree_sitter_diagnostics = self.tree_sitter.diagnostics(stmt);
        //     let pg_query_diagnostics = self.pg_query.diagnostics(stmt);
        //     // merge diagnostics
        // }
    }

    pub fn set_schema_cache(&self, cache: SchemaCache) {
        let mut schema_cache: RwLockWriteGuard<SchemaCache> = self.schema_cache.write().unwrap();
        *schema_cache = cache;
    }

    // add fns here to interact with the IDE
    // e.g. get diagnostics, hover, etc.
}

#[cfg(test)]
mod tests {

    use base_db::{Change, DocumentChange};

    use crate::{PgLspPath, IDE};

    #[test]
    fn test_apply_change() {
        let ide = IDE::new();

        ide.apply_change(
            PgLspPath::new("test.sql"),
            DocumentChange::new(
                1,
                vec![Change {
                    range: None,
                    text: "select 1;".to_string(),
                }],
            ),
        );
    }
}
