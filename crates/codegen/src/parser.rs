use pg_query_proto_parser::ProtoParser;
use quote::quote;
use std::{env, path, path::Path};

use crate::{
    get_location::get_location_mod, get_node_properties::get_node_properties_mod,
    get_nodes::get_nodes_mod, syntax_kind::syntax_kind_mod,
};

pub fn parser_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let syntax_kind = syntax_kind_mod(&proto_file);
    let get_location = get_location_mod(&proto_file);
    let get_node_properties = get_node_properties_mod(&proto_file);
    let get_nodes = get_nodes_mod(&proto_file);

    quote! {
        use std::collections::VecDeque;
        use log::{debug};
        use pg_query::{
            NodeEnum,
            NodeRef,
            protobuf::ScanToken,
            protobuf::Token,
            protobuf::SortByDir,
            protobuf::BoolExprType,
            protobuf::JoinType,
            protobuf::DefElemAction,
            protobuf::AExprKind,
            protobuf::SqlValueFunctionOp,
            protobuf::AlterTableType,
            protobuf::VariableSetKind,
            protobuf::ConstrType,
            protobuf::NullTestType,
            protobuf::FunctionParameterMode,
            protobuf::CoercionContext,
        };
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
}

fn proto_file_path() -> path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("libpg_query/protobuf/pg_query.proto")
        .to_path_buf()
}
