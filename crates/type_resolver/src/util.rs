pub(crate) fn get_string_from_node(node: &sql_parser::pg_query_protobuf::Node) -> String {
    match &node.node {
        Some(sql_parser::AstNode::String(s)) => s.sval.to_string(),
        _ => "".to_string(),
    }
}
