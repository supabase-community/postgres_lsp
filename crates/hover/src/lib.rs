use base_db::{Document, Statement};
use schema_cache::SchemaCache;
use text_size::{TextRange, TextSize};

#[derive(Debug)]
pub struct HoverParams<'a> {
    pub schema_cache: &'a SchemaCache,
    pub offset: TextSize,
}

#[derive(Debug, Clone)]
pub struct Hover {
    pub range: TextRange,
    pub data: HoverData,
}

#[derive(Debug, Clone)]
pub enum HoverData {
    Column,
}

pub trait HoverFeature {
    fn hover(&self, params: HoverParams) -> Option<Hover>;
}

impl HoverFeature for Statement {
    fn hover(&self, params: HoverParams) -> Option<Hover> {
        todo!(
            "to make it real simple to implement these features, we should provide a way to get the ast node at a position. this can be done by a codegen method similar to get_nodes() that stores the node alongside a range."
        )
    }
}

impl HoverFeature for Document {
    fn hover(&self, params: HoverParams) -> Option<Hover> {
        let statement = self.statement_at_offset(params.offset)?;
        statement
            .hover(HoverParams {
                offset: params.offset - statement.range.start(),
                schema_cache: params.schema_cache,
            })
            .map(|hover| Hover {
                range: hover.range + statement.range.start(),
                data: hover.data,
            })
    }
}
