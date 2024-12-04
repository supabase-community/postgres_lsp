use crate::{item::CompletionItemWithRelevance, CompletionResult};

pub(crate) struct CompletionBuilder {
    items: Vec<CompletionItemWithRelevance>,
}

impl CompletionBuilder {
    pub fn new() -> Self {
        CompletionBuilder { items: vec![] }
    }

    pub fn add_item(&mut self, item: CompletionItemWithRelevance) {
        self.items.push(item)
    }

    pub fn finish(mut self) -> CompletionResult {
        self.items.sort_by(|a, b| {
            b.score()
                .cmp(&a.score())
                .then_with(|| a.label().cmp(&b.label()))
        });

        self.items.dedup_by(|a, b| a.label() == b.label());
        self.items.truncate(crate::LIMIT);

        let Self { items, .. } = self;

        CompletionResult { items }
    }
}
