pub(crate) fn get_string_from_node(node: &pg_query_ext::protobuf::Node) -> String {
    match &node.node {
        Some(pg_query_ext::NodeEnum::String(s)) => s.sval.to_string(),
        _ => "".to_string(),
    }
}
