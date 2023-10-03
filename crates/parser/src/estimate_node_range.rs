use std::cmp::{max, min};

use crate::get_location_codegen::get_location;
use crate::get_nodes_codegen::Node;
use cstree::text::{TextRange, TextSize};
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

    // ensure that all children of any given node are already processed before processing the node itself
    nodes.sort_by(|a, b| b.path.cmp(&a.path));

    // we get an estimated range by searching for tokens that match the node property values
    // and, if available, the `location` of the node itself
    nodes.iter().for_each(|n| {
        // first, get the estimated boundaries of the node based on the `location` property of a node
        let nearest_parent_location = get_nearest_parent_location(&n, nodes);
        let furthest_child_location = get_furthest_child_location(&n, nodes);

        let mut child_tokens = Vec::new();

        #[derive(Debug)]
        struct TokenProperty {
            value: Option<String>,
            token: Option<Token>,
        }

        impl TokenProperty {
            fn from_int(value: &i32) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }

            fn from_string(value: &String) -> TokenProperty {
                assert!(value.len() > 0, "String property value has length 0");
                TokenProperty {
                    value: Some(value.to_owned()),
                    token: None,
                }
            }

            fn from_token(token: Token) -> TokenProperty {
                TokenProperty {
                    value: None,
                    token: Some(token),
                }
            }
        }

        let mut get_token = |property: TokenProperty| {
            let token = tokens
                .iter()
                .filter_map(|t| {
                    if property.token.is_some() {
                        // if a token is set, we can safely ignore all tokens that are not of the same type
                        if t.token() != property.token.unwrap() {
                            return None;
                        }
                    }
                    // make a string comparison of the text of the token and the property value
                    if property.value.is_some()
                        && get_token_text(
                            usize::try_from(t.start).unwrap(),
                            usize::try_from(t.end).unwrap(),
                            text,
                        )
                        .to_lowercase()
                            != property.value.as_ref().unwrap().to_lowercase()
                    {
                        return None;
                    }

                    // if the furthest child location is set, and it is smaller than the start of the token,
                    // we can safely ignore this token, because it is not a child of the node
                    if furthest_child_location.is_some()
                        && furthest_child_location.unwrap() < t.start as i32
                    {
                        return None;
                    }

                    // if the token is before the nearest parent location, we can safely ignore it
                    // if not, we calculate the distance to the nearest parent location
                    let distance = t.start - nearest_parent_location;
                    if distance >= 0 {
                        Some((distance, t))
                    } else {
                        None
                    }
                })
                // and use the token with the smallest distance to the nearest parent location
                .min_by_key(|(d, _)| d.to_owned())
                .map(|(_, t)| t);

            if token.is_none() {
                panic!(
                    "No matching token found for property {:?} in {:#?}",
                    property, tokens
                );
            }

            child_tokens.push(token.unwrap());
        };

        match &n.node {
            NodeEnum::RangeVar(n) => {
                get_token(TokenProperty::from_string(&n.relname));
            }
            NodeEnum::Integer(n) => {
                get_token(TokenProperty::from_int(&n.ival));
            }
            NodeEnum::AConst(n) => {
                if n.isnull {
                    get_token(TokenProperty::from_token(Token::NullP));
                }
            }
            NodeEnum::ResTarget(n) => {
                if n.name.len() > 0 {
                    get_token(TokenProperty::from_string(&n.name));
                }
            }
            NodeEnum::SelectStmt(_) => {
                get_token(TokenProperty::from_token(Token::Select));
            }
            _ => panic!("Node type not implemented: {:?}", n.node),
        };

        // For `from`, the location of the node itself is always correct.
        // If not available, the closest estimation is the smaller value of the start of the first direct child token,
        // and the start of all children ranges. If neither is available, let’s panic for now.
        // The parent location as a fallback should never be required, because any node must have either children with tokens, or a token itself.
        let location = get_location(&n.node);
        let from = if location.is_some() {
            location.unwrap()
        } else {
            let start_of_first_child_token = if child_tokens.len() > 0 {
                Some(child_tokens.iter().min_by_key(|t| t.start).unwrap().start)
            } else {
                None
            };
            let start_of_all_children_ranges = if ranged_nodes.len() > 0 {
                Some(
                    ranged_nodes
                        .iter()
                        .filter(|x| x.inner.path.starts_with(n.path.as_str()))
                        .min_by_key(|n| n.range.start())
                        .unwrap()
                        .range
                        .start(),
                )
            } else {
                None
            };

            if start_of_first_child_token.is_some() {
                if start_of_all_children_ranges.is_some() {
                    min(
                        start_of_first_child_token.unwrap(),
                        u32::from(start_of_all_children_ranges.unwrap()) as i32,
                    )
                } else {
                    start_of_first_child_token.unwrap()
                }
            } else if start_of_all_children_ranges.is_some() {
                u32::from(start_of_all_children_ranges.unwrap()) as i32
            } else {
                panic!("No location or child tokens found for node {:?}", n);
            }
        };

        // For `to`, it’s the larger value of the end of the last direkt child token, and the end of all children ranges.
        let end_of_last_child_token = if child_tokens.len() > 0 {
            Some(child_tokens.iter().max_by_key(|t| t.end).unwrap().end)
        } else {
            None
        };
        let end_of_all_children_ranges = if ranged_nodes.len() > 0 {
            Some(
                ranged_nodes
                    .iter()
                    .filter(|x| x.inner.path.starts_with(n.path.as_str()))
                    .max_by_key(|n| n.range.end())
                    .unwrap()
                    .range
                    .end(),
            )
        } else {
            None
        };
        let to = if end_of_last_child_token.is_some() {
            if end_of_all_children_ranges.is_some() {
                max(
                    end_of_last_child_token.unwrap(),
                    u32::from(end_of_all_children_ranges.unwrap()) as i32,
                )
            } else {
                end_of_last_child_token.unwrap()
            }
        } else if end_of_all_children_ranges.is_some() {
            u32::from(end_of_all_children_ranges.unwrap()) as i32
        } else {
            panic!("No child tokens or children ranges found for node {:?}", n);
        };

        ranged_nodes.push(RangedNode {
            inner: n.to_owned(),
            range: TextRange::new(TextSize::from(from as u32), TextSize::from(to as u32)),
        });
    });

    ranged_nodes
}

fn get_token_text(start: usize, end: usize, text: &str) -> String {
    text.chars()
        .skip(start)
        .take(end - start)
        .collect::<String>()
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
