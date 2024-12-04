use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

#[derive(Debug)]
pub(crate) enum CompletionItemData<'a> {
    Table(&'a pg_schema_cache::Table),
}

impl<'a> Into<CompletionItem> for CompletionItemData<'a> {
    fn into(self) -> CompletionItem {
        match self {
            Self::Table(tb) => CompletionItem {
                label: tb.name.clone(),
                label_details: Some(CompletionItemLabelDetails {
                    description: Some(format!("Schema: {}", tb.schema)),
                    detail: None,
                }),
                kind: Some(CompletionItemKind::CLASS),
                detail: None,
                documentation: None,
                deprecated: None,
                preselect: None,
                sort_text: None,
                filter_text: None,
                insert_text: None,
                insert_text_format: None,
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                commit_characters: None,
                data: None,
                tags: None,
                command: None,
            },
        }
    }
}
