mod tables;

pub use tables::complete_tables;

use crate::CompletionParams;

#[derive(Debug)]
pub struct CompletionProviderParams<'a> {
    pub ts_node: Option<tree_sitter::Node<'a>>,
    pub schema: &'a schema_cache::SchemaCache,
    pub source: &'a str,
}

impl<'a> From<&'a CompletionParams<'a>> for CompletionProviderParams<'a> {
    fn from(params: &'a CompletionParams) -> Self {
        let ts_node = if let Some(tree) = params.tree {
            let node = tree.root_node().named_descendant_for_byte_range(
                usize::from(params.position),
                usize::from(params.position),
            );

            if let Some(mut n) = node {
                let node_range = n.range();

                while let Some(parent) = n.parent() {
                    if parent.range() != node_range {
                        break;
                    }

                    n = parent;
                }

                Some(n)
            } else {
                None
            }
        } else {
            None
        };

        Self {
            ts_node,
            schema: params.schema,
            source: params.text,
        }
    }
}
