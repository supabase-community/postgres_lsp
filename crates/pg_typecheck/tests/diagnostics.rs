use pg_console::{
    fmt::{Formatter, HTML},
    markup,
};
use pg_diagnostics::PrintDiagnostic;
use pg_test_utils::test_database::get_new_test_db;
use pg_typecheck::{check_sql, TypecheckParams};
use sqlx::Executor;

async fn test(name: &str, query: &str, setup: &str) {
    let test_db = get_new_test_db().await;

    test_db
        .execute(setup)
        .await
        .expect("Failed to setup test database");

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_sql::language())
        .expect("Error loading sql language");

    let root = pg_query_ext::parse(query).unwrap();
    let tree = parser.parse(query, None);

    let conn = &test_db;
    let result = check_sql(TypecheckParams {
        conn,
        sql: query,
        ast: &root,
        tree: tree.as_ref(),
    })
    .await;

    let mut content = vec![];
    let mut writer = HTML::new(&mut content);

    Formatter::new(&mut writer)
        .write_markup(markup! {
            {PrintDiagnostic::simple(&result.unwrap())}
        })
        .unwrap();

    let content = String::from_utf8(content).unwrap();

    // Normalize line endings
    let normalized_content = content.replace("\r\n", "\n");

    // Replace Windows-style path separators with forward slashes
    let normalized_content = normalized_content.replace("\\", "/");

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(name, normalized_content);
    });
}

#[tokio::test]
async fn invalid_column() {
    test(
        "invalid_column",
        "select id, unknown from contacts;",
        r#"
        create table public.contacts (
            id serial primary key,
            name varchar(255) not null,
            is_vegetarian bool default false,
            middle_name varchar(255)
        );
    "#,
    )
    .await;
}
