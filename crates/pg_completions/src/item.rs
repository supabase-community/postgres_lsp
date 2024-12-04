use tower_lsp::lsp_types;

use crate::{data::CompletionItemData, relevance::CompletionRelevance};

#[derive(Debug)]
pub struct CompletionItemWithRelevance {
    item: lsp_types::CompletionItem,
    relevance: CompletionRelevance,
}

impl CompletionItemWithRelevance {
    pub(crate) fn new(data: CompletionItemData, relevance: CompletionRelevance) -> Self {
        Self {
            item: data.into(),
            relevance,
        }
    }

    pub(crate) fn score(&self) -> i32 {
        self.relevance.score()
    }

    pub(crate) fn label(&self) -> &str {
        &self.item.label
    }
}
