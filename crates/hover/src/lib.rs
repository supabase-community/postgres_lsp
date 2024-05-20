use schema_cache::SchemaCache;
use text_size::{TextRange, TextSize};

pub struct HoverParams {
    pub position: text_size::TextSize,
    pub ast: Option<sql_parser::EnrichedAst>,
    pub tree: tree_sitter::Tree,
    pub schema_cache: SchemaCache,
}

pub struct HoverResult {
    range: Option<TextRange>,
    content: String,
}

pub fn hover(params: HoverParams) -> Option<HoverResult> {
    // if ast, find deepest node at position
    if params.ast.is_some() {
        let ast = params.ast.unwrap();
        let node = ast.covering_node(TextRange::empty(params.position));
        if node.is_some() {
            let node = node.unwrap();
            return Some(HoverResult {
                range: Some(node.range()),
                content: "Hover".to_string(),
            });
        }
    }
    // else, try to find ts node at position in tree
    // using: https://docs.rs/tree-sitter/0.22.6/tree_sitter/struct.TreeCursor.html#method.goto_first_child_for_byte
    // get byte offset from position
    let r = params.tree.root_node().named_descendant_for_byte_range(
        usize::from(params.position),
        usize::from(params.position),
    );
    if r.is_some() {
        let r = r.unwrap();
        match r.kind() {}
    }

    // TODO: get table / column whatever COMMENT from schema cache

    None
}
