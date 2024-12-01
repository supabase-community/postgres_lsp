use pg_schema_cache::SchemaCache;
use text_size::TextSize;

use crate::CompletionParams;

pub(crate) struct CompletionContext<'a> {
    pub ts_node: Option<tree_sitter::Node<'a>>,
    pub tree: Option<&'a tree_sitter::Tree>,
    pub text: &'a str,
    pub schema_cache: &'a SchemaCache,
    pub original_token: Option<char>,
    pub position: TextSize,
}

impl<'a> CompletionContext<'a> {
    pub fn new(params: &'a CompletionParams) -> Self {
        let ts_node = find_ts_node(&params);

        Self {
            ts_node,
            tree: params.tree,
            text: &params.text,
            schema_cache: params.schema,
            original_token: find_original_token(params),
            position: params.position,
        }
    }

    pub fn get_ts_node_content(&self, ts_node: tree_sitter::Node<'a>) -> Option<&'a str> {
        let source = self.text;
        match ts_node.utf8_text(source.as_bytes()) {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}

fn find_original_token<'a>(params: &CompletionParams) -> Option<char> {
    let idx = usize::from(params.position);
    params.text.chars().nth(idx)
}

fn find_ts_node<'a>(params: &'a CompletionParams) -> Option<tree_sitter::Node<'a>> {
    let tree = params.tree.as_ref()?;

    let mut node = tree.root_node().named_descendant_for_byte_range(
        usize::from(params.position),
        usize::from(params.position),
    )?;

    let node_range = node.range();
    while let Some(parent) = node.parent() {
        if parent.range() != node_range {
            break;
        }

        node = parent;
    }

    Some(node)
}
