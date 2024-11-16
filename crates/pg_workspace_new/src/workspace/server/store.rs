use std::usize;

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

pub type StatementId = usize;

pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,
    /// List of statements sorted by range.start()
    pub(crate) statements: Vec<(StatementId, TextRange)>,

    id_generator: IdGenerator,
}

impl Document {
    pub(crate) fn new(content: String, version: i32) -> Self {
        let mut id_generator = IdGenerator::new();

        Self {
            statements: pg_statement_splitter::split(&content).ranges.iter().map(|r| {
                (id_generator.next(), *r)
            }).collect(),
            content,
            version,

            id_generator,
        }
    }

    /// Returns the statement at the given offset
    pub fn statement_at_offset(&self, offset: &TextSize) -> Option<StatementId> {
        self.statements
            .iter()
            .position(|r| r.1.contains(*offset))
    }

    /// Returns the statements at the given range
    pub fn statements_at_range(&self, range: &TextRange) -> Vec<StatementId> {
        self.statements
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                range.contains_range(r.1.to_owned().to_owned()) || r.1.contains_range(range.to_owned())
            })
            .map(|(idx, _)| idx)
            .collect()
    }
}

struct IdGenerator {
    next_id: usize,
}

impl IdGenerator {
    fn new() -> Self {
        Self {
            next_id: 0,
        }
    }

    fn next(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
