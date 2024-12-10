#[derive(Debug)]
pub enum CompletionItemKind {
    Table,
}

#[derive(Debug)]
pub struct CompletionItem {
    pub label: String,
    pub(crate) score: i32,
    pub description: String,
    pub preselected: Option<bool>,
    pub kind: CompletionItemKind,
}

impl From<CompletionItem> for tower_lsp::lsp_types::CompletionItem {
    fn from(i: CompletionItem) -> Self {
        tower_lsp::lsp_types::CompletionItem {
            label: i.label,
            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                description: Some(i.description),
                detail: None,
            }),
            kind: Some(i.kind.into()),
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
        }
    }
}

impl From<CompletionItemKind> for tower_lsp::lsp_types::CompletionItemKind {
    fn from(value: CompletionItemKind) -> Self {
        match value {
            CompletionItemKind::Table => tower_lsp::lsp_types::CompletionItemKind::CLASS,
        }
    }
}
