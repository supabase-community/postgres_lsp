mod get_location;
mod get_node_properties;
mod get_nodes;
mod node_iterator;

use get_location::get_location_mod;
use get_node_properties::get_node_properties_mod;
use get_nodes::get_nodes_mod;
use node_iterator::node_iterator_mod;
use pgt_query_proto_parser::ProtoParser;
use quote::quote;
use std::{env, path, path::Path};

#[proc_macro]
pub fn codegen(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let get_location = get_location_mod(&proto_file);
    let get_node_properties = get_node_properties_mod(&proto_file);
    let get_nodes = get_nodes_mod(&proto_file);
    let iterator = node_iterator_mod(&proto_file);

    quote! {
        use pgt_lexer::SyntaxKind;
        use std::collections::VecDeque;
        use pg_query::{protobuf, protobuf::ScanToken, protobuf::Token, NodeEnum, NodeRef};
        use std::cmp::{min, Ordering};
        use std::fmt::{Display, Formatter};
        use petgraph::stable_graph::{StableGraph};
        use petgraph::graph::{NodeIndex};

        #get_location
        #get_node_properties
        #get_nodes
        #iterator
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
