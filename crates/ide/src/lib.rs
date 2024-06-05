mod lint;
mod pg_query;
mod tree_sitter;

use std::sync::{RwLock, RwLockWriteGuard};

use base_db::{Document, DocumentChange, PgLspPath, StatementRef};
use dashmap::{DashMap, DashSet};
use lint::Linter;
use pg_query::PgQueryParser;
use schema_cache::SchemaCache;
use sqlx::PgPool;
use tracing::{event, span, Level};
use tree_sitter::TreeSitterParser;

pub struct IDE {
    pub documents: DashMap<PgLspPath, Document>,
    // Stores the statements that have changed since the last analysis
    changed_stmts: DashSet<StatementRef>,
    pub schema_cache: RwLock<SchemaCache>,

    pub tree_sitter: TreeSitterParser,
    pub pg_query: PgQueryParser,
    pub linter: Linter,
}

impl IDE {
    pub fn new() -> IDE {
        IDE {
            documents: DashMap::new(),
            schema_cache: RwLock::new(SchemaCache::new()),
            changed_stmts: DashSet::new(),

            tree_sitter: TreeSitterParser::new(),
            pg_query: PgQueryParser::new(),
            linter: Linter::new(),
        }
    }

    /// Applies changes to the current state of the world
    ///
    /// Returns a list of changed statements
    pub fn apply_change(&self, url: PgLspPath, mut change: DocumentChange) {
        let span = span!(Level::INFO, "apply_change");
        let _guard = span.enter();
        event!(Level::INFO, ?url, ?change);

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
                    self.linter.clear_statement_violations(s);
                }
                base_db::StatementChange::Modified(c) => {
                    self.tree_sitter.modify_statement(c);
                    self.pg_query.modify_statement(c);
                    self.linter.clear_statement_violations(&c.statement);
                }
            }

            self.changed_stmts.insert(c.statement().to_owned());
        }
    }

    pub fn remove_document(&self, url: PgLspPath) {
        let r = self.documents.remove(&url);
        if r.is_some() {
            let doc = r.unwrap().1;
            for stmt in doc.statement_refs() {
                self.tree_sitter.remove_statement(&stmt);
                self.pg_query.remove_statement(&stmt);
            }
        }
    }

    /// Collects all diagnostics for a given document. It does not compute them, it just collects.
    pub fn diagnostics(&self, url: &PgLspPath) -> Vec<diagnostics::Diagnostic> {
        let mut diagnostics: Vec<diagnostics::Diagnostic> = vec![];

        let doc = self.documents.get(&url);

        if doc.is_none() {
            return diagnostics;
        }

        let doc = doc.unwrap();

        for (range, stmt) in doc.statement_refs_with_range() {
            diagnostics.extend(self.pg_query.diagnostics(&stmt, range));
            diagnostics.extend(self.linter.diagnostics(&stmt, range));
        }

        diagnostics
    }

    /// Drain changed statements to kick off analysis
    pub fn compute(&self, conn: Option<PgPool>) {
        let changed: Vec<StatementRef> = self
            .changed_stmts
            .iter()
            .map(|arc| (*arc).clone())
            .collect();

        self.changed_stmts.clear();

        changed.iter().for_each(|stmt| {
            self.pg_query.compute_cst(stmt);

            if let Some(ast) = self.pg_query.ast(stmt) {
                self.linter.compute_statement_violations(
                    stmt,
                    ::lint::LinterParams {
                        ast: ast.as_ref(),
                        enriched_ast: self
                            .pg_query
                            .enriched_ast(stmt)
                            .as_ref()
                            .map(|a| a.as_ref()),
                    },
                );
            }
        });
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
    use diagnostics::Diagnostic;
    use text_size::{TextRange, TextSize};

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

    #[test]
    fn test_lint() {
        let ide = IDE::new();
        let path = PgLspPath::new("test.sql");

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                1,
                vec![Change {
                    range: None,
                    text: "select 1 from contact;\nselect 1;\nalter table test drop column id;"
                        .to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();
            assert_eq!(doc.statement_ranges.len(), 3);
            assert_eq!(
                doc.statement_ref(0).text,
                "select 1 from contact;".to_string()
            );
            assert_eq!(doc.statement_ref(1).text, "select 1;".to_string());
            assert_eq!(
                doc.statement_ref(2).text,
                "alter table test drop column id;".to_string()
            );
        }

        ide.compute();

        let d = ide.diagnostics(&path);

        assert_eq!(d.len(), 1);

        assert_eq!(
            d[0],
            Diagnostic {
                message: "Dropping a column may break existing clients.".to_string(),
                description: None,
                severity: diagnostics::Severity::Warning,
                source: "lint".to_string(),
                range: TextRange::new(TextSize::new(50), TextSize::new(64)),
            }
        );
    }

    #[test]
    fn test_apply_change_with_error() {
        let ide = IDE::new();

        let path = PgLspPath::new("test.sql");

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                1,
                vec![Change {
                    range: None,
                    text: "select 1;\nselect 2;".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();
            assert_eq!(doc.statement_ref(0).text, "select 1;".to_string());
            assert_eq!(doc.statement_ref(1).text, "select 2;".to_string());
            assert_eq!(
                doc.statement_ranges[0],
                TextRange::new(TextSize::new(0), TextSize::new(9))
            );
            assert_eq!(
                doc.statement_ranges[1],
                TextRange::new(TextSize::new(10), TextSize::new(19))
            );
        }

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                2,
                vec![Change {
                    range: Some(TextRange::new(7.into(), 8.into())),
                    text: "".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();

            assert_eq!(doc.text, "select ;\nselect 2;");
            assert_eq!(doc.statement_refs().len(), 2);
            assert_eq!(doc.statement_ref(0).text, "select ;".to_string());
            assert_eq!(doc.statement_ref(1).text, "select 2;".to_string());
            assert_eq!(
                doc.statement_ranges[0],
                TextRange::new(TextSize::new(0), TextSize::new(8))
            );
            assert_eq!(
                doc.statement_ranges[1],
                TextRange::new(TextSize::new(9), TextSize::new(18))
            );
        }

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                3,
                vec![Change {
                    range: Some(TextRange::new(7.into(), 7.into())),
                    text: "!".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();

            assert_eq!(doc.text, "select !;\nselect 2;");
            assert_eq!(doc.statement_refs().len(), 2);
            assert_eq!(
                doc.statement_ranges[0],
                TextRange::new(TextSize::new(0), TextSize::new(9))
            );
            assert_eq!(
                doc.statement_ranges[1],
                TextRange::new(TextSize::new(10), TextSize::new(19))
            );
        }

        assert_eq!(ide.diagnostics(&PgLspPath::new("test.sql")).len(), 1);

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                2,
                vec![Change {
                    range: Some(TextRange::new(7.into(), 8.into())),
                    text: "".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();

            assert_eq!(doc.text, "select ;\nselect 2;");
            assert_eq!(doc.statement_refs().len(), 2);
            assert_eq!(
                doc.statement_ranges[0],
                TextRange::new(TextSize::new(0), TextSize::new(8))
            );
            assert_eq!(
                doc.statement_ranges[1],
                TextRange::new(TextSize::new(9), TextSize::new(18))
            );
        }

        ide.apply_change(
            path.clone(),
            DocumentChange::new(
                3,
                vec![Change {
                    range: Some(TextRange::new(7.into(), 7.into())),
                    text: "1".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&path).unwrap();

            assert_eq!(doc.text, "select 1;\nselect 2;");
            assert_eq!(doc.statement_refs().len(), 2);
            assert_eq!(
                doc.statement_ranges[0],
                TextRange::new(TextSize::new(0), TextSize::new(9))
            );
            assert_eq!(
                doc.statement_ranges[1],
                TextRange::new(TextSize::new(10), TextSize::new(19))
            );
        }

        assert_eq!(ide.diagnostics(&PgLspPath::new("test.sql")).len(), 0);
    }
}
