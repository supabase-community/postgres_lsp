use crate::{
    builder::CompletionBuilder, context::CompletionContext, relevance::CompletionRelevanceData,
    CompletionItem, CompletionItemKind,
};

pub fn complete_functions(ctx: &CompletionContext, builder: &mut CompletionBuilder) {
    let available_functions = &ctx.schema_cache.functions;

    for foo in available_functions {
        let item = CompletionItem {
            label: foo.name.clone(),
            score: CompletionRelevanceData::Function(foo).get_score(ctx),
            description: format!("Schema: {}", foo.schema),
            preselected: false,
            kind: CompletionItemKind::Function,
        };

        builder.add_item(item);
    }
}
