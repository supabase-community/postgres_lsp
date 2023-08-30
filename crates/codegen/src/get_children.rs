// use pg_query_proto_parser::{FieldType, Node, ProtoParser};
// use proc_macro2::{Ident, TokenStream};
// use quote::{format_ident, quote};
//
// // todo: get_children should only return a Vec<NestedNode> with location being an Option<i32>
// // we then pass the results into a resolve_locations function that takes a Vec<NestedNode> and returns a Vec<NestedNode>, but where location is an i32
//
// pub fn get_children_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
//     let parser = ProtoParser::new("./libpg_query/protobuf/pg_query.proto");
//     let proto_file = parser.parse();
//
//     let manual_node_names = manual_node_names();
//
//     let node_identifiers = node_identifiers(&proto_file.nodes, &manual_node_names);
//     let node_handlers = node_handlers(&proto_file.nodes, &manual_node_names);
//
//     quote! {
//         use pg_query::NodeEnum;
//         use std::collections::VecDeque;
//
//         #[derive(Debug, Clone)]
//         pub struct ChildrenNode {
//             pub node: NodeEnum,
//             pub depth: i32,
//             pub location: Option<i32>,
//             pub path: String,
//         }
//
//         /// Returns all children of the node, recursively
//         pub fn get_children(node: &NodeEnum, text: String, current_depth: i32) -> Vec<ChildrenNode> {
//             let mut nodes: Vec<ChildrenNode> = vec![];
//             // Node, depth, path
//             let mut stack: VecDeque<(NodeEnum, i32, String)> =
//                 VecDeque::from(vec![(node.to_owned(), current_depth, "0".to_string())]);
//             while !stack.is_empty() {
//                 let (node, depth, path) = stack.pop_front().unwrap();
//                 let current_depth = depth + 1;
//                 let mut child_ctr: i32 = 0;
//                 let mut handle_child = |c: NodeEnum| {
//                     let location = get_location(&c);
//                     let path = path.clone() + "." + child_ctr.to_string().as_str();
//                     child_ctr = child_ctr + 1;
//                     stack.push_back((c.to_owned(), current_depth, path.clone()));
//                     nodes.push(ChildrenNode {
//                         node: c,
//                         depth: current_depth,
//                         location,
//                         path: path.clone(),
//                     });
//                 };
//                 match &node {
//                     // `AConst` is the only node with a `one of` property, so we handle it manually
//                     // if you need to handle other nodes manually, add them to the `manual_node_names` function below
//                     NodeEnum::AConst(n) => {
//                         if n.val.is_some() {
//                             handle_child(match n.val.to_owned().unwrap() {
//                                 pg_query::protobuf::a_const::Val::Ival(v) => NodeEnum::Integer(v),
//                                 pg_query::protobuf::a_const::Val::Fval(v) => NodeEnum::Float(v),
//                                 pg_query::protobuf::a_const::Val::Boolval(v) => NodeEnum::Boolean(v),
//                                 pg_query::protobuf::a_const::Val::Sval(v) => NodeEnum::String(v),
//                                 pg_query::protobuf::a_const::Val::Bsval(v) => NodeEnum::BitString(v),
//                             });
//                         }
//                     }
//                     #(NodeEnum::#node_identifiers(n) => {
//                         #node_handlers
//                     }),*,
//                 };
//             }
//             nodes
//         }
//     }
// }
//
// fn manual_node_names() -> Vec<&'static str> {
//     vec!["AConst"]
// }
//
// fn node_identifiers(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<Ident> {
//     nodes
//         .iter()
//         .filter(|node| !exclude_nodes.contains(&node.name.as_str()))
//         .map(|node| format_ident!("{}", &node.name))
//         .collect()
// }
//
// fn node_handlers(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<TokenStream> {
//     nodes
//         .iter()
//         .filter(|node| !exclude_nodes.contains(&node.name.as_str()))
//         .map(|node| {
//             let property_handlers = property_handlers(&node);
//             quote! {
//                 #(#property_handlers)*
//             }
//         })
//         .collect()
// }
//
// fn property_handlers(node: &Node) -> Vec<TokenStream> {
//     node.fields
//         .iter()
//         .map(|field| {
//             if field.field_type == FieldType::Node && field.repeated {
//                 let field_name = field.name.as_str();
//                 quote! {
//                     n.#field_name
//                         .iter()
//                         .for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));
//                 }
//             } else if field.field_type == FieldType::Node && field.is_one_of == false {
//                 if field.node_name == Some("Node".to_owned()) {
//                     let field_name = field.name.as_str();
//                     quote! {
//                         if n.#field_name.is_some() {
//                             handle_child(n.#field_name.to_owned().unwrap().node.unwrap());
//                         }
//                     }
//                 } else {
//                     let enum_variant_name = field.enum_variant_name.as_ref().unwrap();
//                     let field_name = field.name.as_str();
//                     quote! {
//                         if n.#field_name.is_some() {
//                             handle_child(NodeEnum::#enum_variant_name(n.#field_name.to_owned().unwrap()));
//                         }
//                     }
//                 }
//             } else {
//                 panic!("Unhandled field type: {:?}", field);
//             }
//         })
//         .collect()
// }
