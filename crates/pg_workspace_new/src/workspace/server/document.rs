use pg_fs::PgLspPath;
use text_size::{TextRange, TextSize};

/// Global unique identifier for a statement
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub(crate) struct Statement {
    /// Path of the document
    pub(crate) path: PgLspPath,
    /// Unique id within the document
    pub(crate) id: StatementId,
}

pub type StatementId = usize;

type StatementPos = (StatementId, TextRange);

pub(crate) struct Document {
    pub(crate) path: PgLspPath,
    pub(crate) content: String,
    pub(crate) version: i32,
    /// List of statements sorted by range.start()
    pub(super) positions: Vec<StatementPos>,

    pub(super) id_generator: IdGenerator,
}

impl Document {
    pub(crate) fn new(path: PgLspPath, content: String, version: i32) -> Self {
        let mut id_generator = IdGenerator::new();

        let ranges: Vec<StatementPos> = pg_statement_splitter::split(&content)
            .ranges
            .iter()
            .map(|r| (id_generator.next(), *r))
            .collect();

        Self {
            path,
            positions: ranges,
            content,
            version,

            id_generator,
        }
    }

    pub fn iter_statements(&self) -> impl Iterator<Item = Statement> + '_ {
        self.positions.iter().map(move |(id, _)| Statement {
            id: *id,
            path: self.path.clone(),
        })
    }

    pub fn iter_statements_with_text(&self) -> impl Iterator<Item = (Statement, &str)> + '_ {
        self.positions.iter().map(move |(id, range)| {
            let statement = Statement {
                id: *id,
                path: self.path.clone(),
            };
            let text = &self.content[range.start().into()..range.end().into()];
            (statement, text)
        })
    }

    pub fn iter_statements_with_range(&self) -> impl Iterator<Item = (Statement, &TextRange)> + '_ {
        self.positions.iter().map(move |(id, range)| {
            let statement = Statement {
                id: *id,
                path: self.path.clone(),
            };
            (statement, range)
        })
    }

    pub fn iter_statements_with_text_and_range(
        &self,
    ) -> impl Iterator<Item = (Statement, &TextRange, &str)> + '_ {
        self.positions.iter().map(move |(id, range)| {
            let statement = Statement {
                id: *id,
                path: self.path.clone(),
            };
            (
                statement,
                range,
                &self.content[range.start().into()..range.end().into()],
            )
        })
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
