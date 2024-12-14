use pg_schema_cache::Function;

use crate::{
    builder::CompletionBuilder, context::CompletionContext, CompletionItem, CompletionItemKind,
};

pub fn complete_functions(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_functions = &ctx.schema_cache.functions;

    let completion_items: Vec<CompletionItem> = available_functions
        .iter()
        .map(|foo| CompletionItem {
            label: foo.name,
            score: get_score(ctx, foo),
            description: format!("Schema: {}", foo.schema),
            preselected: None,
            kind: CompletionItemKind::Function,
        })
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

fn get_score(ctx: &CompletionContext, foo: &Function) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{
        context::CompletionContext,
        providers::complete_tables,
        test_helper::{get_test_deps, get_test_params, CURSOR_POS},
    };

    #[tokio::test]
    async fn completes_fn() {
        let setup = r#"
          create or replace function cool() returns trigger
          begin;
            ## Yeahhhh
          end;
        "#;

        let query = format!("select coo{}", CURSOR_POS);

        let (tree, cache, mut builder) = get_test_deps(setup, &query).await;
        let params = get_test_params(&tree, &cache, &query);
        let ctx = CompletionContext::new(&params);

        complete_tables(&ctx, &mut builder);
    }
}
