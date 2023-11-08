use pg_query_proto_parser::ProtoParser;
use quote::quote;

use crate::{
    get_node_properties::get_node_properties_mod, get_nodes::get_nodes_mod,
    syntax_kind::syntax_kind_mod,
};

pub fn parser_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new("libpg_query/protobuf/pg_query.proto");
    let proto_file = parser.parse();

    let syntax_kind = syntax_kind_mod(&proto_file);
    let get_node_properties = get_node_properties_mod(&proto_file);
    let get_nodes = get_nodes_mod(&proto_file);

    quote! {
        use std::collections::VecDeque;
        use log::{debug};
        use pg_query::{protobuf::ScanToken, protobuf::Token, NodeEnum, protobuf::SortByDir, NodeRef};
        use cstree::text::{TextRange, TextSize};
        use cstree::Syntax;

        #syntax_kind
        #get_node_properties
        #get_nodes
    }
}
