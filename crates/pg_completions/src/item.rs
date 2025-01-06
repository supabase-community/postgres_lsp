use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Table,
    Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub(crate) score: i32,
    pub description: String,
    pub preselected: bool,
    pub kind: CompletionItemKind,
}
