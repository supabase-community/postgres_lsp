mod syntax_kind;

use pg_query_proto_parser::ProtoParser;
use quote::quote;
use std::{env, path, path::Path};

#[proc_macro]
pub fn lexer_codegen(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = ProtoParser::new(&proto_file_path());
    let proto_file = parser.parse();

    let syntax_kind = syntax_kind::syntax_kind_mod(&proto_file);

    quote! {
        use pg_query::{protobuf, protobuf::ScanToken, protobuf::Token, NodeEnum, NodeRef};
        use cstree::Syntax;

        #syntax_kind
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
