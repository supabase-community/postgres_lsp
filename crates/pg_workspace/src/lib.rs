mod lint;
mod pg_query;
mod tree_sitter;
mod typecheck;

use std::sync::{RwLock, RwLockWriteGuard};

use dashmap::{DashMap, DashSet};
use lint::Linter;
use pg_base_db::{Document, DocumentChange, PgLspPath, StatementRef};
use pg_query::PgQueryParser;
use pg_schema_cache::SchemaCache;
use sqlx::PgPool;
use tree_sitter::TreeSitterParser;
use typecheck::Typechecker;

pub struct Workspace {
    pub documents: DashMap<PgLspPath, Document>,
    // Stores the statements that have changed since the last analysis
    changed_stmts: DashSet<StatementRef>,
    pub schema_cache: RwLock<SchemaCache>,

    pub tree_sitter: TreeSitterParser,
    pub pg_query: PgQueryParser,
    pub linter: Linter,
    pub typechecker: Typechecker,
}

impl Workspace {
    pub fn new() -> Workspace {
        Workspace {
            documents: DashMap::new(),
            schema_cache: RwLock::new(SchemaCache::new()),
            changed_stmts: DashSet::new(),

            tree_sitter: TreeSitterParser::new(),
            pg_query: PgQueryParser::new(),
            linter: Linter::new(),
            typechecker: Typechecker::new(),
        }
    }

    /// Applies changes to the current state of the world
    ///
    /// Returns a list of changed statements
    pub fn apply_change(&self, url: PgLspPath, mut change: DocumentChange) {
        let mut doc = self
            .documents
            .entry(url.clone())
            .or_insert(Document::new_empty(url));

        change.apply(doc.value_mut());

        let changed_stmts = change.collect_statement_changes();

        for c in &changed_stmts {
            match c {
                pg_base_db::StatementChange::Added(s) => {
                    self.tree_sitter.add_statement(s);
                    self.pg_query.add_statement(s);

                    self.changed_stmts.insert(s.to_owned());
                }
                pg_base_db::StatementChange::Deleted(s) => {
                    self.tree_sitter.remove_statement(s);
                    self.pg_query.remove_statement(s);
                    self.linter.clear_statement_violations(s);
                    self.typechecker.clear_statement_errors(s);

                    self.changed_stmts.insert(s.to_owned());
                }
                pg_base_db::StatementChange::Modified(s) => {
                    self.tree_sitter.modify_statement(s);
                    self.pg_query.modify_statement(s);
                    self.linter.clear_statement_violations(&s.statement);
                    self.typechecker.clear_statement_errors(&s.statement);

                    self.changed_stmts.remove(&s.statement);
                    self.changed_stmts.insert(s.new_statement().to_owned());
                }
            }
        }
    }

    pub fn remove_document(&self, url: PgLspPath) {
        let r = self.documents.remove(&url);
        if r.is_some() {
            let doc = r.unwrap().1;
            for stmt in doc.statement_refs() {
                self.tree_sitter.remove_statement(&stmt);
                self.pg_query.remove_statement(&stmt);
                self.linter.clear_statement_violations(&stmt);
                self.typechecker.clear_statement_errors(&stmt);
            }
        }
    }

    /// Collects all diagnostics for a given document. It does not compute them, it just collects.
    pub fn diagnostics(&self, url: &PgLspPath) -> Vec<pg_diagnostics::Diagnostic> {
        let mut diagnostics: Vec<pg_diagnostics::Diagnostic> = vec![];

        let doc = self.documents.get(&url);

        if doc.is_none() {
            return diagnostics;
        }

        let doc = doc.unwrap();

        for (range, stmt) in doc.statement_refs_with_range() {
            diagnostics.extend(self.pg_query.diagnostics(&stmt, range));
            diagnostics.extend(self.linter.diagnostics(&stmt, range));
            diagnostics.extend(self.typechecker.diagnostics(&stmt, range));
        }

        diagnostics
    }

    /// Drain changed statements to kick off analysis
    pub fn compute(&self, conn: Option<PgPool>) -> Vec<StatementRef> {
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
                    ::pg_lint::LinterParams {
                        ast: ast.as_ref(),
                        enriched_ast: self
                            .pg_query
                            .enriched_ast(stmt)
                            .as_ref()
                            .map(|a| a.as_ref()),
                    },
                );
                if let Some(conn) = conn.as_ref() {
                    self.typechecker.run_typecheck(
                        stmt,
                        ::pg_typecheck::TypecheckerParams {
                            conn,
                            sql: &stmt.text,
                            ast: ast.as_ref(),
                            enriched_ast: self
                                .pg_query
                                .enriched_ast(stmt)
                                .as_ref()
                                .map(|a| a.as_ref()),
                        },
                    );
                }
            }
        });
        changed
    }

    pub fn set_schema_cache(&self, cache: SchemaCache) {
        let mut schema_cache: RwLockWriteGuard<SchemaCache> = self.schema_cache.write().unwrap();
        *schema_cache = cache;

        // clear all schema cache related diagnostics
        // and add all statements to the changed statements
        self.typechecker.clear_errors();
        self.documents
            .iter()
            .flat_map(|entry| entry.value().statement_refs())
            .for_each(|f| {
                self.changed_stmts.insert(f);
            })
    }
}

#[cfg(test)]
mod tests {

    use pg_base_db::{Change, DocumentChange};
    use pg_diagnostics::Diagnostic;
    use text_size::{TextRange, TextSize};

    use crate::{PgLspPath, Workspace};

    #[test]
    fn test_apply_change() {
        let ide = Workspace::new();

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
    fn test_diagnostics_within_statement() {
        let ide = Workspace::new();

        let url = PgLspPath::new("test.sql");

        ide.apply_change(
            url.clone(),
            DocumentChange::new(
                1,
                vec![Change {
                    range: None,
                    text: "select unknown from contact;\n\nselect 12345;\n\nalter table test drop column id;\n".to_string(),
                }],
            ),
        );

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);

        {
            let doc = ide.documents.get(&PgLspPath::new("test.sql")).unwrap();
            assert_eq!(doc.statement_refs().len(), 3);
            assert_eq!(
                doc.statement_ref(0).text,
                "select unknown from contact;".to_string()
            );
            assert_eq!(doc.statement_ref(1).text, "select 12345;".to_string());
            assert_eq!(
                doc.statement_ref(2).text,
                "alter table test drop column id;".to_string()
            );
        }

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);

        ide.apply_change(
            PgLspPath::new("test.sql"),
            DocumentChange::new(
                1,
                vec![Change {
                    range: Some(TextRange::new(76.into(), 76.into())),
                    text: "a".to_string(),
                }],
            ),
        );

        {
            let doc = ide.documents.get(&PgLspPath::new("test.sql")).unwrap();
            assert_eq!(doc.statement_refs().len(), 3);
            assert_eq!(
                doc.statement_ref(0).text,
                "select unknown from contact;".to_string()
            );
            assert_eq!(doc.statement_ref(1).text, "select 12345;".to_string());
            assert_eq!(
                doc.statement_ref(2).text,
                "alter table test drop column ida;".to_string()
            );
        }

        // the problem is here!
        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);
    }

    #[test]
    fn test_apply_deletion_change() {
        let ide = Workspace::new();

        let url = PgLspPath::new("test.sql");

        ide.apply_change(
            url.clone(),
            DocumentChange::new(
                1,
                vec![Change {
                    range: None,
                    text: "select unknown from contact;\n\nselect 12345;\n\nalter table test drop column id;\n".to_string(),
                }],
            ),
        );

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);

        {
            let doc = ide.documents.get(&PgLspPath::new("test.sql")).unwrap();
            assert_eq!(doc.statement_refs().len(), 3);
            assert_eq!(
                doc.statement_ref(0).text,
                "select unknown from contact;".to_string()
            );
            assert_eq!(doc.statement_ref(1).text, "select 12345;".to_string());
            assert_eq!(
                doc.statement_ref(2).text,
                "alter table test drop column id;".to_string()
            );
        }

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);

        ide.apply_change(
            PgLspPath::new("test.sql"),
            DocumentChange::new(
                1,
                vec![Change {
                    range: Some(TextRange::new(39.into(), 40.into())),
                    text: "".to_string(),
                }],
            ),
        );

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);

        {
            let doc = ide.documents.get(&PgLspPath::new("test.sql")).unwrap();
            assert_eq!(doc.statement_refs().len(), 3);
            assert_eq!(
                doc.statement_ref(0).text,
                "select unknown from contact;".to_string()
            );
            assert_eq!(doc.statement_ref(1).text, "select 1245;".to_string());
            assert_eq!(
                doc.statement_ref(2).text,
                "alter table test drop column id;".to_string()
            );
        }

        ide.compute(None);

        assert_eq!(ide.diagnostics(&url).len(), 1);
    }

    #[test]
    fn test_lint() {
        let ide = Workspace::new();
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

        ide.compute(None);

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
        let ide = Workspace::new();

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
