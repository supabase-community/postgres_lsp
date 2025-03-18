use std::collections::HashSet;

use pgt_query_proto_parser::{Node, ProtoFile, Token};
use proc_macro2::{Ident, Literal};
use quote::{format_ident, quote};

pub fn syntax_kind_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let custom_node_names = custom_node_names();
    let custom_node_identifiers = custom_node_identifiers(&custom_node_names);

    let node_identifiers = node_identifiers(&proto_file.nodes);

    let token_identifiers = token_identifiers(&proto_file.tokens);
    let token_value_literals = token_value_literals(&proto_file.tokens);

    let syntax_kind_from_impl =
        syntax_kind_from_impl(&node_identifiers, &token_identifiers, &token_value_literals);

    let mut enum_variants = HashSet::new();
    enum_variants.extend(&custom_node_identifiers);
    enum_variants.extend(&node_identifiers);
    enum_variants.extend(&token_identifiers);
    let unique_enum_variants = enum_variants.into_iter().collect::<Vec<_>>();

    quote! {
        /// An u32 enum of all valid syntax elements (nodes and tokens) of the postgres
        /// sql dialect, and a few custom ones that are not parsed by pg_query.rs, such
        /// as `Whitespace`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u32)]
        pub enum SyntaxKind {
            #(#unique_enum_variants),*,
        }

        #syntax_kind_from_impl
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
        "Eof",
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

fn syntax_kind_from_impl(
    node_identifiers: &[Ident],
    token_identifiers: &[Ident],
    token_value_literals: &[Literal],
) -> proc_macro2::TokenStream {
    quote! {
        /// Converts a `pg_query` node to a `SyntaxKind`
        impl From<&NodeEnum> for SyntaxKind {
            fn from(node: &NodeEnum) -> SyntaxKind {
                match node {
                    #(NodeEnum::#node_identifiers(_) => SyntaxKind::#node_identifiers),*
                }
            }

        }

        impl From<Token> for SyntaxKind {
            fn from(token: Token) -> SyntaxKind {
                match i32::from(token) {
                    #(#token_value_literals => SyntaxKind::#token_identifiers),*,
                    _ => panic!("Unknown token: {:?}", token),
                }
            }
        }

        impl From<&ScanToken> for SyntaxKind {
            fn from(token: &ScanToken) -> SyntaxKind {
                match token.token {
                    #(#token_value_literals => SyntaxKind::#token_identifiers),*,
                    _ => panic!("Unknown token: {:?}", token.token),
                }
            }
        }
    }
}
