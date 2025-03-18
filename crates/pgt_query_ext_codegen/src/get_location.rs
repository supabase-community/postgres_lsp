use pgt_query_proto_parser::{FieldType, Node, ProtoFile};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_location_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let manual_node_names = manual_node_names();

    let node_identifiers = node_identifiers(&proto_file.nodes, &manual_node_names);
    let location_idents = location_idents(&proto_file.nodes, &manual_node_names);

    quote! {
        /// Returns the location of a node
        pub fn get_location(node: &NodeEnum) -> Option<usize> {
            let loc = get_location_internal(node);
            if loc.is_some() {
                usize::try_from(loc.unwrap()).ok()
            } else {
                None
            }
        }

        fn get_location_internal(node: &NodeEnum) -> Option<i32> {
            let location = match node {
                // for some nodes, the location of the node itself is after their children location.
                // we implement the logic for those nodes manually.
                // if you add one, make sure to add its name to `manual_node_names()`.
                NodeEnum::BoolExpr(n) => {
                    let a = n.args.iter().min_by(|a, b| {
                        let loc_a = get_location_internal(&a.node.as_ref().unwrap());
                        let loc_b = get_location_internal(&b.node.as_ref().unwrap());
                        loc_a.cmp(&loc_b)
                    });
                    get_location_internal(&a.unwrap().node.as_ref().unwrap())
                },
                NodeEnum::AExpr(n) => get_location_internal(&n.lexpr.as_ref().unwrap().node.as_ref().unwrap()),
                NodeEnum::WindowDef(n) => {
                    if n.partition_clause.len() > 0 || n.order_clause.len() > 0 {
                        // the location is not correct if its the definition clause, e.g. for
                        // window w as (partition by a order by b)
                        // the location is the start of the `partition` token
                        None
                    } else  {
                        Some(n.location)
                    }
                },
                NodeEnum::CollateClause(n) => get_location_internal(&n.arg.as_ref().unwrap().node.as_ref().unwrap()),
                NodeEnum::TypeCast(n) => get_location_internal(&n.arg.as_ref().unwrap().node.as_ref().unwrap()),
                NodeEnum::ColumnDef(n) => if n.colname.len() > 0 {
                    Some(n.location)
                } else {
                    None
                },
                NodeEnum::NullTest(n) => if n.arg.is_some()  {
                    get_location_internal(&n.arg.as_ref().unwrap().node.as_ref().unwrap())
                } else {
                    Some(n.location)
                },
                NodeEnum::PublicationObjSpec(n) => {
                    match &n.pubtable {
                        Some(pubtable) => match &pubtable.relation {
                            Some(range_var) => Some(range_var.location),
                            None => Some(n.location),
                        },
                        None => Some(n.location),
                    }
                },
                NodeEnum::BooleanTest(n) => {
                    if n.arg.is_some() {
                        get_location_internal(&n.arg.as_ref().unwrap().node.as_ref().unwrap())
                    } else {
                        Some(n.location)
                    }
                },
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
    vec![
        "BoolExpr",
        "AExpr",
        "WindowDef",
        "CollateClause",
        "TypeCast",
        "ColumnDef",
        "NullTest",
        "PublicationObjSpec",
    ]
}

fn location_idents(nodes: &[Node], exclude_nodes: &[&str]) -> Vec<TokenStream> {
    nodes
        .iter()
        .filter(|n| !exclude_nodes.contains(&n.name.as_str()))
        .map(|node| {
            if node
                .fields
                .iter()
                .any(|n| n.name == "location" && n.field_type == FieldType::Int32)
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
