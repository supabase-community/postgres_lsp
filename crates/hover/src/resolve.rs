use sql_parser::{AstNode, EnrichedAst};
use text_size::{TextRange, TextSize};
use tree_sitter::Tree;

#[derive(Debug, Eq, PartialEq)]
pub struct HoverableRelation {
    pub name: String,
    pub schema: Option<String>,
    pub range: TextRange,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HoverableColumn {
    pub name: String,
    pub table: Option<String>,
    pub schema: Option<String>,
    pub range: TextRange,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Hoverable {
    Relation(HoverableRelation),
    Column(HoverableColumn),
}

pub fn resolve_from_enriched_ast(pos: TextSize, ast: &EnrichedAst) -> Option<Hoverable> {
    let node = ast.covering_node(TextRange::empty(pos))?;

    match node.node {
        AstNode::RangeVar(ref range_var) => Some(Hoverable::Relation(HoverableRelation {
            range: node.range(),
            name: range_var.relname.clone(),
            schema: if range_var.schemaname.is_empty() {
                None
            } else {
                Some(range_var.schemaname.clone())
            },
        })),
        _ => None,
    }
}

pub fn resolve_from_tree_sitter(pos: TextSize, tree: &Tree, source: &str) -> Option<Hoverable> {
    let mut node = tree
        .root_node()
        .named_descendant_for_byte_range(usize::from(pos), usize::from(pos))?;

    let node_range = node.range();

    while let Some(parent) = node.parent() {
        if parent.range() != node_range {
            break;
        }
        node = parent;
    }

    match node.kind() {
        "relation" => Some(Hoverable::Relation(HoverableRelation {
            range: TextRange::new(
                TextSize::try_from(node.range().start_byte).unwrap(),
                TextSize::try_from(node.range().end_byte).unwrap(),
            ),
            name: node.utf8_text(source.as_bytes()).unwrap().to_string(),
            schema: None,
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use sql_parser::parse_ast;
    use text_size::{TextRange, TextSize};

    use super::{Hoverable, HoverableRelation};

    #[test]
    fn test_resolve_from_enriched_ast() {
        let input = "select id from contact;";
        let position = TextSize::new(15);

        let root = sql_parser::parse_sql_statement(input).unwrap();
        let ast = parse_ast(input, &root).ast;

        let hover = super::resolve_from_enriched_ast(position, &ast);

        assert!(hover.is_some());

        assert_eq!(
            hover.unwrap(),
            Hoverable::Relation(HoverableRelation {
                range: TextRange::new(TextSize::new(15), TextSize::new(22)),
                name: "contact".to_string(),
                schema: None,
            })
        );
    }

    #[test]
    fn test_resolve_from_tree_sitter() {
        let input = "select id from contact;";
        let position = TextSize::new(15);

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(input, None).unwrap();

        let hover = super::resolve_from_tree_sitter(position, &tree, input);

        assert!(hover.is_some());

        assert_eq!(
            hover.unwrap(),
            Hoverable::Relation(HoverableRelation {
                range: TextRange::new(TextSize::new(15), TextSize::new(22)),
                name: "contact".to_string(),
                schema: None,
            })
        );
    }
}
