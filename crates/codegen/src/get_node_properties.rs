use pg_query_proto_parser::{FieldType, Node, ProtoFile};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_node_properties_mod(proto_file: &ProtoFile) -> proc_macro2::TokenStream {
    let node_identifiers = node_identifiers(&proto_file.nodes);
    let node_handlers = node_handlers(&proto_file.nodes);

    quote! {
        #[derive(Debug, Clone)]
        pub struct TokenProperty {
            pub value: Option<String>,
            pub kind: Option<SyntaxKind>,
        }

        impl From<i32> for TokenProperty {
            fn from(value: i32) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }

        impl From<u32> for TokenProperty {
            fn from(value: u32) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }


        impl From<i64> for TokenProperty {
            fn from(value: i64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }

        impl From<u64> for TokenProperty {
            fn from(value: u64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }

        impl From<f64> for TokenProperty {
            fn from(value: f64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }

        impl From<bool> for TokenProperty {
            fn from(value: bool) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    kind: None,
                }
            }
        }

        impl From<String> for TokenProperty {
            fn from(value: String) -> TokenProperty {
                assert!(value.len() > 0, "String property value has length 0");
                TokenProperty {
                    value: Some(value.to_lowercase()),
                    kind: None,
                }
            }
        }


        impl From<&pg_query::protobuf::Integer> for TokenProperty {
            fn from(node: &pg_query::protobuf::Integer) -> TokenProperty {
                TokenProperty {
                        value: Some(node.ival.to_string()),
                        kind: Some(SyntaxKind::Iconst)
                    }
            }
        }

        impl From<&pg_query::protobuf::Boolean> for TokenProperty {
            fn from(node: &pg_query::protobuf::Boolean) -> TokenProperty {
                TokenProperty {
                        value: Some(node.boolval.to_string()),
                        kind: match node.boolval {
                            true => Some(SyntaxKind::TrueP),
                            false => Some(SyntaxKind::FalseP),
                        }
                    }
            }
        }

        impl From<Token> for TokenProperty {
            fn from(token: Token) -> TokenProperty {
                TokenProperty {
                    value: None,
                    kind: Some(SyntaxKind::from(token)),
                }
            }
        }

        pub fn get_node_properties(node: &NodeEnum) -> Vec<TokenProperty> {
            let mut tokens: Vec<TokenProperty> = Vec::new();

            match node {
                #(NodeEnum::#node_identifiers(n) => {#node_handlers}),*,
            };

            tokens
        }

    }
}

fn node_identifiers(nodes: &[Node]) -> Vec<Ident> {
    nodes
        .iter()
        .map(|node| format_ident!("{}", &node.name))
        .collect()
}

fn node_handlers(nodes: &[Node]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|node| {
            let string_property_handlers = string_property_handlers(&node);
            let custom_handlers = custom_handlers(&node);
            quote! {
                #custom_handlers
                #(#string_property_handlers)*
            }
        })
        .collect()
}

fn custom_handlers(node: &Node) -> TokenStream {
    match node.name.as_str() {
        "SelectStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Select));
            if n.distinct_clause.len() > 0 {
                tokens.push(TokenProperty::from(Token::Distinct));
            }
            if n.values_lists.len() > 0 {
                tokens.push(TokenProperty::from(Token::Values));
            }
            if n.from_clause.len() > 0 {
                tokens.push(TokenProperty::from(Token::From));
            }
            if n.where_clause.is_some() {
                tokens.push(TokenProperty::from(Token::Where));
            }
            if n.group_clause.len() > 0 {
                tokens.push(TokenProperty::from(Token::GroupP));
                tokens.push(TokenProperty::from(Token::By));
            }
        },
        "BoolExpr" => quote! {
            match n.boolop {
                1 => tokens.push(TokenProperty::from(Token::And)),
                2 => tokens.push(TokenProperty::from(Token::Or)),
                _ => panic!("Unknown BoolExpr {:#?}", n.boolop),
            }
        },
        "JoinExpr" => quote! {
            tokens.push(TokenProperty::from(Token::Join));
            tokens.push(TokenProperty::from(Token::On));
        },
        "ResTarget" => quote! {
            if n.name.len() > 0 {
                tokens.push(TokenProperty::from(Token::As));
            }
        },
        "Integer" => quote! {
            tokens.push(TokenProperty::from(n));
        },
        "WindowDef" => quote! {
            if n.partition_clause.len() > 0 {
                tokens.push(TokenProperty::from(Token::Window));
            } else {
                tokens.push(TokenProperty::from(Token::Over));
            }
        },
        "Boolean" => quote! {
            tokens.push(TokenProperty::from(n));
        },
        "AStar" => quote! {
            tokens.push(TokenProperty::from(Token::Ascii42));
        },
        "FuncCall" => quote! {
            if n.funcname.len() == 1 && n.args.len() == 0 {
                // check if count(*)
                if let Some(node) = &n.funcname[0].node {
                    if let NodeEnum::String(n) = node {
                        if n.sval == "count" {
                            tokens.push(TokenProperty::from(Token::Ascii42));
                        }
                    }
                }
            }
            if n.agg_filter.is_some() {
                tokens.push(TokenProperty::from(Token::Filter));
                tokens.push(TokenProperty::from(Token::Where));
            }
        },
        "SqlvalueFunction" => quote! {
            match n.op {
                // 1 SvfopCurrentDate
                // 2 SvfopCurrentTime
                // 3 SvfopCurrentTimeN
                // 4 SvfopCurrentTimestamp
                // 5 SvfopCurrentTimestampN
                // 6 SvfopLocaltime
                // 7 SvfopLocaltimeN
                // 8 SvfopLocaltimestamp
                // 9 SvfopLocaltimestampN
                // 10 SvfopCurrentRole
                10 => tokens.push(TokenProperty::from(Token::CurrentRole)),
                // 11 SvfopCurrentUser
                11 => tokens.push(TokenProperty::from(Token::CurrentUser)),
                // 12 SvfopUser
                // 13 SvfopSessionUser
                // 14 SvfopCurrentCatalog
                // 15 SvfopCurrentSchema
                _ => panic!("Unknown SqlvalueFunction {:#?}", n.op),
            }
        },
        "SortBy" => quote! {
            tokens.push(TokenProperty::from(Token::Order));
            tokens.push(TokenProperty::from(Token::By));
            match n.sortby_dir {
                2 => tokens.push(TokenProperty::from(Token::Asc)),
                3 => tokens.push(TokenProperty::from(Token::Desc)),
                _ => {}
            }
        },
        "AConst" => quote! {
            if n.isnull {
                tokens.push(TokenProperty::from(Token::NullP));
            }
        },
        "AlterTableStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Alter));
            tokens.push(TokenProperty::from(Token::Table));
        },
        "AlterTableCmd" => quote! {
            tokens.push(TokenProperty::from(Token::Alter));
            match n.subtype {
                4 => {
                    tokens.push(TokenProperty::from(Token::Column));
                    tokens.push(TokenProperty::from(Token::Set));
                    tokens.push(TokenProperty::from(Token::Default));
                },
                _ => panic!("Unknown AlterTableCmd {:#?}", n.subtype),
            }
        },
        "CopyStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Copy));
            tokens.push(TokenProperty::from(Token::From));
        },
        "RenameStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Alter));
            tokens.push(TokenProperty::from(Token::Table));
            tokens.push(TokenProperty::from(Token::Rename));
            tokens.push(TokenProperty::from(Token::To));
        },
        "Constraint" => quote! {
            match n.contype {
                10 => {
                    // ConstrForeign
                    tokens.push(TokenProperty::from(Token::References));
                },
                _ => panic!("Unknown Constraint {:#?}", n.contype),
            }
        },
        "InsertStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Insert));
            tokens.push(TokenProperty::from(Token::Into));
        },
        "DeleteStmt" => quote! {
            tokens.push(TokenProperty::from(Token::DeleteP));
            tokens.push(TokenProperty::from(Token::From));
            if n.where_clause.is_some() {
                tokens.push(TokenProperty::from(Token::Where));
            }
            if n.using_clause.len() > 0 {
                tokens.push(TokenProperty::from(Token::Using));
            }
        },
        "ViewStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Create));
            tokens.push(TokenProperty::from(Token::View));
            if n.query.is_some() {
                tokens.push(TokenProperty::from(Token::As));
            }
            if n.replace {
                tokens.push(TokenProperty::from(Token::Or));
                tokens.push(TokenProperty::from(Token::Replace));
            }
        },
        "CreateStmt" => quote! {
            tokens.push(TokenProperty::from(Token::Create));
            tokens.push(TokenProperty::from(Token::Table));
            if n.if_not_exists {
                tokens.push(TokenProperty::from(Token::IfP));
                tokens.push(TokenProperty::from(Token::Not));
                tokens.push(TokenProperty::from(Token::Exists));
            }
        },
        _ => quote! {},
    }
}

fn string_property_handlers(node: &Node) -> Vec<TokenStream> {
    node.fields
        .iter()
        .filter_map(|field| {
            if field.repeated {
                return None;
            }
            let field_name = format_ident!("{}", field.name.as_str());
            match field.field_type {
                // just handle string values for now
                FieldType::String => Some(quote! {
                    // most string values are never None, but an empty string
                    if n.#field_name.len() > 0 {
                        tokens.push(TokenProperty::from(n.#field_name.to_owned()));
                    }
                }),
                _ => None,
            }
        })
        .collect()
}
