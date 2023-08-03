use pg_query_proto_parser::{FieldType, ProtoFile, ProtoParser};
use sourcegen::{
    Attribute, Builder, Comment, Enum, Function, Implementation, Imports, Match, SourceFile, Struct,
};
use std::fs;

// NodeEnum::Expr(_) => todo!(),
// NodeEnum::SqlvalueFunction(_) => todo!(),
// NodeEnum::CreatePlangStmt(_) => todo!(),
// NodeEnum::AlterTsdictionaryStmt(_) => todo!(),
// NodeEnum::AlterTsconfigurationStmt(_) => todo!(),
// NodeEnum::Null(_) => todo!(),

const UNKNOWN_NODES: [&str; 0] = [
    // ignore nodes not known to pg_query.rs
];

fn main() {
    let parser = ProtoParser::new("./proto/source.proto");
    let file = parser.parse();

    fs::write(
        "./src/syntax_kind_generated.rs",
        generate_syntax_kind(&file),
    )
    .unwrap();

    fs::write(
        "./src/pg_query_utils_generated.rs",
        generate_pg_query_utils(&file),
    )
    .unwrap();
}

fn generate_pg_query_utils(f: &ProtoFile) -> String {
    SourceFile::new()
        .add_comment("Utilities for working with pg_query.rs".to_string())
        .add_comment("This file is generated from the libg_query proto".to_string())
        .add_block(
            Imports::new()
                .with_import(
                    "pg_query".to_string(),
                    vec!["NodeEnum".to_string()],
                )
                .finish(),
        )
        .add_block(
            Function::new("get_location".to_string())
                .public()
                .with_parameter("node".to_string(), Some("&NodeEnum".to_string()))
                .with_return_type("Option<i32>".to_string())
                .with_body(
                    Match::new("node".to_string())
                        .with(|b| {
                            f.nodes.iter().for_each(|node| {
                                let mut right = "None";
                                let mut left = format!("NodeEnum::{}(_)", node.name.to_string());
                                if node
                                    .fields
                                    .iter()
                                    .find(|n| {
                                        n.name == "location" && n.field_type == FieldType::Int32
                                    })
                                    .is_some()
                                {
                                    right = "Some(n.location)";
                                    left = format!("NodeEnum::{}(n)", node.name.to_string());
                                }

                                b.with_arm(left.to_string(), right.to_string());
                            });
                            b
                        })
                        .finish(),
                )
                .finish(),
        )
        .add_block(
            Struct::new("NestedNode".to_string())
            .public()
            .with_field("node".to_string(), "NodeEnum".to_string())
            .with_field("depth".to_string(), "i32".to_string())
            .with_field("location".to_string(), "i32".to_string())
            .finish()
        )
        // TODO: 1) always use match or if .is_some() when unwrapping a node and ignore if none
        // TODO: 2) add location to NestedNode, and use it in get_children.
        // TODO:    to get location, first check if node has location field.
        // TODO:    if not, use the location of the parent.
        // in some cases, for example cte, location is on a generic Node child, which has a node prop that
        // contains the actual node (e.g. insert statement) --> if we get the location within
        // get_children, we can easily get the location of the actual node because we can just
        // fallback to the location of the immediate parent if the current node does not have a
        // location.
        .add_block(
            Function::new("get_children".to_string())
                .public()
                .with_comment("Returns all children of the node, recursively".to_string())
                .with_parameter("node".to_string(), Some("&NodeEnum".to_string()))
                .with_parameter("current_depth".to_string(), Some("i32".to_string()))
                .with_return_type("Vec<NestedNode>".to_string())
                .with(|b| {
                    let mut content = "let current_depth = current_depth + 1;".to_string();
                    let match_ = Match::new("&node".to_string())
                        .with(|b| {
                            f.nodes.iter().for_each(|node| {
                                let mut content = "".to_string();
                                content.push_str("{\n");
                                content.push_str(
                                    "let mut nodes: Vec<NestedNode> = vec![];\n",
                                );
                                node.fields.iter().for_each(|field| {
                                    if field.field_type == FieldType::Node && field.repeated {
                                        content.push_str(
                                            format!(
                                                "nodes.append(&mut n.{}.iter().flat_map(|x| get_children(&x.node.as_ref().unwrap(), current_depth)).collect());\n",
                                                field.name.to_string()
                                            ).as_str()
                                        );
                                    } else if field.field_type == FieldType::Node && field.is_one_of == false {
                                        if field.node_name == Some("Node".to_owned()) {
                                            content.push_str(
                                                format!(
                                                    "let {} = n.{}.as_ref().unwrap().to_owned().node.unwrap();\n",
                                                    field.name.to_string(),
                                                    field.name.to_string(),
                                                ).as_str(),
                                            );
                                            content.push_str(
                                                format!(
                                                    "nodes.append(&mut get_children(&{}, current_depth));\n",
                                                    field.name.to_string()
                                                ).as_str(),

                                            );
                                            content.push_str(
                                                format!(
                                                    "nodes.push(NestedNode {{
                                                        node: {},
                                                        depth: current_depth,
                                                    }});\n",
                                                    field.name.to_string()
                                                ).as_str(),
                                            );
                                        } else {
                                            content.push_str(
                                                format!(
                                                    "let {} = NodeEnum::{}(n.{}.as_ref().unwrap().to_owned());\n",
                                                    field.name.to_string(),
                                                    field.enum_variant_name.as_ref().unwrap(),
                                                    field.name.to_string()
                                                )
                                                .as_str()
                                            );
                                            content.push_str(
                                                format!(
                                                    "nodes.append(&mut get_children(&{}, current_depth));\n",
                                                    field.name.to_string()
                                                ).as_str()
                                            );
                                            content.push_str(
                                                format!(
                                                    "nodes.push(NestedNode {{
                                                        node: {},
                                                        depth: current_depth,
                                                    }});\n",
                                                    field.name.to_string()
                                                ).as_str()
                                            );
                                        }
                                    }
                                    content.push_str("\n");
                                });
                                content.push_str("nodes\n}");
                                b.with_arm(
                                    format!("NodeEnum::{}(n)", node.name.to_string()),
                                    format!("{}", content),
                                );
                            });

                            b
                        })
                        .finish();

                    content.push_str(match_.to_string().as_str());

                    b.with_body(content);

                    b
                })
                .finish(),
        )
        .finish()
}

fn generate_syntax_kind(f: &ProtoFile) -> String {
    SourceFile::new()
    .add_comment(
        Comment::new("//!".to_string())
        .with_text("This module bridges the gap between pg_query.rs nodes, and the `SyntaxKind` cstree requires.".to_string())
        .with_text("The file is generated from the libg_query proto".to_string())
        .finish()
    )
    .add_block(
        Imports::new()
        .with_import("cstree".to_string(), vec!["Syntax".to_string()])
        .with_import("pg_query".to_string(), vec!["protobuf::ScanToken".to_string(), "NodeEnum".to_string()])
        .finish()
    )
    .add_block(
        Enum::new("SyntaxKind".to_string())
        .public()
        .with_comment("An u32 enum of all valid syntax elements (nodes and tokens) of the postgres sql dialect, and a few custom ones that are not parsed by pg_query.rs, such as `Whitespace`.".to_string())
        .with_attribute(
            Attribute::new("derive".to_string())
            .with_argument("Copy".to_string(), None)
            .with_argument("Clone".to_string(), None)
            .with_argument("PartialEq".to_string(), None)
            .with_argument("Eq".to_string(), None)
            .with_argument("PartialOrd".to_string(), None)
            .with_argument("Ord".to_string(), None)
            .with_argument("Hash".to_string(), None)
            .with_argument("Debug".to_string(), None)
            .with_argument("Syntax".to_string(), None)
            .finish()
        )
        .with_attribute(
             Attribute::new("repr".to_string())
            .with_argument("u32".to_string(), None)
            .finish(),
        )
        .with(|b| {
            vec![
                "SourceFile".to_string(),
                "Comment".to_string(),
                "Whitespace".to_string(),
                "Newline".to_string(),
                "Tab".to_string(),
                "Word".to_string(),
                "Stmt".to_string(),
            ].iter().for_each(|kind| {
                b.with_value(kind.to_string(), None);
            });

            f.nodes.iter().for_each(|node| {
                if !UNKNOWN_NODES.contains(&node.name.as_str()) {
                    b.with_value(node.name.to_string(), None);
                }
            });

            f.tokens.iter().for_each(|token| {
                b.with_value(token.name.to_string(), None);
            });

            b
        })
        .finish()
    )
    .add_block(
        Enum::new("SyntaxKindType".to_string())
        .with_comment("
 Kind of a `SyntaxKind`
 This is the only manual definition required for properly creating a concrete syntax tree.
 If a token is of type `Follow`, it is not immediately applied to the syntax tree, but put into
 a buffer. Before the next node is started, all buffered tokens are applied to the syntax tree
 at the depth of the node that is opened next.

 For example, in `select * from contact;`, the whitespace between `*` and `from` should be a direct
 child of the `SelectStmt` node. Without this concept, it would be put into the `ColumnRef`
 node.

 SelectStmt@0..22
   Select@0..6 \"select\"
   Whitespace@6..7 \" \"
   ResTarget@7..8
     ColumnRef@7..8
       Ascii42@7..8 \"*\"
   Whitespace@8..9 \" \"
   From@9..13 \"from\"
  Whitespace@13..14 \" \"
   RangeVar@14..21
     Ident@14..21 \"contact\"
   Ascii59@21..22 \";\"".to_string()
   )
        .public()
        .with_value("Follow".to_string(), None)
        .with_value("Close".to_string(), None)
        .finish()
    )
    .add_block(
        Implementation::new("SyntaxKind".to_string())
        .add_block(
            Function::new("new_from_pg_query_node".to_string())
            .public()
            .with_comment(
                "Converts a `pg_query` node to a `SyntaxKind`".to_string()
            )
            .with_return_type("Self".to_string())
            .with_parameter("node".to_string(), Some("&NodeEnum".to_string()))
            .with_body(
                Match::new("node".to_string())
                .with(|b| {
                    f.nodes.iter().for_each(|node| {
                        if !UNKNOWN_NODES.contains(&node.name.as_str()) {
                            b.with_arm(
                                format!("NodeEnum::{}(_)", node.name.to_string()),
                                format!("SyntaxKind::{}", node.name.to_string()),
                            );
                        }
                    });
                    b
                })
                .finish()
            )
            .finish()
        )
        .add_block(
            Function::new("new_from_pg_query_token".to_string())
            .public()
            .with_comment(
                "Converts a `pg_query` token to a `SyntaxKind`".to_string()
            )
            .with_return_type("Self".to_string())
            .with_parameter("token".to_string(), Some("&ScanToken".to_string()))
            .with_body(
                Match::new("token.token".to_string())
               .with(|b| {
                   f.tokens.iter().for_each(|token| {
                       b.with_arm(
                           token.value.to_string(),
                           format!("SyntaxKind::{}", token.name.to_string()),
                       );
                   });
                   b.with_arm("_".to_string(), "panic!(\"Unknown token\")".to_string());
                   b
               })
               .finish()
            )
            .finish()
        )
        .add_block(
            Function::new("get_type".to_string())
            .public()
            .with_comment(
                "Returns the `SyntaxKindType` of a `SyntaxKind`".to_string()
            )
            .with_return_type("Option<SyntaxKindType>".to_string())
            .with_parameter("&self".to_string(), None)
            .with_body(
                Match::new("self".to_string())
               .with(|b| {
                   f.tokens.iter().for_each(|token| {
                       // Ascii59 (";") closes a statement
                       let value = match token.name.to_string().as_str() {
                            "Ascii59" => "Some(SyntaxKindType::Close)",
                            _ => "Some(SyntaxKindType::Follow)",
                       };
                       b.with_arm(
                           format!("SyntaxKind::{}", token.name.to_string()),
                           value.to_string(),
                        );
                   });
                   b.with_arm(
                       "_".to_string(),
                       "None".to_string(),
                   );
                   b
               })
               .finish()
            )
        .finish()
    )
    .finish()
    )
    .finish()
}
