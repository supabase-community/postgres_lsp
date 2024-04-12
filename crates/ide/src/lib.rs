mod pg_query;
mod tree_sitter;

use std::sync::{RwLock, RwLockWriteGuard};

use base_db::{Document, DocumentChange, PgLspPath};
use dashmap::DashMap;
use pg_query::PgQueryParser;
use schema_cache::SchemaCache;
use tree_sitter::TreeSitterParser;

pub struct IDE {
    documents: DashMap<PgLspPath, Document>,
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
        self.documents.entry(url).and_modify(|d| change.apply(d));
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

    pub fn set_schema_cache(&self, cache: SchemaCache) {
        let mut schema_cache: RwLockWriteGuard<SchemaCache> = self.schema_cache.write().unwrap();
        *schema_cache = cache;
    }

    // add fns here to interact with the IDE
    // e.g. get diagnostics, hover, etc.
}
