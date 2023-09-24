use crate::get_children_codegen::ChildrenNode;
use crate::get_location_codegen::get_location;
use cstree::text::{TextRange, TextSize};
use pg_query::{protobuf::ScanToken, NodeEnum};

#[derive(Debug, Clone)]
pub struct NestedNode {
    pub id: usize,
    pub inner: ChildrenNode,
    // .start property of `ScanToken`
    pub tokens: Vec<i32>,
    pub range: TextRange,
}

/// Turns a `Vec<ChildrenNode>` into a `Vec<NestedNode>` by adding `tokens` and `range` to each node.
///
/// For each node, we walk all properties and search for tokens that match the property value. The
/// token that is closest to the node or a parent is used.
///
/// The node range is the minimum start and maximum end of all tokens.
pub fn resolve_tokens(
    children: &Vec<ChildrenNode>,
    tokens: &Vec<ScanToken>,
    text: &str,
) -> Vec<NestedNode> {
    children
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            let nearest_parent_location = get_nearest_parent_location(&c, children);
            let furthest_child_location = get_furthest_child_location(&c, children);

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

            match &c.node {
                NodeEnum::RangeVar(n) => {
                    find_token(n.relname.to_owned());
                }
                _ => {}
            };

            NestedNode {
                id: idx,
                inner: c.to_owned(),
                tokens: child_tokens.iter().map(|t| t.start).collect(),
                range: TextRange::new(
                    TextSize::from(
                        child_tokens.iter().min_by_key(|t| t.start).unwrap().start as u32,
                    ),
                    TextSize::from(child_tokens.iter().max_by_key(|t| t.end).unwrap().end as u32),
                ),
            }
        })
        .collect()
}

fn get_token_text(start: usize, end: usize, text: &str) -> String {
    text.chars()
        .skip(start)
        .take(end - start)
        .collect::<String>()
}

fn get_furthest_child_location(c: &ChildrenNode, children: &Vec<ChildrenNode>) -> Option<i32> {
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

fn get_nearest_parent_location(n: &ChildrenNode, children: &Vec<ChildrenNode>) -> i32 {
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
