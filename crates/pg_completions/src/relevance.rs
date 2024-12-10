use crate::context::CompletionContext;

#[derive(Debug, Default)]
pub(crate) struct CompletionRelevance {
    score: i32,
}

impl CompletionRelevance {
    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn check_matches_query_input(&mut self, ctx: &CompletionContext, name: &str) {
        let node = ctx.ts_node.unwrap();

        let content = match ctx.get_ts_node_content(node) {
            Some(c) => c,
            None => return,
        };

        if name.starts_with(content) {
            let len: i32 = content
                .len()
                .try_into()
                .expect("The length of the input exceeds i32 capacity");

            self.score += len * 5;
        };
    }

    pub fn check_matches_schema(&mut self, ctx: &CompletionContext, schema: &str) {
        if ctx.schema_name.is_none() {
            return;
        }

        let name = ctx.schema_name.as_ref().unwrap();

        if name == schema {
            self.score += 25;
        } else {
            self.score -= 10;
        }
    }

    pub fn check_if_catalog(&mut self, ctx: &CompletionContext) {
        if ctx.schema_name.as_ref().is_some_and(|n| n == "pg_catalog") {
            return;
        }

        self.score -= 5; // unlikely that the user wants schema data
    }
}
