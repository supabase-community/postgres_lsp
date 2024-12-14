use pg_schema_cache::Function;

use crate::{
    builder::CompletionBuilder, context::CompletionContext, relevance::CompletionRelevanceData,
    CompletionItem, CompletionItemKind,
};

pub fn complete_functions(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_functions = &ctx.schema_cache.functions;

    let completion_items: Vec<CompletionItem> = available_functions
        .iter()
        .map(|foo| CompletionItem {
            label: foo.name.clone(),
            score: CompletionRelevanceData::Function(foo).get_score(ctx),
            description: format!("Schema: {}", foo.schema),
            preselected: false,
            kind: CompletionItemKind::Function,
        })
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::CompletionContext,
        providers::complete_functions,
        test_helper::{get_test_deps, get_test_params, CURSOR_POS},
        CompletionItem,
    };

    #[tokio::test]
    async fn completes_fn() {
        let setup = r#"
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

        let query = format!("select coo{}", CURSOR_POS);

        let (tree, cache, mut builder) = get_test_deps(setup, &query).await;
        let params = get_test_params(&tree, &cache, &query);
        let ctx = CompletionContext::new(&params);

        complete_functions(&ctx, &mut builder);

        let results = builder.finish();

        let CompletionItem { label, .. } = results
            .into_iter()
            .next()
            .expect("Should return at least one completion item");

        assert_eq!(label, "cool");
    }
}
