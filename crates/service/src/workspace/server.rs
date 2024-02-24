use std::{
    cmp::{max, min},
    iter::once,
    panic::RefUnwindSafe,
};

use dashmap::DashMap;

use fs::FilePath;
use parser::get_statements;
use schema_cache::SchemaCache;
use text_size::TextRange;
use tree_sitter::Tree;

use crate::diagnostics::{NotFound, WorkspaceError};

use super::{CloseFileParams, FileChange, FileChangesParams, OpenFileParams, Workspace};

pub(super) struct WorkspaceServer {
    documents: DashMap<FilePath, Document>,
    schema_cache: SchemaCache,
}

#[derive(Debug)]
pub(crate) struct Statement {
    pub(crate) content: String,

    pub(crate) tree_sitter: Tree,
}

impl Statement {
    fn new(content: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(&content, None).unwrap();

        Self {
            content,
            tree_sitter: tree,
        }
    }

    fn apply_change(&mut self, change: FileChange) {
        assert!(change.range.is_some());
        // TODO update tree sitter and content
    }
}

// we get the version from lsp
#[derive(Debug)]
pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,

    pub(crate) statements: Vec<(TextRange, Statement)>,
}

impl Document {
    fn new(content: String, version: i32) -> Self {
        Self {
            version,
            statements: get_statements(&content)
                .iter()
                .map(|(range, content)| {
                    let statement = Statement::new(content.clone());
                    (range.clone(), statement)
                })
                .collect(),
            content,
        }
    }

    fn apply_changes(&mut self, changes: FileChangesParams) {
        changes.changes.iter().for_each(|c| {
            self.apply_change(c);
        });
        self.version = changes.version;
    }

    fn apply_change(&mut self, change: &FileChange) {
        if change.range.is_none() {
            self.content = change.text.clone();
            self.statements = get_statements(&self.content)
                .iter()
                .map(|(range, content)| {
                    let statement = Statement::new(content.clone());
                    (range.clone(), statement)
                })
                .collect();
            return;
        }

        // TODO: update content
        if let Some(changed_stmt_pos) = self
            .statements
            .iter()
            .position(|(range, _)| range.contains_range(change.range.unwrap()))
        {
            let stmt_range = self.statements[changed_stmt_pos].0;
            self.statements
                .get_mut(changed_stmt_pos)
                .unwrap()
                .1
                .apply_change(FileChange {
                    text: change.text.clone(),
                    // range must be relative to the start of the statement
                    range: change.range.unwrap().checked_sub(stmt_range.start()),
                });
            self.statements
                .iter_mut()
                .skip_while(|(range, _)| range.start() <= change.range.unwrap().end())
                .for_each(|(range, _)| {
                    if range.start() > stmt_range.end() {
                        if change.is_addition() {
                            range.checked_add(change.diff_size());
                        } else if change.is_deletion() {
                            range.checked_sub(change.diff_size());
                        }
                    }
                });
        } else {
            // remove all statements that are affected by this change,
            let mut min = change.range.unwrap().start();
            let mut max = change.range.unwrap().end();

            for (range, _) in self
                .statements
                .extract_if(|(range, _)| change.range.unwrap().contains_range(range.clone()))
            {
                if range.start() < min {
                    min = range.start();
                }
                if range.end() > max {
                    max = range.end();
                }
            }
            // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
            let extracted_text = self
                .content
                .as_str()
                .get(usize::from(min)..usize::from(max))
                .unwrap();
            // and reparse
            self.statements
                .extend(
                    get_statements(extracted_text)
                        .iter()
                        .map(|(range, content)| {
                            let statement = Statement::new(content.clone());
                            (range.clone(), statement)
                        }),
                );
        }
    }
}

/// The `Workspace` object is long-lived, so we want it to be able to cross
/// unwind boundaries.
/// In return, we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead too hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

impl Workspace for WorkspaceServer {
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        let OpenFileParams {
            path,
            content,
            version,
        } = params;

        self.documents
            .insert(path.clone(), Document::new(content.clone(), version));

        Ok(())
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.documents.remove(&params.path);
        Ok(())
    }

    fn apply_file_changes(&self, params: FileChangesParams) -> Result<(), WorkspaceError> {
        let mut doc = self
            .documents
            .get_mut(&params.path)
            .ok_or_else(|| WorkspaceError::NotFound(NotFound))?;

        doc.apply_changes(params);

        Ok(())
    }
}
