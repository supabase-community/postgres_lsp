use pg_query_proto_parser::{FieldType, Node, ProtoFile};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn build_node_graph_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let manual_node_names = manual_node_names();

    let node_identifiers = node_identifiers(&proto_file.nodes, &manual_node_names);
    let node_handlers = node_handlers(&proto_file.nodes, &manual_node_names);

    quote! {
        #[derive(Debug, Clone)]
        pub struct NodeRef<'a> {
            pub node: &'a NodeEnum,
            pub depth: usize,
        }

        /// Returns all children of the node, recursively
        /// location is resolved manually
        pub fn build_node_graph<'a>(g: StableGraph<NodeRef<'a>, ()>, root: &'a NodeEnum) -> StableGraph<NodeRef<'a>, ()> {
            let mut g = StableGraph::<Node, ()>::new();

            let root_node_idx = g.add_node(Node {
                kind: SyntaxKind::from(node),
                depth: 0,
                properties: get_node_properties(node, None),
                location: get_location(node),
            });

            // Parent node idx, Node, depth
            let mut stack: VecDeque<(NodeIndex, NodeEnum, usize)> =
                VecDeque::from(vec![(root_node_idx, node.to_owned(), 0)]);
            while !stack.is_empty() {
                let (parent_idx, node, depth) = stack.pop_front().unwrap();
                let current_depth = depth + 1;
                let mut handle_child = |c: NodeEnum| {
                    if match &c {
                        // all "simple nodes" are not handled individually but merged with their parent
                        NodeEnum::String(n) => true,
                        NodeEnum::Integer(n) => true,
                        NodeEnum::Float(n) => true,
                        NodeEnum::Boolean(n) => true,
                        NodeEnum::BitString(n) => true,
                        _ => false
                    } {
                        g[parent_idx].properties.extend(get_node_properties(&c, Some(&node)));
                    } else {
                        let node_idx = g.add_node(Node {
                            kind: SyntaxKind::from(&c),
                            depth: current_depth,
                            properties: get_node_properties(&c, Some(&node)),
                            location: get_location(&c),
                        });
                        g.add_edge(parent_idx, node_idx, ());
                        stack.push_back((node_idx, c.to_owned(), current_depth));
                    }
                };
                match &node {
                    // `AConst` is the only node with a `one of` property, so we handle it manually
                    // if you need to handle other nodes manually, add them to the `manual_node_names` function below
                    NodeEnum::AConst(n) => {
                        if n.val.is_some() {
                            handle_child(match n.val.to_owned().unwrap() {
                                pg_query::protobuf::a_const::Val::Ival(v) => NodeEnum::Integer(v),
                                pg_query::protobuf::a_const::Val::Fval(v) => NodeEnum::Float(v),
                                pg_query::protobuf::a_const::Val::Boolval(v) => NodeEnum::Boolean(v),
                                pg_query::protobuf::a_const::Val::Sval(v) => NodeEnum::String(v),
                                pg_query::protobuf::a_const::Val::Bsval(v) => NodeEnum::BitString(v),
                            });
                        }
                    }
                    #(NodeEnum::#node_identifiers(n) => {#node_handlers}),*,
                };
            }
            g
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
                            handle_child(x.node.as_ref().unwrap().to_owned());
                        });
                })
            } else if field.field_type == FieldType::Node && field.is_one_of == false {
                if field.node_name == Some("Node".to_owned()) {
                    Some(quote! {
                        if n.#field_name.is_some() {
                            handle_child(n.#field_name.to_owned().unwrap().node.unwrap());
                        }
                    })
                } else {
                    let enum_variant_name =
                        format_ident!("{}", field.enum_variant_name.as_ref().unwrap().as_str());
                    Some(quote! {
                        if n.#field_name.is_some() {
                            handle_child(NodeEnum::#enum_variant_name(n.#field_name.to_owned().unwrap()));
                        }
                    })
                }
            } else {
                None
            }
        })
        .collect()
}
