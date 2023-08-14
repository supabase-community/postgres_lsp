use std::collections::VecDeque;

use pg_query::{Node, NodeEnum};

use crate::{pg_query_utils_generated::get_location, pg_query_utils_manual::derive_location};

#[derive(Debug, Clone)]
struct NestedNode {
    node: NodeEnum,
    depth: i32,
    location: i32,
    path: String,
}

// some nodes that do not have a loc property, but have the same loc as their only child, eg AConst
// some nodes do not have a loc property, but have the same loc as their parent, e.g. AStar

// Problem: we need to get the location of a node, but many do not have a location property.
// 1. get children with location Option<i32>
// 2. if get_location returns None
//   a. call get children for node
//   b. apply regexp on input after get location of parent
//   c. if just one match, return its location. if more than one, return the one that is the first before the earliest child location.
//
// start with parent location 0. for this to work, we have to resolve the location of the parent
// first.
//
// if select with list.
// - select has parent, so i can get its location via regexp and parent location. --> wrong: we
// dont have the children locations at this point
// - then for children, i have parent, so both location and parent location are never None.
//
// add location_stack:
// - if no location prop, add to location stack with an id / path. the stack must be lifo.
// - nodes in stack have parent id / path. the id could be something like "1.2", where 1 is the parent of parent id,
// and 2 is the direct parent id.
// - when nodes are resolved, work on location stack before starting with nodes again
// - get location of parent node
// - get childs from nodes list (if i am id 1.2, get all nodes that start with 1.2) and get earliest location from them

pub fn get_children_test(node: &NodeEnum, text: String, current_depth: i32) -> Vec<NestedNode> {
    let mut nodes: Vec<NestedNode> = vec![];

    // node, depth, parent_location, path
    let mut stack: VecDeque<(NodeEnum, i32, String)> =
        VecDeque::from(vec![(node.to_owned(), current_depth, "0".to_string())]);

    // node, depth, path
    let mut location_stack: VecDeque<(NodeEnum, i32, String)> = VecDeque::new();

    while !stack.is_empty() || !location_stack.is_empty() {
        if !stack.is_empty() {
            let (node, depth, path) = stack.pop_front().unwrap();
            let current_depth = depth + 1;
            let mut child_ctr: i32 = 0;

            let mut handle_child = |c: NodeEnum| {
                let location = get_location(&c);
                let path = path.clone() + "." + child_ctr.to_string().as_str();
                child_ctr = child_ctr + 1;
                stack.push_back((c.to_owned(), current_depth, path.clone()));
                if location.is_some() {
                    nodes.push(NestedNode {
                        node: c,
                        depth: current_depth,
                        location: location.unwrap(),
                        path: path.clone(),
                    });
                } else {
                    location_stack.push_back((c, current_depth, path));
                }
            };

            match &node {
                NodeEnum::Alias(n) => {
                    n.colnames
                        .iter()
                        .for_each(|n| handle_child(n.node.as_ref().unwrap().to_owned()));
                }
                NodeEnum::RangeVar(n) => {
                    if n.alias.is_some() {
                        handle_child(NodeEnum::Alias(n.alias.to_owned().unwrap()));
                    }
                }
                n => {
                    println!("{:?}", n);
                    todo!();
                }
            }
        } else if !location_stack.is_empty() {
            // then, start with the beginning. we now always have a location for a parent, and SHOULD HAVE at least one child location. to get childs, use "starts_with(my_path)".
            // if no child, pass earliest_child_location = None
            let (node, depth, path) = location_stack.pop_front().unwrap();
            let parent_location = nodes
                .iter()
                .find(|n| {
                    let mut path_elements = path.split(".").collect::<Vec<&str>>();
                    path_elements.pop();
                    let parent_path = path_elements.join(".");
                    n.path == parent_path
                })
                .unwrap()
                .location;
            // get earliest child
            let earliest_child_location = nodes
                .iter()
                .filter(|n| n.path.starts_with(path.as_str()))
                .min_by(|a, b| a.location.cmp(&b.location))
                .map(|n| n.location);
            let location = derive_location(
                &node,
                text.clone(),
                parent_location,
                earliest_child_location,
            );
            nodes.push(NestedNode {
                node,
                depth,
                location,
                path: path.clone(),
            });
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use std::fs;
    use std::path::Path;

    use crate::pg_query_utils_generated::get_children;
    // use crate::pg_query_utils_generated_test::get_children_test;

    const VALID_STATEMENTS_PATH: &str = "test_data/statements/valid/";

    #[test]
    fn test_get_children() {
        let input = "with c as (insert into contact (id) values ('id') select * from c;";

        let pg_query_root = match pg_query::parse(input) {
            Ok(parsed) => {
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .for_each(|n| println!("{:?}", n));
                Some(
                    parsed
                        .protobuf
                        .nodes()
                        .iter()
                        .find(|n| n.1 == 1)
                        .unwrap()
                        .0
                        .to_enum(),
                )
            }
            Err(_) => None,
        };

        println!("{:?}", pg_query_root);

        let result = get_children(&pg_query_root.unwrap(), 1);
        println!("NUMBER OF CHILDREN: {:?}", result.len());
        result.iter().for_each(|n| {
            println!("##");
            println!("{:?}", n)
        });
    }
}
