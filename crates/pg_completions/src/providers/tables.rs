use pg_schema_cache::Table;
use text_size::{TextRange, TextSize};

use crate::{
    builder::CompletionBuilder,
    context::CompletionContext,
    item::{CompletionItem, CompletionItemData},
    relevance::CompletionRelevance,
};

pub fn complete_tables(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_tables = &ctx.schema_cache.tables;

    let completion_items: Vec<CompletionItem> = available_tables
        .iter()
        .map(|table| to_completion_item(ctx, table))
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}

fn to_completion_item(ctx: &CompletionContext, table: &Table) -> CompletionItem {
    let data = CompletionItemData::Table(table);

    let start = ctx.position;
    let end = start + TextSize::from(table.name.len() as u32);
    let range = TextRange::new(start, end);

    let mut relevance = CompletionRelevance::default();

    relevance.set_is_catalog(&table.schema);
    relevance.set_matches_prefix(ctx, &table.name);
    relevance.set_matches_schema(ctx, &table.schema);

    CompletionItem::new(range, data, relevance)
}
