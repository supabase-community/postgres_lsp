use text_size::TextRange;

#[derive(Debug, PartialEq, Eq)]
pub enum CompletionItemData {
    Table(pg_schema_cache::Table),
}

impl CompletionItemData {
    pub fn label(&self) -> &str {
        match self {
            CompletionItemData::Table(t) => t.name.as_str(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompletionItem {
    pub score: i32,
    pub range: TextRange,
    pub preselect: bool,
    pub data: CompletionItemData,
}

impl CompletionItem {
    pub fn new_simple(score: i32, range: TextRange, data: CompletionItemData) -> Self {
        Self {
            score,
            range,
            preselect: false,
            data,
        }
    }
}
