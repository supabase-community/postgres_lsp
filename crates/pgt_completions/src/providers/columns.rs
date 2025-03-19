use crate::{
    CompletionItem, CompletionItemKind, builder::CompletionBuilder, context::CompletionContext,
    relevance::CompletionRelevanceData,
};

pub fn complete_columns(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_columns = &ctx.schema_cache.columns;

    for col in available_columns {
        let item = CompletionItem {
            label: col.name.clone(),
            score: CompletionRelevanceData::Column(col).get_score(ctx),
            description: format!("Table: {}.{}", col.schema_name, col.table_name),
            preselected: false,
            kind: CompletionItemKind::Column,
        };

        builder.add_item(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CompletionItem, complete,
        test_helper::{CURSOR_POS, InputQuery, get_test_deps, get_test_params},
    };

    struct TestCase {
        query: String,
        message: &'static str,
        label: &'static str,
        description: &'static str,
    }

    impl TestCase {
        fn get_input_query(&self) -> InputQuery {
            let strs: Vec<&str> = self.query.split_whitespace().collect();
            strs.join(" ").as_str().into()
        }
    }

    #[tokio::test]
    async fn completes_columns() {
        let setup = r#"
            create schema private;

            create table public.users (
                id serial primary key,
                name text
            );

            create table public.audio_books (
                id serial primary key,
                narrator text
            );

            create table private.audio_books (
                id serial primary key,
                narrator_id text
            );
        "#;

        let queries: Vec<TestCase> = vec![
            TestCase {
                message: "correctly prefers the columns of present tables",
                query: format!(r#"select na{} from public.audio_books;"#, CURSOR_POS),
                label: "narrator",
                description: "Table: public.audio_books",
            },
            TestCase {
                message: "correctly handles nested queries",
                query: format!(
                    r#"
                select
                    *
                from (
                    select id, na{}
                    from private.audio_books
                ) as subquery
                join public.users u
                on u.id = subquery.id;
                "#,
                    CURSOR_POS
                ),
                label: "narrator_id",
                description: "Table: private.audio_books",
            },
            TestCase {
                message: "works without a schema",
                query: format!(r#"select na{} from users;"#, CURSOR_POS),
                label: "name",
                description: "Table: public.users",
            },
        ];

        for q in queries {
            let (tree, cache) = get_test_deps(setup, q.get_input_query()).await;
            let params = get_test_params(&tree, &cache, q.get_input_query());
            let results = complete(params);

            let CompletionItem {
                label, description, ..
            } = results
                .into_iter()
                .next()
                .expect("Should return at least one completion item");

            assert_eq!(label, q.label, "{}", q.message);
            assert_eq!(description, q.description, "{}", q.message);
        }
    }

    #[tokio::test]
    async fn shows_multiple_columns_if_no_relation_specified() {
        let setup = r#"
            create schema private;

            create table public.users (
                id serial primary key,
                name text
            );

            create table public.audio_books (
                id serial primary key,
                narrator text
            );

            create table private.audio_books (
                id serial primary key,
                narrator_id text
            );
        "#;

        let case = TestCase {
            query: format!(r#"select n{};"#, CURSOR_POS),
            description: "",
            label: "",
            message: "",
        };

        let (tree, cache) = get_test_deps(setup, case.get_input_query()).await;
        let params = get_test_params(&tree, &cache, case.get_input_query());
        let mut results = complete(params);

        let _ = results.items.split_off(3);

        results.items.sort_by(|a, b| a.label.cmp(&b.label));

        let labels: Vec<String> = results.items.into_iter().map(|c| c.label).collect();

        assert_eq!(labels, vec!["name", "narrator", "narrator_id"]);
    }
}
