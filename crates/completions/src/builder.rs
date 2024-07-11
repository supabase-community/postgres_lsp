use crate::{CompletionItem, CompletionResult};

pub struct CompletionBuilder<'a> {
    pub items: Vec<CompletionItem<'a>>,
}

pub struct CompletionConfig {}

impl<'a> From<&'a CompletionConfig> for CompletionBuilder<'a> {
    fn from(config: &CompletionConfig) -> Self {
        Self { items: Vec::new() }
    }
}

impl<'a> CompletionBuilder<'a> {
    pub fn finish(mut self) -> CompletionResult<'a> {
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
