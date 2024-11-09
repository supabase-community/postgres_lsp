use pg_fs::PgLspPath;
use text_size::TextRange;

/// Global unique identifier for a statement
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub(crate) struct StatementId {
    /// Path of the document
    pub(crate) path: PgLspPath,
    /// Unique id within the document
    pub(crate) id: usize,
}


pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,
    /// List of statements sorted by range.start()
    pub(crate) statements: Vec<(usize, TextRange)>,

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
