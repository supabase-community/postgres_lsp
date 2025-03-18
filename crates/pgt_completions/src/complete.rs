use pgt_text_size::TextSize;
use serde::{Deserialize, Serialize};

use crate::{
    builder::CompletionBuilder,
    context::CompletionContext,
    item::CompletionItem,
    providers::{complete_columns, complete_functions, complete_tables},
};

pub const LIMIT: usize = 50;

#[derive(Debug)]
pub struct CompletionParams<'a> {
    pub position: TextSize,
    pub schema: &'a pgt_schema_cache::SchemaCache,
    pub text: String,
    pub tree: Option<&'a tree_sitter::Tree>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompletionResult {
    pub(crate) items: Vec<CompletionItem>,
}

impl IntoIterator for CompletionResult {
    type Item = CompletionItem;
    type IntoIter = <Vec<CompletionItem> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

pub fn complete(params: CompletionParams) -> CompletionResult {
    let ctx = CompletionContext::new(&params);

    let mut builder = CompletionBuilder::new();

    complete_tables(&ctx, &mut builder);
    complete_functions(&ctx, &mut builder);
    complete_columns(&ctx, &mut builder);

    builder.finish()
}
