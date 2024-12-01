use text_size::TextSize;

use crate::{
    builder::CompletionBuilder, context::CompletionContext, item::CompletionItem, providers,
};

pub const LIMIT: usize = 50;

#[derive(Debug)]
pub struct CompletionParams<'a> {
    pub position: TextSize,
    pub schema: &'a pg_schema_cache::SchemaCache,
    pub text: &'a str,
    pub tree: Option<&'a tree_sitter::Tree>,
}

#[derive(Debug, Default)]
pub struct CompletionResult {
    pub items: Vec<CompletionItem>,
}

pub fn complete(params: CompletionParams) -> CompletionResult {
    let ctx = CompletionContext::new(&params);
    let mut builder = CompletionBuilder::new();

    if let Some(node) = ctx.ts_node {
        match node.kind() {
            "relation" => providers::complete_tables(&ctx, &mut builder),
            _ => {}
        }
    } else {
        // if query emtpy, autocomplete select keywords etc?
    }

    builder.finish()
}

#[cfg(test)]
mod tests {
    use pg_schema_cache::SchemaCache;
    use pg_test_utils::test_database::*;

    use sqlx::Executor;

    use crate::{complete, CompletionParams};

    #[tokio::test]
    async fn test_complete() {
        let pool = get_new_test_db().await;

        let input = "select id from c;";

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();

        let schema_cache = SchemaCache::load(&pool).await;

        let p = CompletionParams {
            position: 15.into(),
            schema: &schema_cache,
            text: input,
            tree: Some(&tree),
        };

        let result = complete(p);

        assert!(result.items.len() > 0);
    }

    #[tokio::test]
    async fn test_complete_two() {
        let pool = get_new_test_db().await;

        let input = "select id, name, test1231234123, unknown from co;";

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();
        let schema_cache = SchemaCache::load(&pool).await;

        let p = CompletionParams {
            position: 47.into(),
            schema: &schema_cache,
            text: input,
            tree: Some(&tree),
        };

        let result = complete(p);

        assert!(result.items.len() > 0);
    }

    #[tokio::test]
    async fn test_complete_three() {
        let test_db = get_new_test_db().await;

        let setup = r#"
            create table users (
                id serial primary key,
                name text,
                password text
            );
        "#;

        test_db
            .execute(setup)
            .await
            .expect("Failed to execute setup query");

        let input = "select * from u";

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();
        let schema_cache = SchemaCache::load(&test_db).await;

        let p = CompletionParams {
            position: ((input.len() - 1) as u32).into(),
            schema: &schema_cache,
            text: input,
            tree: Some(&tree),
        };

        let result = complete(p);

        // TODO: actually assert that we get good autocompletion suggestions
        assert!(result.items.len() > 0);
    }
}
