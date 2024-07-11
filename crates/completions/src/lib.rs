mod builder;
mod providers;

pub use providers::CompletionProviderParams;
use text_size::{TextRange, TextSize};

pub const LIMIT: usize = 50;

#[derive(Debug)]
pub struct CompletionParams<'a> {
    pub position: TextSize,
    pub schema: &'a schema_cache::SchemaCache,
    pub text: &'a str,
    pub tree: Option<&'a tree_sitter::Tree>,
}

#[derive(Debug, Default)]
pub struct CompletionResult<'a> {
    pub items: Vec<CompletionItem<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompletionItem<'a> {
    pub score: i32,
    pub range: TextRange,
    pub preselect: bool,
    pub data: CompletionItemData<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompletionItemData<'a> {
    Table(&'a schema_cache::Table),
}

impl<'a> CompletionItemData<'a> {
    pub fn label(&self) -> &'a str {
        match self {
            CompletionItemData::Table(t) => t.name.as_str(),
        }
    }
}

impl<'a> CompletionItem<'a> {
    pub fn new_simple(score: i32, range: TextRange, data: CompletionItemData<'a>) -> Self {
        Self {
            score,
            range,
            preselect: false,
            data,
        }
    }
}

pub fn complete<'a>(params: &'a CompletionParams<'a>) -> CompletionResult<'a> {
    let mut builder = builder::CompletionBuilder::from(&builder::CompletionConfig {});

    let params = CompletionProviderParams::from(params);

    providers::complete_tables(params, &mut builder);

    builder.finish()
}

#[cfg(test)]
mod tests {
    use async_std::task::block_on;
    use schema_cache::SchemaCache;
    use sqlx::PgPool;

    use crate::{complete, CompletionParams};

    #[test]
    fn test_complete() {
        let input = "select id from c;";

        let conn_string = std::env::var("DB_CONNECTION_STRING").unwrap();

        let pool = block_on(PgPool::connect(conn_string.as_str())).unwrap();

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();

        let schema_cache = block_on(SchemaCache::load(&pool));

        let p = CompletionParams {
            position: 15.into(),
            schema: &schema_cache,
            text: input,
            tree: Some(&tree),
        };

        let result = complete(&p);

        assert!(result.items.len() > 0);
    }

    #[test]
    fn test_complete_two() {
        let input = "select id, name, test1231234123, unknown from co;";

        let conn_string = std::env::var("DB_CONNECTION_STRING").unwrap();

        let pool = block_on(PgPool::connect(conn_string.as_str())).unwrap();

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();
        let schema_cache = block_on(SchemaCache::load(&pool));

        let p = CompletionParams {
            position: 47.into(),
            schema: &schema_cache,
            text: input,
            tree: Some(&tree),
        };

        let result = complete(&p);

        assert!(result.items.len() > 0);
    }
}
