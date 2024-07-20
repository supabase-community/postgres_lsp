use text_size::{TextRange, TextSize};

use crate::{builder::CompletionBuilder, CompletionItem, CompletionItemData};

use super::CompletionProviderParams;

// todo unify this in a type resolver crate
pub fn complete_tables<'a>(
    params: CompletionProviderParams<'a>,
    builder: &mut CompletionBuilder<'a>,
) {
    if let Some(ts) = params.ts_node {
        let range = TextRange::new(
            TextSize::try_from(ts.start_byte()).unwrap(),
            TextSize::try_from(ts.end_byte()).unwrap(),
        );
        match ts.kind() {
            "relation" => {
                // todo better search
                params.schema.tables.iter().for_each(|table| {
                    builder.items.push(CompletionItem::new_simple(
                        1,
                        range,
                        CompletionItemData::Table(table),
                    ));
                });
            }
            _ => {}
        }
    }
}
