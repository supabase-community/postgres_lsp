use std::{
    cmp::{max, min},
    convert::identity,
};

use crate::get_location_codegen::get_location;
use crate::get_nodes_codegen::Node;
use cstree::text::{TextRange, TextSize};
use pg_query::{protobuf::ScanToken, NodeEnum};

#[derive(Debug, Clone)]
pub struct RangedNode {
    pub inner: Node,
    pub estimated_range: TextRange,
}

/// Turns a `Vec<Node>` into a `Vec<RangedNode>` by estimating their range.
pub fn resolve_tokens(nodes: &Vec<Node>, tokens: &Vec<ScanToken>, text: &str) -> Vec<RangedNode> {
    let mut ranged_nodes: Vec<RangedNode> = Vec::new();

    // we get an estimated range by searching for tokens that match the node property values
    // and, if available, the `location` of the node itself
    nodes.iter().for_each(|n| {
        let nearest_parent_location = get_nearest_parent_location(&n, nodes);
        let furthest_child_location = get_furthest_child_location(&n, nodes);

        let mut child_tokens = Vec::new();

        let mut find_token = |property: String| {
            child_tokens.push(
                tokens
                    .iter()
                    .filter_map(|t| {
                        if get_token_text(
                            usize::try_from(t.start).unwrap(),
                            usize::try_from(t.end).unwrap(),
                            text,
                        ) != property
                        {
                            return None;
                        }

                        if furthest_child_location.is_some()
                            && furthest_child_location.unwrap() < t.start as i32
                        {
                            return None;
                        }

                        let distance = t.start - nearest_parent_location;
                        if distance > 0 {
                            Some((distance, t))
                        } else {
                            None
                        }
                    })
                    .min_by_key(|(d, _)| d.to_owned())
                    .map(|(_, t)| t)
                    .unwrap(),
            );
        };

        match &n.node {
            NodeEnum::RangeVar(n) => {
                find_token(n.relname.to_owned());
            }
            _ => {}
        };

        let from_locations: Vec<i32> = [
            get_location(&n.node),
            Some(nearest_parent_location),
            Some(child_tokens.iter().min_by_key(|t| t.start).unwrap().start),
        ]
        .into_iter()
        .filter_map(|x| x)
        .collect();

        ranged_nodes.push(RangedNode {
            inner: n.to_owned(),
            estimated_range: TextRange::new(
                TextSize::from(from_locations.iter().min().unwrap_or(&0).to_owned() as u32),
                TextSize::from(child_tokens.iter().max_by_key(|t| t.end).unwrap().end as u32),
            ),
        });
    });

    // FIXME: this additional loop is not required if we order the nodes by path first
    ranged_nodes
        .iter()
        .map(|n| RangedNode {
            inner: n.inner.to_owned(),
            // the range of a node must be larger than the range of all children nodes
            estimated_range: get_largest_child_range(&n, &ranged_nodes),
        })
        .collect()
}

fn get_token_text(start: usize, end: usize, text: &str) -> String {
    text.chars()
        .skip(start)
        .take(end - start)
        .collect::<String>()
}

fn get_largest_child_range(node: &RangedNode, nodes: &Vec<RangedNode>) -> TextRange {
    let mut start: TextSize = node.estimated_range.start().to_owned();
    let mut end: TextSize = node.estimated_range.end().to_owned();

    nodes.iter().for_each(|n| {
        if !n.inner.path.starts_with(node.inner.path.as_str()) {
            return;
        }
        if start < n.estimated_range.start() {
            start = n.estimated_range.start();
        }
        if end > n.estimated_range.end() {
            end = n.estimated_range.end();
        }
    });

    TextRange::new(start, end)
}

fn get_furthest_child_location(c: &Node, children: &Vec<Node>) -> Option<i32> {
    children
        .iter()
        .filter_map(|n| {
            if !n.path.starts_with(c.path.as_str()) {
                return None;
            }
            get_location(&n.node)
        })
        .max()
}

fn get_nearest_parent_location(n: &Node, children: &Vec<Node>) -> i32 {
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
