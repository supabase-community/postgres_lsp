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
