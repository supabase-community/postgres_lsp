use crate::{
    builder::CompletionBuilder, context::CompletionContext, data::CompletionItemData,
    item::CompletionItemWithScore, relevance::CompletionRelevance,
};

pub fn complete_tables(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_tables = &ctx.schema_cache.tables;

    let completion_items: Vec<CompletionItemWithScore> = available_tables
        .iter()
        .map(|table| {
            let data = CompletionItemData::Table(table);
            let relevance = CompletionRelevance::from_data_and_ctx(&data, ctx);
            CompletionItemWithScore::new(data, relevance)
        })
        .collect();

    for item in completion_items {
        builder.add_item(item);
    }
}
