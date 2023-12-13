use codegen::parser_codegen;

parser_codegen!();

#[cfg(test)]
mod tests {
    use log::debug;

    use crate::codegen::{get_nodes, SyntaxKind, TokenProperty};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_get_nodes() {
        init();

        let input = "with c as (insert into contact (id) values ('id')) select * from c;";

        let pg_query_root = match pg_query::parse(input) {
            Ok(parsed) => Some(
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .find(|n| n.1 == 1)
                    .unwrap()
                    .0
                    .to_enum(),
            ),
            Err(_) => None,
        };

        let node_graph = get_nodes(&pg_query_root.unwrap(), 0);
        assert_eq!(node_graph.node_count(), 13);
    }

    fn test_get_node_properties(input: &str, kind: SyntaxKind, expected: Vec<TokenProperty>) {
        init();

        let pg_query_root = match pg_query::parse(input) {
            Ok(parsed) => Some(
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .find(|n| n.1 == 1)
                    .unwrap()
                    .0
                    .to_enum(),
            ),
            Err(_) => None,
        };

        debug!("pg_query_root: {:#?}", pg_query_root);

        let node_graph = get_nodes(&pg_query_root.unwrap(), 0);

        debug!("node graph: {:#?}", node_graph);

        let node_index = node_graph
            .node_indices()
            .find(|n| node_graph[*n].kind == kind)
            .unwrap();

        debug!("selected node: {:#?}", node_graph[node_index]);

        // note: even though we test for strict equality of the two vectors the order
        // of the properties does not have to match the order of the tokens in the string
        assert_eq!(node_graph[node_index].properties, expected);
        assert_eq!(node_graph[node_index].properties.len(), expected.len());
    }

    #[test]
    fn test_simple_select() {
        test_get_node_properties(
            "select 1;",
            SyntaxKind::SelectStmt,
            vec![TokenProperty::from(SyntaxKind::Select)],
        )
    }

    #[test]
    fn test_select_with_from() {
        test_get_node_properties(
            "select 1 from contact;",
            SyntaxKind::SelectStmt,
            vec![
                TokenProperty::from(SyntaxKind::Select),
                TokenProperty::from(SyntaxKind::From),
            ],
        )
    }

    #[test]
    fn test_create_domain() {
        test_get_node_properties(
            "create domain us_postal_code as text check (value is not null);",
            SyntaxKind::CreateDomainStmt,
            vec![
                TokenProperty::from(SyntaxKind::Create),
                TokenProperty::from(SyntaxKind::DomainP),
                TokenProperty::from(SyntaxKind::As),
                TokenProperty::from("us_postal_code".to_string()),
            ],
        )
    }

    #[test]
    fn test_create_schema() {
        test_get_node_properties(
            "create schema if not exists test authorization joe;",
            SyntaxKind::CreateSchemaStmt,
            vec![
                TokenProperty::from(SyntaxKind::Create),
                TokenProperty::from(SyntaxKind::Schema),
                TokenProperty::from(SyntaxKind::IfP),
                TokenProperty::from(SyntaxKind::Not),
                TokenProperty::from(SyntaxKind::Exists),
                TokenProperty::from(SyntaxKind::Authorization),
                TokenProperty::from("test".to_string()),
            ],
        )
    }

    #[test]
    fn test_create_view() {
        test_get_node_properties(
            "create or replace temporary view comedies as select * from films;",
            SyntaxKind::ViewStmt,
            vec![
                TokenProperty::from(SyntaxKind::Create),
                TokenProperty::from(SyntaxKind::View),
                TokenProperty::from(SyntaxKind::As),
                TokenProperty::from(SyntaxKind::Or),
                TokenProperty::from(SyntaxKind::Replace),
                TokenProperty::from(SyntaxKind::Temporary),
            ],
        )
    }

    #[test]
    fn test_create_enum() {
        test_get_node_properties(
            "create type status as enum ('open', 'closed');",
            SyntaxKind::CreateEnumStmt,
            vec![
                TokenProperty::from(SyntaxKind::Create),
                TokenProperty::from(SyntaxKind::TypeP),
                TokenProperty::from(SyntaxKind::As),
                TokenProperty::from(SyntaxKind::EnumP),
                TokenProperty::from("status".to_string()),
                TokenProperty::from("open".to_string()),
                TokenProperty::from("closed".to_string()),
            ],
        )
    }

    #[test]
    fn test_create_cast() {
        test_get_node_properties(
            "create cast (bigint as int4) with inout as assignment;",
            SyntaxKind::CreateCastStmt,
            vec![
                TokenProperty::from(SyntaxKind::Create),
                TokenProperty::from(SyntaxKind::Cast),
                TokenProperty::from(SyntaxKind::As),
                TokenProperty::from(SyntaxKind::With),
                TokenProperty::from(SyntaxKind::Inout),
                TokenProperty::from(SyntaxKind::As),
                TokenProperty::from(SyntaxKind::Assignment),
            ],
        )
    }
}
