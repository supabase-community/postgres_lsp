#[derive(Debug)]
pub enum CompletionItemKind {
    Table,
    Function,
}

#[derive(Debug)]
pub struct CompletionItem {
    pub label: String,
    pub(crate) score: i32,
    pub description: String,
    pub preselected: bool,
    pub kind: CompletionItemKind,
}
