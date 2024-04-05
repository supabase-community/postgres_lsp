mod tree_sitter;

use base_db::{Document, DocumentChange, PgLspPath};
use dashmap::DashMap;
use tree_sitter::TreeSitterParser;

pub struct IDE {
    documents: DashMap<PgLspPath, Document>,

    tree_sitter: TreeSitterParser,
}

impl IDE {
    pub fn new() -> IDE {
        IDE {
            documents: DashMap::new(),
            tree_sitter: TreeSitterParser::new(),
        }
    }

    /// Applies changes to the current state of the world
    pub fn apply_change(&self, url: PgLspPath, mut change: DocumentChange) {
        self.documents.entry(url).and_modify(|d| change.apply(d));
        let changed_stmts = change.collect_statement_changes();

        self.tree_sitter.process_changes(&changed_stmts);
    }
}
