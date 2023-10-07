use std::cmp::{max, min};

use crate::get_child_token_range_codegen::get_child_token_range;
use crate::get_location_codegen::get_location;
use crate::get_nodes_codegen::Node;
use cstree::text::{TextRange, TextSize};
use log::debug;
use pg_query::{protobuf::ScanToken, protobuf::Token, NodeEnum};

#[derive(Debug, Clone)]
pub struct RangedNode {
    pub inner: Node,
    pub range: TextRange,
}

/// Turns a `Vec<Node>` into a `Vec<RangedNode>` by estimating their range.
pub fn estimate_node_range(
    nodes: &mut Vec<Node>,
    tokens: &Vec<ScanToken>,
    text: &str,
) -> Vec<RangedNode> {
    let mut ranged_nodes: Vec<RangedNode> = Vec::new();

    let mut used_tokens: Vec<i32> = Vec::new();

    // ensure that all children of any given node are already processed before processing the node itself
    nodes.sort_by(|a, b| b.path.cmp(&a.path));

    // we get an estimated range by searching for tokens that match the node property values
    // and, if available, the `location` of the node itself
    nodes.iter().for_each(|n| {
        // first, get the estimated boundaries of the node based on the `location` property of a node
        let nearest_parent_location = get_nearest_parent_location(&n, nodes);

        let child_token_range = get_child_token_range(
            &n.node,
            tokens
                .iter()
                .filter(|t| !used_tokens.contains(&t.start))
                .collect(),
            text,
            nearest_parent_location,
        );

        used_tokens.extend(child_token_range.child_token_indices);

        // For `from`, the location of the node itself is always correct.
        // If not available, the closest estimation is the smaller value of the start of the first direct child token,
        // and the start of all children ranges. If neither is available, let’s panic for now.
        // The parent location as a fallback should never be required, because any node must have either children with tokens, or a token itself.
        let child_node_ranges = ranged_nodes
            .iter()
            .filter(|x| x.inner.path.starts_with(n.path.as_str()))
            .collect::<Vec<&RangedNode>>();
        let location = get_location(&n.node);
        let from = if location.is_some() {
            Some(TextSize::from(location.unwrap()))
        } else {
            let start_of_all_children_ranges = if child_node_ranges.len() > 0 {
                Some(
                    child_node_ranges
                        .iter()
                        .min_by_key(|n| n.range.start())
                        .unwrap()
                        .range
                        .start(),
                )
            } else {
                None
            };

            if child_token_range.range.is_some() {
                let start_of_first_child_token = child_token_range.range.unwrap().start();
                if start_of_all_children_ranges.is_some() {
                    Some(min(
                        start_of_first_child_token,
                        start_of_all_children_ranges.unwrap(),
                    ))
                } else {
                    Some(start_of_first_child_token)
                }
            } else if start_of_all_children_ranges.is_some() {
                Some(start_of_all_children_ranges.unwrap())
            } else {
                debug!("No location or child tokens found for node {:?}", n);
                None
            }
        };

        // For `to`, it’s the larger value of the end of the last direkt child token, and the end of all children ranges.
        let end_of_all_children_ranges = if child_node_ranges.len() > 0 {
            Some(
                child_node_ranges
                    .iter()
                    .max_by_key(|n| n.range.end())
                    .unwrap()
                    .range
                    .end(),
            )
        } else {
            None
        };
        let to = if child_token_range.range.is_some() {
            let end_of_last_child_token = child_token_range.range.unwrap().end();
            if end_of_all_children_ranges.is_some() {
                Some(max(
                    end_of_last_child_token,
                    end_of_all_children_ranges.unwrap(),
                ))
            } else {
                Some(end_of_last_child_token)
            }
        } else if end_of_all_children_ranges.is_some() {
            Some(end_of_all_children_ranges.unwrap())
        } else {
            debug!("No child tokens or children ranges found for node {:?}", n);
            None
        };

        if from.is_some() && to.is_some() {
            ranged_nodes.push(RangedNode {
                inner: n.to_owned(),
                range: TextRange::new(from.unwrap(), to.unwrap()),
            });
        }
    });

    // sort by start of range, and then by depth
    ranged_nodes.sort_by_key(|i| (i.range.start(), i.inner.depth));

    ranged_nodes
}

fn get_nearest_parent_location(n: &Node, children: &Vec<Node>) -> u32 {
    // if location is set, return it
    let location = get_location(&n.node);
    if location.is_some() {
        return location.unwrap();
    }

    // go up in the tree and check if location exists on any parent
    let mut path_elements = n.path.split(".").collect::<Vec<&str>>();
    path_elements.pop();
    while path_elements.len() > 0 {
        let parent_path = path_elements.join(".");
        let node = children.iter().find(|c| c.path == parent_path);
        if node.is_some() {
            let location = get_location(&node.unwrap().node);
            if location.is_some() {
                return location.unwrap();
            }
        }

        path_elements.pop();
    }

    // fallback to 0
    return 0;
}

#[cfg(test)]
mod tests {
    use cstree::text::{TextRange, TextSize};
    use pg_query::NodeEnum;

    use crate::estimate_node_range::estimate_node_range;
    use crate::get_nodes_codegen::get_nodes;

    #[test]
    fn test_estimate_node_range() {
        let input = "select null";

        let pg_query_tokens = match pg_query::scan(input) {
            Ok(scanned) => scanned.tokens,
            Err(_) => Vec::new(),
        };

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

        let mut nodes = get_nodes(&pg_query_root.unwrap(), input.to_string(), 1);

        let ranged_nodes = estimate_node_range(&mut nodes, &pg_query_tokens, &input);

        assert!(ranged_nodes
            .iter()
            .find(
                |n| n.range == TextRange::new(TextSize::from(0), TextSize::from(11))
                    && match &n.inner.node {
                        NodeEnum::SelectStmt(_) => true,
                        _ => false,
                    }
            )
            .is_some());

        assert!(ranged_nodes
            .iter()
            .find(
                |n| n.range == TextRange::new(TextSize::from(7), TextSize::from(11))
                    && match &n.inner.node {
                        NodeEnum::ResTarget(_) => true,
                        _ => false,
                    }
            )
            .is_some());

        assert!(ranged_nodes
            .iter()
            .find(
                |n| n.range == TextRange::new(TextSize::from(7), TextSize::from(11))
                    && match &n.inner.node {
                        NodeEnum::AConst(_) => true,
                        _ => false,
                    }
            )
            .is_some());
    }
}
