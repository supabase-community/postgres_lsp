use crate::{item::CompletionItem, CompletionResult};

pub struct CompletionBuilder {
    pub items: Vec<CompletionItem>,
}

pub struct CompletionConfig {}

impl From<&CompletionConfig> for CompletionBuilder {
    fn from(_config: &CompletionConfig) -> Self {
        Self { items: Vec::new() }
    }
}

impl CompletionBuilder {
    pub fn finish(mut self) -> CompletionResult {
        self.items.sort_by(|a, b| {
            b.preselect
                .cmp(&a.preselect)
                .then_with(|| b.score.cmp(&a.score))
                .then_with(|| a.data.label().cmp(b.data.label()))
        });

        self.items.dedup_by(|a, b| a.data.label() == b.data.label());
        self.items.truncate(crate::LIMIT);
        let Self { items, .. } = self;
        CompletionResult { items }
    }
}
