use pg_schema_cache::Table;

use crate::{
    builder::CompletionBuilder,
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind},
    relevance::CompletionRelevance,
};

pub fn complete_tables(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_tables = &ctx.schema_cache.tables;

    let completion_items: Vec<CompletionItem> = available_tables
        .iter()
        .map(|table| CompletionItem {
            label: table.name.clone(),
            score: get_score(ctx, table),
            description: format!("Schema: {}", table.schema),
            preselected: None,
            kind: CompletionItemKind::Table,
        })
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

fn get_score(ctx: &CompletionContext, table: &Table) -> i32 {
    let mut relevance = CompletionRelevance::default();

    relevance.check_matches_query_input(ctx, &table.name);
    relevance.check_matches_schema(ctx, &table.schema);
    relevance.check_if_catalog(ctx);

    relevance.score()
}
