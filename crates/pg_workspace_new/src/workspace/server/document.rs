use pg_fs::PgLspPath;
use text_size::{TextRange, TextSize};

/// Global unique identifier for a statement
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub(crate) struct StatementRef {
    /// Path of the document
    pub(crate) path: PgLspPath,
    /// Unique id within the document
    pub(crate) id: StatementId,
}

/// Represenation of a statement
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Statement {
    pub(crate) ref_: StatementRef,
    pub(crate) text: String,
}

pub type StatementId = usize;

type StatementPosition = (StatementId, TextRange);

pub(crate) struct Document {
    pub(crate) path: PgLspPath,
    pub(crate) content: String,
    pub(crate) version: i32,
    /// List of statements sorted by range.start()
    pub(super) statements: Vec<StatementPosition>,

    pub(super) id_generator: IdGenerator,
}

impl Document {
    pub(crate) fn new(path: PgLspPath, content: String, version: i32) -> Self {
        let mut id_generator = IdGenerator::new();

        let statements: Vec<StatementPosition> = pg_statement_splitter::split(&content)
            .ranges
            .iter()
            .map(|r| (id_generator.next(), *r))
            .collect();

        Self {
            path,
            statements,
            content,
            version,

            id_generator,
        }
    }

    pub fn debug_statements(&self) {
        for (id, range) in self.statements.iter() {
            tracing::info!(
                "Document::debug_statements: statement: id: {}, range: {:?}, text: {:?}",
                id,
                range,
                &self.content[*range]
            );
        }
    }

    pub fn get_statements(&self) -> &[StatementPosition] {
        &self.statements
    }

    pub fn statement_refs(&self) -> Vec<StatementRef> {
        self.statements
            .iter()
            .map(|inner_ref| self.statement_ref(inner_ref))
            .collect()
    }

    /// Returns the statement ref at the given offset
    pub fn statement_ref_at_offset(&self, offset: &TextSize) -> Option<StatementRef> {
        self.statements.iter().find_map(|r| {
            if r.1.contains(*offset) {
                Some(self.statement_ref(r))
            } else {
                None
            }
        })
    }

    /// Returns the statement refs at the given range
    pub fn statement_refs_at_range(&self, range: &TextRange) -> Vec<StatementRef> {
        self.statements
            .iter()
            .filter(|(_, r)| {
                range.contains_range(r.to_owned().to_owned()) || r.contains_range(range.to_owned())
            })
            .map(|x| self.statement_ref(x))
            .collect()
    }

    /// Returns the statement at the given offset
    pub fn statement_at_offset(&self, offset: &TextSize) -> Option<Statement> {
        self.statements.iter().find_map(|r| {
            if r.1.contains(*offset) {
                Some(self.statement(r))
            } else {
                None
            }
        })
    }

    /// Returns the statements at the given range
    pub fn statements_at_range(&self, range: &TextRange) -> Vec<Statement> {
        self.statements
            .iter()
            .filter(|(_, r)| {
                range.contains_range(r.to_owned().to_owned()) || r.contains_range(range.to_owned())
            })
            .map(|x| self.statement(x))
            .collect()
    }

    pub(super) fn statement_ref(&self, inner_ref: &StatementPosition) -> StatementRef {
        StatementRef {
            id: inner_ref.0,
            path: self.path.clone(),
        }
    }

    pub(super) fn statement(&self, inner_ref: &StatementPosition) -> Statement {
        Statement {
            ref_: self.statement_ref(inner_ref),
            text: self.content[inner_ref.1].to_string(),
        }
    }
}

pub(crate) struct IdGenerator {
    pub(super) next_id: usize,
}

impl IdGenerator {
    fn new() -> Self {
        Self { next_id: 0 }
    }

    pub(super) fn next(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
