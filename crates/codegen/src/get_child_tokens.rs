use pg_query_proto_parser::{FieldType, Node, ProtoParser};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_child_tokens_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new("./libpg_query/protobuf/pg_query.proto");

    let proto_file = parser.parse();

    let node_identifiers = node_identifiers(&proto_file.nodes);
    let node_handlers = node_handlers(&proto_file.nodes);

    quote! {
        use pg_query::{protobuf::ScanToken, protobuf::Token, NodeEnum};

        #[derive(Debug)]
        struct TokenProperty {
            value: Option<String>,
            token: Option<Token>,
        }

        impl From<i32> for TokenProperty {
            fn from(value: i32) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }

        impl From<u32> for TokenProperty {
            fn from(value: u32) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }


        impl From<i64> for TokenProperty {
            fn from(value: i64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }

        impl From<u64> for TokenProperty {
            fn from(value: u64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }

        impl From<f64> for TokenProperty {
            fn from(value: f64) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }

        impl From<bool> for TokenProperty {
            fn from(value: bool) -> TokenProperty {
                TokenProperty {
                    value: Some(value.to_string()),
                    token: None,
                }
            }
        }

        impl From<String> for TokenProperty {
            fn from(value: String) -> TokenProperty {
                assert!(value.len() > 0, "String property value has length 0");
                TokenProperty {
                    value: Some(value),
                    token: None,
                }
            }
        }


        impl From<&pg_query::protobuf::Integer> for TokenProperty {
            fn from(node: &pg_query::protobuf::Integer) -> TokenProperty {
                TokenProperty {
                        value: Some(node.ival.to_string()),
                        token: Some(Token::Iconst)
                    }
            }
        }

        impl From<&pg_query::protobuf::Boolean> for TokenProperty {
            fn from(node: &pg_query::protobuf::Boolean) -> TokenProperty {
                TokenProperty {
                        value: Some(node.boolval.to_string()),
                        token: match node.boolval {
                            true => Some(Token::TrueP),
                            false => Some(Token::FalseP),
                        }
                    }
            }
        }

        impl From<Token> for TokenProperty {
            fn from(token: Token) -> TokenProperty {
                TokenProperty {
                    value: None,
                    token: Some(token),
                }
            }
        }

        fn get_token_text(start: usize, end: usize, text: &str) -> String {
            text.chars()
                .skip(start)
                .take(end - start)
                .collect::<String>()
        }


        pub fn get_child_tokens<'tokens>(node: &NodeEnum, tokens: &'tokens Vec<ScanToken>, text: &str, nearest_parent_location: i32, furthest_child_location: Option<i32>) -> Vec<&'tokens ScanToken> {
            let mut child_tokens = Vec::new();

            let mut get_token = |property: TokenProperty| {
                let token = tokens
                    .iter()
                    .filter_map(|t| {
                        if property.token.is_some() {
                            // if a token is set, we can safely ignore all tokens that are not of the same type
                            if t.token() != property.token.unwrap() {
                                return None;
                            }
                        }
                        // make a string comparison of the text of the token and the property value
                        if property.value.is_some()
                            && get_token_text(
                                usize::try_from(t.start).unwrap(),
                                usize::try_from(t.end).unwrap(),
                                text,
                            )
                            .to_lowercase()
                                != property.value.as_ref().unwrap().to_lowercase()
                        {
                            return None;
                        }

                        // if the furthest child location is set, and it is smaller than the start of the token,
                        // we can safely ignore this token, because it is not a child of the node
                        if furthest_child_location.is_some()
                            && furthest_child_location.unwrap() < t.start as i32
                        {
                            return None;
                        }

                        // if the token is before the nearest parent location, we can safely ignore it
                        // if not, we calculate the distance to the nearest parent location
                        let distance = t.start - nearest_parent_location;
                        if distance >= 0 {
                            Some((distance, t))
                        } else {
                            None
                        }
                    })
                    // and use the token with the smallest distance to the nearest parent location
                    .min_by_key(|(d, _)| d.to_owned())
                    .map(|(_, t)| t);

                if token.is_none() {
                    panic!(
                        "No matching token found for property {:?} in {:#?}",
                        property, tokens
                    );
                }

                child_tokens.push(token.unwrap());
            };

            match node {
                #(NodeEnum::#node_identifiers(n) => {#node_handlers}),*,
            };

            child_tokens
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
            get_token(TokenProperty::from(Token::Select));
        },
        "Integer" => quote! {
            get_token(TokenProperty::from(n));
        },
        "Boolean" => quote! {
            get_token(TokenProperty::from(n));
        },
        "AConst" => quote! {
            if n.isnull {
                get_token(TokenProperty::from(Token::NullP));
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
                        get_token(TokenProperty::from(n.#field_name.to_owned()));
                    }
                }),
                _ => None,
            }
        })
        .collect()
}
