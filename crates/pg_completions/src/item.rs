use text_size::TextRange;

use crate::relevance::CompletionRelevance;

#[derive(Debug)]
pub enum CompletionItemData<'a> {
    Table(&'a pg_schema_cache::Table),
}

impl<'a> CompletionItemData<'a> {
    pub fn label(&self) -> String {
        match self {
            CompletionItemData::Table(t) => t.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CompletionItem {
    pub range: TextRange,
    pub label: String,
    relevance: CompletionRelevance,
}

impl CompletionItem {
    pub(crate) fn new(
        range: TextRange,
        data: CompletionItemData,
        relevance: CompletionRelevance,
    ) -> Self {
        Self {
            range,
            label: data.label(),
            relevance,
        }
    }

    pub(crate) fn score(&self) -> i32 {
        self.relevance.score()
    }
}
