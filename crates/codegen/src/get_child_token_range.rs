use pg_query_proto_parser::{FieldType, Node, ProtoParser};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_child_token_range_mod(_item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = ProtoParser::new("./libpg_query/protobuf/pg_query.proto");

    let proto_file = parser.parse();

    let node_identifiers = node_identifiers(&proto_file.nodes);
    let node_handlers = node_handlers(&proto_file.nodes);

    quote! {
        use log::{debug};
        use pg_query::{protobuf::ScanToken, protobuf::Token, NodeEnum, protobuf::SortByDir};
        use cstree::text::{TextRange, TextSize};

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
                    value: Some(value.to_lowercase()),
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

        fn get_token_text(token: &ScanToken ,text: &str) -> String {
            let start = usize::try_from(token.start).unwrap();
            let end = usize::try_from(token.end).unwrap();
            text.chars()
                .skip(start)
                .take(end - start)
                .collect::<String>()
                .to_lowercase()
        }


        /// list of aliases from https://www.postgresql.org/docs/current/datatype.html
        const ALIASES: [&[&str]; 2]= [
            &["integer", "int", "int4"],
            &["real", "float4"],
        ];

        /// returns a list of aliases for a string. primarily used for data types.
        fn aliases(text: &str) -> Vec<&str> {
            for alias in ALIASES {
                if alias.contains(&text) {
                    return alias.to_vec();
                }
            }
            return vec![text];
        }

        #[derive(Debug)]
        pub enum ChildTokenRangeResult {
            TooManyTokens,
            NoTokens,
            /// indices are the .start of all child tokens used to estimate the range
            ChildTokenRange { used_token_indices: Vec<i32>, range: TextRange },
        }

        pub fn get_child_token_range(node: &NodeEnum, tokens: Vec<&ScanToken>, text: &str, nearest_parent_location: Option<u32>) -> ChildTokenRangeResult {
            let mut child_tokens: Vec<&ScanToken> = Vec::new();

            // if true, we found more than one valid token for at least one property of the node
            let mut has_too_many_tokens: bool = false;

            let mut get_token = |property: TokenProperty| {
                let possible_tokens = tokens
                    .iter()
                    .filter_map(|t| {
                        if property.token.is_some() {
                            // if a token is set, we can safely ignore all tokens that are not of the same type
                            if t.token() != property.token.unwrap() {
                                return None;
                            }
                        }

                        // make a string comparison of the text of the token and the property value
                        if property.value.is_some() {
                            let mut token_text = get_token_text(t, text);
                            // if token is Sconst, remove leading and trailing quotes
                            if t.token() == Token::Sconst {
                                let string_delimiter: &[char; 2] = &['\'', '$'];
                                token_text = token_text.trim_start_matches(string_delimiter).trim_end_matches(string_delimiter).to_string();
                            }

                            if !aliases(property.value.as_ref().unwrap()).contains(&token_text.as_str()) {
                                return None;
                            }
                        }

                        Some(t)
                    })
                    .collect::<Vec<&&ScanToken>>();

                if possible_tokens.len() == 0 {
                    debug!(
                        "No matching token found for property {:#?} of node {:#?} in {:#?} with tokens {:#?}",
                        property, node, text, tokens
                    );
                    return;
                }

                if possible_tokens.len() == 1 {
                    debug!(
                        "Found token {:#?} for property {:#?} of node {:#?}",
                        possible_tokens[0], property, node
                    );
                    child_tokens.push(possible_tokens[0]);
                    return;
                }

                if nearest_parent_location.is_none() {
                    debug!("Found {:#?} for property {:#?} and no nearest_parent_location set", possible_tokens, property);
                    has_too_many_tokens = true;
                    return;
                }

                let token = possible_tokens
                    .iter().map(|t| ((nearest_parent_location.unwrap() as i32 - t.start), t))
                    .min_by_key(|(d, _)| d.to_owned())
                    .map(|(_, t)| t);

                debug!("Selected {:#?} as token closest from parent {:#?} as location {:#?}", token.unwrap(), node, nearest_parent_location);

                child_tokens.push(token.unwrap());
            };

            match node {
                #(NodeEnum::#node_identifiers(n) => {#node_handlers}),*,
            };


            if has_too_many_tokens == true {
                ChildTokenRangeResult::TooManyTokens
            } else if child_tokens.len() == 0 {
                ChildTokenRangeResult::NoTokens
            } else {
                ChildTokenRangeResult::ChildTokenRange {
                    used_token_indices: child_tokens.iter().map(|t| t.start).collect(),
                    range: TextRange::new(
                        TextSize::from(child_tokens.iter().min_by_key(|t| t.start).unwrap().start as u32),
                        TextSize::from(child_tokens.iter().max_by_key(|t| t.end).unwrap().end as u32),
                    )
                }
            }
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
            if n.distinct_clause.len() > 0 {
                get_token(TokenProperty::from(Token::Distinct));
            }
        },
        "Integer" => quote! {
            get_token(TokenProperty::from(n));
        },
        "WindowDef" => quote! {
            if n.partition_clause.len() > 0 {
                get_token(TokenProperty::from(Token::Window));
            } else {
                get_token(TokenProperty::from(Token::Over));
            }
        },
        "Boolean" => quote! {
            get_token(TokenProperty::from(n));
        },
        "AStar" => quote! {
            get_token(TokenProperty::from(Token::Ascii42));
        },
        "FuncCall" => quote! {
            if n.agg_filter.is_some() {
                get_token(TokenProperty::from(Token::Filter));
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
                10 => get_token(TokenProperty::from(Token::CurrentRole)),
                // 11 SvfopCurrentUser
                11 => get_token(TokenProperty::from(Token::CurrentUser)),
                // 12 SvfopUser
                // 13 SvfopSessionUser
                // 14 SvfopCurrentCatalog
                // 15 SvfopCurrentSchema
                _ => panic!("Unknown SqlvalueFunction {:#?}", n.op),
            }
        },
        "SortBy" => quote! {
            get_token(TokenProperty::from(Token::Order));
            match n.sortby_dir {
                2 => get_token(TokenProperty::from(Token::Asc)),
                3 => get_token(TokenProperty::from(Token::Desc)),
                _ => {}
            }
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
