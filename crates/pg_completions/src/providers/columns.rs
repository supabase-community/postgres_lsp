use crate::{
    builder::CompletionBuilder, context::CompletionContext, relevance::CompletionRelevanceData,
    CompletionItem, CompletionItemKind,
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
