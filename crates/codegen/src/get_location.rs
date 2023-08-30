use pg_query_proto_parser::{FieldType, Node, ProtoParser};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_location_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new("./libpg_query/protobuf/pg_query.proto");
    let proto_file = parser.parse();

    let manual_node_names = manual_node_names();

    let node_identifiers = node_identifiers(&proto_file.nodes, &manual_node_names);
    let location_idents = location_idents(&proto_file.nodes, &manual_node_names);

    quote! {
        use pg_query::NodeEnum;

        //! Returns the location of a node
        pub fn get_location(node: &NodeEnum) -> Option<i32> {
            let location = match node {
                // for some nodes, the location of the node itself is after their childrens location.
                // we implement the logic for those nodes manually.
                // if you add one, make sure to add its name to `manual_node_names()`.
                NodeEnum::BoolExpr(n) => {
                    let a = n.args.iter().min_by(|a, b| {
                        let loc_a = get_location(&a.node.as_ref().unwrap());
                        let loc_b = get_location(&b.node.as_ref().unwrap());
                        loc_a.cmp(&loc_b)
                    });
                    get_location(&a.unwrap().node.as_ref().unwrap())
                },
                NodeEnum::AExpr(n) => get_location(&n.lexpr.as_ref().unwrap().node.as_ref().unwrap()),
                #(NodeEnum::#node_identifiers(n) => #location_idents),*
            };
            if location.is_some() && location.unwrap() < 0 {
                None
            } else {
                location
            }
        }
    }
}

fn manual_node_names() -> Vec<&'static str> {
    vec!["BoolExpr", "AExpr"]
}

fn location_idents(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<TokenStream> {
    nodes
        .iter()
        .filter(|n| !exclude_nodes.contains(&n.name.as_str()))
        .map(|node| {
            if node
                .fields
                .iter()
                .find(|n| n.name == "location" && n.field_type == FieldType::Int32)
                .is_some()
            {
                quote! { Some(n.location) }
            } else {
                quote! { None }
            }
        })
        .collect()
}

fn node_identifiers(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<Ident> {
    nodes
        .iter()
        .filter(|n| !exclude_nodes.contains(&n.name.as_str()))
        .map(|node| format_ident!("{}", &node.name))
        .collect()
}
