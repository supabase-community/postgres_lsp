use pg_schema_cache::SchemaCache;
use pg_test_utils::test_database::get_new_test_db;
use sqlx::Executor;

use crate::CompletionParams;

pub static CURSOR_POS: char = '€';

pub(crate) async fn get_test_deps(
    setup: &str,
    input: &str,
) -> (tree_sitter::Tree, pg_schema_cache::SchemaCache) {
    let test_db = get_new_test_db().await;

    test_db
        .execute(setup)
        .await
        .expect("Failed to execute setup query");

    let schema_cache = SchemaCache::load(&test_db)
        .await
        .expect("Failed to load Schema Cache");

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_sql::language())
        .expect("Error loading sql language");

    let tree = parser.parse(input, None).unwrap();

    (tree, schema_cache)
}

pub(crate) fn get_text_and_position(sql: &str) -> (usize, String) {
    // the cursor is to the left of the `CURSOR_POS`
    let position = sql
        .find(CURSOR_POS)
        .expect("Please insert the CURSOR_POS into your query.").saturating_sub(1);

    let text = sql.replace(CURSOR_POS, "");

    (position, text)
}

pub(crate) fn get_test_params<'a>(
    tree: &'a tree_sitter::Tree,
    schema_cache: &'a pg_schema_cache::SchemaCache,
    sql: &'a str,
) -> CompletionParams<'a> {
    let (position, text) = get_text_and_position(sql);

    CompletionParams {
        position: (position as u32).into(),
        schema: schema_cache,
        tree: Some(tree),
        text,
    }
}
