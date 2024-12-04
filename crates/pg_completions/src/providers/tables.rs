use pg_schema_cache::Table;

use crate::{
    builder::CompletionBuilder, context::CompletionContext, data::CompletionItemData,
    item::CompletionItemWithRelevance, relevance::CompletionRelevance,
};

pub fn complete_tables(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_tables = &ctx.schema_cache.tables;

    let completion_items: Vec<CompletionItemWithRelevance> = available_tables
        .iter()
        .map(|table| to_completion_item(ctx, table))
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

fn to_completion_item(ctx: &CompletionContext, table: &Table) -> CompletionItemWithRelevance {
    let data = CompletionItemData::Table(table);
    let relevance = CompletionRelevance::from_data_and_ctx(&data, ctx);
    CompletionItemWithRelevance::new(data, relevance)
}
