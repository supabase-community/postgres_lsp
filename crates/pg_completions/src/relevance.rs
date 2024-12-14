use crate::context::CompletionContext;

#[derive(Debug)]
pub(crate) enum CompletionRelevanceData<'a> {
    Table(&'a pg_schema_cache::Table),
    Function(&'a pg_schema_cache::Function),
}

impl<'a> CompletionRelevanceData<'a> {
    pub fn get_score(self, ctx: &CompletionContext) -> i32 {
        CompletionRelevance::from(self).into_score(ctx)
    }
}

impl<'a> From<CompletionRelevanceData<'a>> for CompletionRelevance<'a> {
    fn from(value: CompletionRelevanceData<'a>) -> Self {
        Self {
            score: 0,
            data: value,
        }
    }
}

#[derive(Debug)]
pub(crate) struct CompletionRelevance<'a> {
    score: i32,
    data: CompletionRelevanceData<'a>,
}

impl<'a> CompletionRelevance<'a> {
    pub fn into_score(mut self, ctx: &CompletionContext) -> i32 {
        self.check_matches_schema(ctx);
        self.check_matches_query_input(ctx);
        self.check_if_catalog(ctx);

        self.score
    }

    fn check_matches_query_input(&mut self, ctx: &CompletionContext) {
        let node = ctx.ts_node.unwrap();

        let content = match ctx.get_ts_node_content(node) {
            Some(c) => c,
            None => return,
        };

        let name = match self.data {
            CompletionRelevanceData::Function(f) => f.name.as_str(),
            CompletionRelevanceData::Table(t) => t.name.as_str(),
        };

        if name.starts_with(content) {
            let len: i32 = content
                .len()
                .try_into()
                .expect("The length of the input exceeds i32 capacity");

            self.score += len * 5;
        };
    }

    fn check_matches_schema(&mut self, ctx: &CompletionContext) {
        let schema_name = match ctx.schema_name.as_ref() {
            None => return,
            Some(n) => n,
        };

        let data_schema = match self.data {
            CompletionRelevanceData::Function(f) => f.schema.as_str(),
            CompletionRelevanceData::Table(t) => t.schema.as_str(),
        };

        if schema_name == data_schema {
            self.score += 25;
        } else {
            self.score -= 10;
        }
    }

    fn check_if_catalog(&mut self, ctx: &CompletionContext) {
        if ctx.schema_name.as_ref().is_some_and(|n| n == "pg_catalog") {
            return;
        }

        self.score -= 5; // unlikely that the user wants schema data
    }
}
