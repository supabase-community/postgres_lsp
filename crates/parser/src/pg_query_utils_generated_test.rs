use pg_query::NodeEnum;

use crate::pg_query_utils_generated::NestedNode;

pub trait RemoveFirst<T> {
    fn remove_first(&mut self) -> Option<T>;
}

impl<T> RemoveFirst<T> for Vec<T> {
    fn remove_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.remove(0))
    }
}

// pub fn get_children(node: &NodeEnum, current_depth: i32) -> Vec<NestedNode> {
//     let mut nodes: Vec<NestedNode> = vec![];
//
//     // Node, depth, location
//     let mut stack: Vec<(NodeEnum, i32)> = vec![(node.to_owned(), current_depth)];
//
//     while stack.len() > 0 {
//         let (node, depth) = stack.remove_first().unwrap();
//
//         let current_depth = depth + 1;
//
//         match &node {
//             NodeEnum::Alias(n) => {
//                 n.colnames.iter().for_each(|x| {
//                     stack.push((x.node.to_owned().unwrap(), current_depth));
//                     nodes.push(NestedNode {
//                         node: x.node.to_owned().unwrap(),
//                         depth: current_depth,
//                     });
//                 });
//             }
//             NodeEnum::RangeVar(n) => {
//                 if n.alias.is_some() {
//                     let alias = NodeEnum::Alias(n.alias.to_owned().unwrap());
//                     stack.push((alias.to_owned(), current_depth));
//                     nodes.push(NestedNode {
//                         node: alias,
//                         depth: current_depth,
//                     });
//                 }
//             }
//             NodeEnum::Param(n) => {
//                 if n.xpr.is_some() {
//                     let xpr = n.xpr.to_owned().unwrap().node.unwrap();
//                     stack.push((xpr.to_owned(), current_depth));
//                     nodes.push(NestedNode {
//                         node: xpr,
//                         depth: current_depth,
//                     });
//                 }
//             }
//             _ => panic!("Not implemented"),
//         };
//     }
//
//     nodes
// }

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use std::fs;
    use std::path::Path;

    use crate::pg_query_utils_generated::get_children;

    const VALID_STATEMENTS_PATH: &str = "test_data/statements/valid/";

    #[test]
    fn test_get_children() {
        let input =
            "with t as (insert into contact (id) values ('id') returning *) select id from t;";

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
