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
