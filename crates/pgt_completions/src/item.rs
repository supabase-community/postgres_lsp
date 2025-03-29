#[derive(Debug, PartialEq, Eq)]
pub enum CompletionItemKind {
    Table,
    Function,
    Column,
}

#[derive(Debug)]
pub struct CompletionItem {
    pub label: String,
    pub score: i32,
    pub description: String,
    pub preselected: bool,
    pub kind: CompletionItemKind,
}
