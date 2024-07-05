use pg_query_proto_parser::{FieldType, Node, ProtoFile};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn node_iterator_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let manual_node_names = manual_node_names();

    let node_identifiers = node_identifiers(&proto_file.nodes, &manual_node_names);
    let node_handlers = node_handlers(&proto_file.nodes, &manual_node_names);

    quote! {
        #[derive(Debug, Clone)]
        pub struct ChildrenIterator {
            stack: VecDeque<(NodeEnum, usize)>,
            nodes: Vec<NodeEnum>,
        }

        impl ChildrenIterator {
            pub fn new(root: NodeEnum) -> Self {
                Self {
                    stack: VecDeque::from(vec![(root, 0)]),
                    nodes: Vec::new(),
                }
            }
        }

        impl Iterator for ChildrenIterator {
            type Item = NodeEnum;

            fn next(&mut self) -> Option<Self::Item> {
                if self.stack.is_empty() {
                    return None;
                }

                let (node, depth) = self.stack.pop_front().unwrap();

                let current_depth = depth + 1;

                match &node {
                    // `AConst` is the only node with a `one of` property, so we handle it manually
                    // if you need to handle other nodes manually, add them to the `manual_node_names` function below
                    NodeEnum::AConst(n) => {
                        // if n.val.is_some() {
                        //     let new_node = match n.val.as_ref().unwrap() {
                        //         pg_query::protobuf::a_const::Val::Ival(v) => Box::new(NodeEnum::Integer(v.clone())),
                        //         pg_query::protobuf::a_const::Val::Fval(v) => Box::new(NodeEnum::Float(v.clone())),
                        //         pg_query::protobuf::a_const::Val::Boolval(v) => Box::new(NodeEnum::Boolean(v.clone())),
                        //         pg_query::protobuf::a_const::Val::Sval(v) => Box::new(NodeEnum::String(v.clone())),
                        //         pg_query::protobuf::a_const::Val::Bsval(v) => Box::new(NodeEnum::BitString(v.clone())),
                        //     };
                        //     self.stack.push_back((&new_node, current_depth));
                        //     self.boxed_nodes.push(new_node);
                        // }
                    }
                    #(NodeEnum::#node_identifiers(n) => {#node_handlers}),*,
                };

                Some(node)
            }
        }
    }
}

fn manual_node_names() -> Vec<&'static str> {
    vec!["AConst"]
}

fn node_identifiers(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<Ident> {
    nodes
        .iter()
        .filter(|node| !exclude_nodes.contains(&node.name.as_str()))
        .map(|node| format_ident!("{}", &node.name))
        .collect()
}

fn node_handlers(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<TokenStream> {
    nodes
        .iter()
        .filter(|node| !exclude_nodes.contains(&node.name.as_str()))
        .map(|node| {
            let property_handlers = property_handlers(&node);
            quote! {
                #(#property_handlers)*
            }
        })
        .collect()
}

fn property_handlers(node: &Node) -> Vec<TokenStream> {
    node.fields
        .iter()
        .filter_map(|field| {
            let field_name = format_ident!("{}", field.name.as_str());
            if field.field_type == FieldType::Node && field.repeated {
                Some(quote! {
                    n.#field_name
                        .iter()
                        .for_each(|x| if x.node.is_some() {
                            self.stack.push_back((x.node.as_ref().unwrap().to_owned(), current_depth));
                        });
                })
            } else if field.field_type == FieldType::Node && field.is_one_of == false {
                if field.node_name == Some("Node".to_owned()) {
                    Some(quote! {
                        if n.#field_name.is_some() {
                            self.stack.push_back((n.#field_name.to_owned().unwrap().node.unwrap(), current_depth));
                        }
                    })
                } else {
                    let enum_variant_name =
                        format_ident!("{}", field.enum_variant_name.as_ref().unwrap().as_str());
                    Some(quote! {
                        if n.#field_name.is_some() {
                            self.stack.push_back((NodeEnum::#enum_variant_name(n.#field_name.to_owned().unwrap()), current_depth));
                        }
                    })
                }
            } else {
                None
            }
        })
        .collect()
}
