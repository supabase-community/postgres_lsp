use schema_cache::SchemaCache;
use text_size::TextSize;

use crate::functions_args::FunctionArgHint;

pub struct InlayHintsParams<'a> {
    pub ast: Option<&'a sql_parser::AstNode>,
    pub enriched_ast: Option<&'a sql_parser::EnrichedAst>,
    pub tree: Option<&'a tree_sitter::Tree>,
    pub cst: Option<&'a sql_parser::Cst>,
    pub schema_cache: &'a SchemaCache,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InlayHintContent {
    FunctionArg(FunctionArgHint),
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlayHint {
    pub offset: TextSize,
    pub content: InlayHintContent,
}

pub trait InlayHintsResolver {
    fn find_all(params: InlayHintsParams) -> Vec<InlayHint>;
}
