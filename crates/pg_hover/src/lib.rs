//! # pg_hover
//!
//! This crate implements the hover feature. Essentially, it takes a position in a sql statement, and checks what node is located at that position. If the node is a valid hover result type, it resolves the type from the schema cache and returns it. The consumer of this crate is responsible for rendering the data.
//!
//! Note that we have two ways of resolving the hover result. We first try to resolve it from the enriched AST, and if that fails, we try to resolve it from the tree-sitter CST. This is because the enriched AST is more accurate, but the tree-sitter CST is more reliable.

mod resolve;

use pg_schema_cache::SchemaCache;
use resolve::Hoverable;
use text_size::TextRange;

pub struct HoverParams<'a> {
    pub position: text_size::TextSize,
    pub source: &'a str,
    pub enriched_ast: Option<&'a pg_syntax::AST>,
    pub tree: Option<&'a tree_sitter::Tree>,
    pub schema_cache: SchemaCache,
}

#[derive(Debug)]
pub struct HoverResult {
    pub range: Option<TextRange>,
    pub content: String,
}

pub fn hover(params: HoverParams) -> Option<HoverResult> {
    let elem = if params.enriched_ast.is_some() {
        resolve::resolve_from_enriched_ast(params.position, params.enriched_ast.unwrap())
    } else if params.tree.is_some() {
        resolve::resolve_from_tree_sitter(params.position, params.tree.unwrap(), &params.source)
    } else {
        None
    };

    if elem.is_none() {
        return None;
    }

    match elem.unwrap() {
        Hoverable::Relation(r) => {
            let table = params.schema_cache.find_table(&r.name, r.schema.as_deref());

            table.map(|t| {
                let mut content = t.name.to_owned();

                if t.comment.is_some() {
                    content.push_str("\n");
                    content.push_str(t.comment.as_ref().unwrap());
                }

                return HoverResult {
                    range: Some(r.range),
                    content,
                };
            })
        }
    }
}
