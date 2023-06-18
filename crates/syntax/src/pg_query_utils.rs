use pg_query::NodeRef;

/// Gets the position value for a pg_query node
/// can mostly be generated.
/// there are some exceptions where the location on the node itself is not leftmost position, e.g. for AExpr.
pub fn get_position_for_pg_query_node(node: &NodeRef) -> i32 {
    match node {
        NodeRef::ResTarget(n) => n.location,
        NodeRef::AExpr(n) => get_position_for_pg_query_node(
            &n.lexpr.as_ref().unwrap().node.as_ref().unwrap().to_ref(),
        ),
        NodeRef::RangeVar(n) => n.location,
        NodeRef::ColumnRef(n) => n.location,
        NodeRef::AConst(n) => n.location,
        _ => -1,
    }
}
