use tower_lsp::lsp_types::{self, CompletionItem};

use crate::{data::CompletionItemData, relevance::CompletionRelevance};

#[derive(Debug)]
pub struct CompletionItemWithScore {
    pub item: lsp_types::CompletionItem,
    pub score: i32,
}

impl CompletionItemWithScore {
    pub(crate) fn new(data: CompletionItemData, relevance: CompletionRelevance) -> Self {
        Self {
            item: data.into(),
            score: relevance.score(),
        }
    }

    pub(crate) fn label(&self) -> &str {
        &self.item.label
    }

    pub(crate) fn set_preselected(&mut self, is_preselected: bool) {
        self.item.preselect = Some(is_preselected)
    }
}

impl Into<CompletionItem> for CompletionItemWithScore {
    fn into(self) -> CompletionItem {
        self.item
    }
}
