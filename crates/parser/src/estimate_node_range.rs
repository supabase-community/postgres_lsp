use std::cmp::max;

use crate::get_child_token_range_codegen::{get_child_token_range, ChildTokenRangeResult};
use crate::get_location_codegen::get_location;
use crate::get_nodes_codegen::Node;
use cstree::text::{TextRange, TextSize};
use pg_query::protobuf::ScanToken;

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
    // ensure that all children of any given node are already processed before processing the node itself
    nodes.sort_by(|a, b| b.path.cmp(&a.path));

    // first get ranges only from child tokens
    let mut used_tokens: Vec<i32> = Vec::new();
    let mut child_token_ranges: Vec<Option<TextRange>> = Vec::new();
    let mut too_many_tokens_at: Vec<usize> = Vec::new();

    nodes.iter().for_each(|n| {
        match get_child_token_range(
            &n.node,
            tokens
                .iter()
                .filter(|t| !used_tokens.contains(&t.start))
                .collect(),
            text,
            None,
        ) {
            ChildTokenRangeResult::TooManyTokens => {
                too_many_tokens_at.push(nodes.iter().position(|x| x.path == n.path).unwrap());
                child_token_ranges.push(None);
            }
            ChildTokenRangeResult::ChildTokenRange {
                used_token_indices,
                range,
            } => {
                used_tokens.extend(used_token_indices);
                child_token_ranges.push(Some(range));
            }
            ChildTokenRangeResult::NoTokens => {
                child_token_ranges.push(None);
            }
        };
    });

    // second iteration using the nearest parent from the first
    for idx in too_many_tokens_at {
        // get the nearest parent location
        let nearest_parent_start =
            get_nearest_parent_start(&nodes[idx], &nodes, &child_token_ranges);
        let nearest_parent_location = get_nearest_parent_location(&nodes[idx], &nodes);

        match get_child_token_range(
            &nodes[idx].node,
            tokens
                .iter()
                .filter(|t| !used_tokens.contains(&t.start))
                .collect(),
            text,
            Some(max(nearest_parent_start, nearest_parent_location)),
        ) {
            ChildTokenRangeResult::ChildTokenRange {
                used_token_indices,
                range,
            } => {
                used_tokens.extend(used_token_indices);
                child_token_ranges[idx] = Some(range)
            }
            _ => {}
        };
    }

    let mut ranged_nodes: Vec<RangedNode> = Vec::new();

    // we get an estimated range by searching for tokens that match the node property values
    // and, if available, the `location` of the node itself
    nodes.iter().enumerate().for_each(|(idx, n)| {
        let child_token_range = child_token_ranges[idx];

        println!("node: {:#?}, child_token_range: {:?}", n, child_token_range);

        let child_node_ranges = ranged_nodes
            .iter()
            .filter(|x| x.inner.path.starts_with(n.path.as_str()))
            .collect::<Vec<&RangedNode>>();

        // get `from` location
        let node_location = match get_location(&n.node) {
            Some(l) => Some(TextSize::from(l)),
            None => None,
        };
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
        let start_of_first_child_token = match child_token_range {
            Some(r) => Some(r.start()),
            None => None,
        };

        let from_locations: [Option<TextSize>; 3] = [
            node_location,
            start_of_all_children_ranges,
            start_of_first_child_token,
        ];
        let from = from_locations.iter().filter(|v| v.is_some()).min();

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
        let end_of_last_child_token = match child_token_range {
            Some(r) => Some(r.end()),
            None => None,
        };
        let to_locations: [Option<TextSize>; 2] =
            [end_of_all_children_ranges, end_of_last_child_token];
        let to = to_locations.iter().filter(|v| v.is_some()).max();

        if from.is_some() && to.is_some() {
            ranged_nodes.push(RangedNode {
                inner: n.to_owned(),
                range: TextRange::new(from.unwrap().unwrap(), to.unwrap().unwrap()),
            });
        }
    });

    // sort by start of range, and then by depth
    ranged_nodes.sort_by_key(|i| (i.range.start(), i.inner.depth));

    ranged_nodes
}

fn get_nearest_parent_start(
    node: &Node,
    nodes: &Vec<Node>,
    child_token_ranges: &Vec<Option<TextRange>>,
) -> u32 {
    let mut path_elements = node.path.split(".").collect::<Vec<&str>>();
    path_elements.pop();
    while path_elements.len() > 0 {
        let parent_path = path_elements.join(".");
        let parent_idx = nodes.iter().position(|c| c.path == parent_path);
        if parent_idx.is_some() {
            if child_token_ranges[parent_idx.unwrap()].is_some() {
                return u32::from(child_token_ranges[parent_idx.unwrap()].unwrap().start());
            }
        }

        path_elements.pop();
    }

    // fallback to 0
    0
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
    0
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
