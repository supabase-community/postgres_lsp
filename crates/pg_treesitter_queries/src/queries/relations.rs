use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::{Query, QueryResult};

use super::QueryTryFrom;

static INSTANCE: OnceCell<Arc<tree_sitter::Query>> = OnceCell::const_new();

static QUERY: &'static str = r#"
    (relation
        (object_reference 
            .
            (identifier) @schema_or_table
            "."?
            (identifier)? @table
        )+
    )
"#;

pub struct RelationMatch<'a> {
    pub(crate) schema: Option<tree_sitter::Node<'a>>,
    pub(crate) table: tree_sitter::Node<'a>,
}

impl<'a> TryFrom<&'a QueryResult<'a>> for &'a RelationMatch<'a> {
    type Error = String;

    fn try_from(q: &'a QueryResult<'a>) -> Result<Self, Self::Error> {
        match q {
            QueryResult::Relation(r) => Ok(&r),

            #[allow(unreachable_patterns)]
            _ => Err("Invalid QueryResult type".into()),
        }
    }
}

impl<'a> QueryTryFrom<'a> for RelationMatch<'a> {
    type Ref = &'a RelationMatch<'a>;
}

impl<'a> Query<'a> for RelationMatch<'a> {
    async fn execute(
        root_node: tree_sitter::Node<'a>,
        stmt: &'a str,
    ) -> Vec<crate::QueryResult<'a>> {
        let query = INSTANCE
            .get_or_init(|| async {
                Arc::new(
                    tree_sitter::Query::new(tree_sitter_sql::language(), &QUERY)
                        .expect("Invalid Query."),
                )
            })
            .await;

        let mut cursor = tree_sitter::QueryCursor::new();

        let matches = cursor.matches(&query, root_node, stmt.as_bytes());

        let mut to_return = vec![];

        for m in matches {
            if m.captures.len() == 1 {
                let capture = m.captures[0].node;
                to_return.push(QueryResult::Relation(RelationMatch {
                    schema: None,
                    table: capture,
                }));
            }

            if m.captures.len() == 2 {
                let schema = m.captures[0].node;
                let table = m.captures[1].node;

                to_return.push(QueryResult::Relation(RelationMatch {
                    schema: Some(schema),
                    table,
                }));
            }
        }

        to_return
    }
}
