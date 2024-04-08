mod get_location;
mod get_node_properties;
mod get_nodes;
mod syntax_kind;

use get_location::get_location_mod;
use get_node_properties::get_node_properties_mod;
use get_nodes::get_nodes_mod;
use pg_query_proto_parser::ProtoParser;
use quote::quote;
use std::{env, path, path::Path};

#[proc_macro]
pub fn parser_codegen(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let syntax_kind = syntax_kind::syntax_kind_mod(&proto_file);
    let get_location = get_location_mod(&proto_file);
    let get_node_properties = get_node_properties_mod(&proto_file);
    let get_nodes = get_nodes_mod(&proto_file);

    quote! {
        use std::collections::VecDeque;
        use pg_query::{protobuf, protobuf::ScanToken, protobuf::Token, NodeEnum, NodeRef};
        use cstree::text::{TextRange, TextSize};
        use cstree::Syntax;
        use std::cmp::{min, Ordering};
        use std::fmt::{Display, Formatter};
        use petgraph::stable_graph::{StableGraph};
        use petgraph::graph::{NodeIndex};

        #syntax_kind
        #get_location
        #get_node_properties
        #get_nodes
    }
    .into()
}

fn proto_file_path() -> path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("libpg_query/protobuf/pg_query.proto")
        .to_path_buf()
}
