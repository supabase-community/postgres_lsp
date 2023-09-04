use std::collections::HashSet;

use pg_query_proto_parser::{Node, ProtoParser, Token};
use proc_macro2::{Ident, Literal};
use quote::{format_ident, quote};

pub fn syntax_kind_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new("./libpg_query/protobuf/pg_query.proto");
    let proto_file = parser.parse();

    let custom_node_names = custom_node_names();
    let custom_node_identifiers = custom_node_identifiers(&custom_node_names);

    let node_identifiers = node_identifiers(&proto_file.nodes);

    let token_identifiers = token_identifiers(&proto_file.tokens);
    let token_value_literals = token_value_literals(&proto_file.tokens);

    let syntax_kind_impl =
        syntax_kind_impl(&node_identifiers, &token_identifiers, &token_value_literals);

    let mut enum_variants = HashSet::new();
    enum_variants.extend(&custom_node_identifiers);
    enum_variants.extend(&node_identifiers);
    enum_variants.extend(&token_identifiers);
    let unique_enum_variants = enum_variants.into_iter().collect::<Vec<_>>();

    quote! {
        use cstree::Syntax;
        use pg_query::{protobuf::ScanToken, NodeEnum, NodeRef};

        /// An u32 enum of all valid syntax elements (nodes and tokens) of the postgres
        /// sql dialect, and a few custom ones that are not parsed by pg_query.rs, such
        /// as `Whitespace`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Syntax)]
        #[repr(u32)]
        pub enum SyntaxKind {
            #(#unique_enum_variants),*,
        }

        #syntax_kind_impl
    }
}

fn custom_node_names() -> Vec<&'static str> {
    vec![
        "SourceFile",
        "Comment",
        "Whitespace",
        "Newline",
        "Tab",
        "Stmt",
    ]
}

fn custom_node_identifiers(custom_node_names: &[&str]) -> Vec<Ident> {
    custom_node_names
        .iter()
        .map(|&node_name| format_ident!("{}", node_name))
        .collect()
}

fn node_identifiers(nodes: &[Node]) -> Vec<Ident> {
    nodes
        .iter()
        .map(|node| format_ident!("{}", &node.name))
        .collect()
}

fn token_identifiers(tokens: &[Token]) -> Vec<Ident> {
    tokens
        .iter()
        .map(|token| format_ident!("{}", &token.name))
        .collect()
}

fn token_value_literals(tokens: &[Token]) -> Vec<Literal> {
    tokens
        .iter()
        .map(|token| Literal::i32_unsuffixed(token.value))
        .collect()
}

fn syntax_kind_impl(
    node_identifiers: &[Ident],
    token_identifiers: &[Ident],
    token_value_literals: &[Literal],
) -> proc_macro2::TokenStream {
    let new_from_pg_query_node_fn = new_from_pg_query_node_fn(node_identifiers);
    let new_from_pg_query_token_fn =
        new_from_pg_query_token_fn(token_identifiers, token_value_literals);
    quote! {
        impl SyntaxKind {
            #new_from_pg_query_node_fn

            #new_from_pg_query_token_fn
        }
    }
}

fn new_from_pg_query_node_fn(node_identifiers: &[Ident]) -> proc_macro2::TokenStream {
    quote! {
        /// Converts a `pg_query` node to a `SyntaxKind`
        pub fn new_from_pg_query_node(node: &NodeEnum) -> Self {
            match node {
                #(NodeEnum::#node_identifiers(_) => SyntaxKind::#node_identifiers),*
            }
        }
    }
}

fn new_from_pg_query_token_fn(
    token_identifiers: &[Ident],
    token_value_literals: &[Literal],
) -> proc_macro2::TokenStream {
    quote! {
        /// Converts a `pg_query` token to a `SyntaxKind`
        pub fn new_from_pg_query_token(token: &ScanToken) -> Self {
            match token.token {
                #(#token_value_literals => SyntaxKind::#token_identifiers),*,
                _ => panic!("Unknown token: {:?}", token.token),
            }
        }
    }
}
