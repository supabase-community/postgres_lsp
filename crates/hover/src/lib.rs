mod resolve;

use resolve::Hoverable;
use schema_cache::SchemaCache;
use text_size::TextRange;

pub struct HoverParams<'a> {
    pub position: text_size::TextSize,
    pub source: &'a str,
    pub enriched_ast: Option<&'a sql_parser::EnrichedAst>,
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
        _ => None,
    }
}
