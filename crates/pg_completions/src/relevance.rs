use crate::{context::CompletionContext, data::CompletionItemData};

#[derive(Debug, Default)]
pub(crate) struct CompletionRelevance {
    /// does the underlying data match the expected schema we can determine from the query?
    matches_schema: bool,

    /// Is the underlying item from the pg_catalog schema?
    is_catalog: bool,

    /// Do the characters the users typed match at least the first 3 characters
    /// of the underlying data's name?
    matches_prefix: usize,
}

impl CompletionRelevance {
    pub fn from_data_and_ctx(data: &CompletionItemData, ctx: &CompletionContext) -> Self {
        let mut relevance = CompletionRelevance::default();

        match data {
            CompletionItemData::Table(tb) => {
                relevance.set_is_catalog(&tb.schema);
                relevance.set_matches_schema(ctx, &tb.schema);
                relevance.set_matches_prefix(ctx, &tb.name);
            }
        }

        relevance
    }

    pub fn score(&self) -> i32 {
        let mut score: i32 = 0;

        if self.matches_schema {
            score += 5;
        } else if self.is_catalog {
            score -= 1;
        }

        score += (self.matches_prefix * 5) as i32;

        score
    }

    pub fn set_matches_schema(&mut self, ctx: &CompletionContext, schema: &str) {
        let node = ctx.ts_node.unwrap();
        self.matches_schema = node
            .prev_named_sibling()
            .is_some_and(|n| ctx.get_ts_node_content(n).is_some_and(|c| c == schema));
    }

    pub fn set_is_catalog(&mut self, schema: &str) {
        self.is_catalog = schema == "pg_catalog"
    }

    pub fn set_matches_prefix(&mut self, ctx: &CompletionContext, name: &str) {
        let node = ctx.ts_node.unwrap();

        let content = match ctx.get_ts_node_content(node) {
            Some(c) => c,
            None => return,
        };

        if name.starts_with(content) {
            self.matches_prefix = content.len();
        };
    }
}
