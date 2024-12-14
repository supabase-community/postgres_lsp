use crate::{
    builder::CompletionBuilder,
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind},
    relevance::CompletionRelevanceData,
};

pub fn complete_tables(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_tables = &ctx.schema_cache.tables;

    let completion_items: Vec<CompletionItem> = available_tables
        .iter()
        .map(|table| CompletionItem {
            label: table.name.clone(),
            score: CompletionRelevanceData::Table(table).get_score(ctx),
            description: format!("Schema: {}", table.schema),
            preselected: false,
            kind: CompletionItemKind::Table,
        })
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        complete,
        test_helper::{get_test_deps, get_test_params, CURSOR_POS},
        CompletionItem, CompletionItemKind,
    };

    #[tokio::test]
    async fn prefers_table_in_from_clause() {
        let setup = r#"
          create table coos (
            id serial primary key,
            name text
          );

          create or replace function cool() 
          returns trigger
          language plpgsql
          security invoker
          as $$
          begin
            raise exception 'dont matter';
          end;
          $$;
        "#;

        let query = format!(r#"select * from coo{}"#, CURSOR_POS);

        let (tree, cache) = get_test_deps(setup, &query).await;
        let params = get_test_params(&tree, &cache, &query);

        let results = complete(params);

        let CompletionItem { label, kind, .. } = results
            .into_iter()
            .next()
            .expect("Should return at least one completion item");

        assert_eq!(label, "coos");
        assert_eq!(kind, CompletionItemKind::Table);
    }
}
