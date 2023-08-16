use pg_query_proto_parser::{FieldType, ProtoFile, ProtoParser};
use sourcegen::{
    Attribute, Builder, Comment, Enum, Function, Implementation, Imports, Match, SourceFile, Struct,
};
use std::fs;

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
                .with_import("std::collections".to_string(), vec!["VecDeque".to_string()])
                .with_import("crate".to_string(), vec!["pg_query_utils_manual::derive_location".to_string()])
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
            .with_attribute(
                Attribute::new("derive".to_string())
                .with_argument("Debug".to_string(), None)
                .with_argument("Clone".to_string(), None)
                .finish()
            )
            .with_field("node".to_string(), "NodeEnum".to_string(), true)
            .with_field("depth".to_string(), "i32".to_string(), true)
            .with_field("location".to_string(), "i32".to_string(), true)
            .with_field("path".to_string(), "String".to_string(), true)
            .finish()
        )
        .add_block(
            Function::new("get_children".to_string())
                .public()
                .with_comment("Returns all children of the node, recursively".to_string())
                .with_parameter("node".to_string(), Some("&NodeEnum".to_string()))
                .with_parameter("text".to_string(), Some("String".to_string()))
                .with_parameter("current_depth".to_string(), Some("i32".to_string()))
                .with_return_type("Vec<NestedNode>".to_string())
                .with(|b| {
                    let mut content = "let mut nodes: Vec<NestedNode> = vec![];\n".to_string();
                    content.push_str("// Node, depth, path\n");
                    content.push_str("let mut stack: VecDeque<(NodeEnum, i32, String)> = VecDeque::from(vec![(node.to_owned(), current_depth, \"0\".to_string())]);");
                    content.push_str("// Node, depth, path\n");
                    content.push_str("let mut location_stack: VecDeque<(NodeEnum, i32, String)> = VecDeque::new();");
                    content.push_str("while !stack.is_empty() || !location_stack.is_empty() {\n");
                    content.push_str("if !stack.is_empty() {");
                    content.push_str("let (node, depth, path) = stack.pop_front().unwrap();\n");
                    content.push_str("let current_depth = depth + 1;\n");
                    content.push_str("let mut child_ctr: i32 = 0;\n");
                    content.push_str("let mut handle_child = |c: NodeEnum| {\n");
                    content.push_str("let location = get_location(&c);\n");
                    content.push_str("let path = path.clone() + \".\" + child_ctr.to_string().as_str();\n");
                    content.push_str("child_ctr = child_ctr + 1;\n");
                    content.push_str("stack.push_back((c.to_owned(), current_depth, path.clone()));\n");
                    content.push_str("if location.is_some() {\n");
                    content.push_str("nodes.push(NestedNode {\n");
                    content.push_str("node: c,\n");
                    content.push_str("depth: current_depth,\n");
                    content.push_str("location: location.unwrap(),\n");
                    content.push_str("path: path.clone(),\n");
                    content.push_str("});\n");
                    content.push_str("} else {\n");
                    content.push_str("location_stack.push_back((c, current_depth, path));\n");
                    content.push_str("}\n");
                    content.push_str("};\n");

                    let match_ = Match::new("&node".to_string())
                        .with(|b| {
                            f.nodes.iter().for_each(|node| {
                                if node.name == "AConst" {
                                    // AConst is the only node with one of, so we handle it
                                    // manually
                                    let content = "{
            if n.val.is_some() {
                handle_child(match n.val.to_owned().unwrap() {
                    pg_query::protobuf::a_const::Val::Ival(v) => NodeEnum::Integer(v),
                    pg_query::protobuf::a_const::Val::Fval(v) => NodeEnum::Float(v),
                    pg_query::protobuf::a_const::Val::Boolval(v) => NodeEnum::Boolean(v),
                    pg_query::protobuf::a_const::Val::Sval(v) => NodeEnum::String(v),
                    pg_query::protobuf::a_const::Val::Bsval(v) => NodeEnum::BitString(v),
                });
            }
        }";

                                    b.with_arm(
                                        format!("NodeEnum::{}(n)", node.name.to_string()),
                                        format!("{}", content),
                                    );
                                } else {
                                    let mut field_content: Vec<String> = vec![];
                                    node.fields.iter().for_each(|field| {
                                        if field.field_type == FieldType::Node && field.repeated {
                                            field_content.push(
                                                format!(
                                                    "n.{}.iter().for_each(|x| handle_child(x.node.as_ref().unwrap().to_owned()));\n",
                                                    field.name.to_string()
                                                )
                                            );
                                        } else if field.field_type == FieldType::Node && field.is_one_of == false {
                                            if field.node_name == Some("Node".to_owned()) {
                                                let mut node_content = "".to_string();
                                                node_content.push_str(format!("if n.{}.is_some() {{\n", field.name.to_string()).as_str());
                                                node_content.push_str(
                                                    format!(
                                                        "handle_child(n.{}.to_owned().unwrap().node.unwrap());\n",
                                                        field.name.to_string()
                                                    ).as_str(),
                                                );
                                                node_content.push_str("}\n");
                                                field_content.push(node_content);
                                            } else {
                                                let mut node_content = "".to_string();
                                                node_content.push_str(format!("if n.{}.is_some() {{\n", field.name.to_string()).as_str());
                                                node_content.push_str(
                                                    format!(
                                                        "handle_child(NodeEnum::{}(n.{}.to_owned().unwrap()));\n",
                                                        field.enum_variant_name.as_ref().unwrap(),
                                                        field.name.to_string()
                                                    )
                                                    .as_str()
                                                );
                                                node_content.push_str("}\n");
                                                field_content.push(node_content);
                                            }
                                        }
                                    });

                                    let content = if field_content.len() > 0 {
                                        format!("{{\n{}\n}}", field_content.join("\n"))
                                    } else {
                                        "()".to_string()
                                    };

                                    b.with_arm(
                                        format!("NodeEnum::{}(n)", node.name.to_string()),
                                        format!("{}", content),
                                    );
                                }
                            });

                            b
                        })
                        .finish();

                    content.push_str(match_.to_string().as_str());
                    content.push_str(";\n");

                    content.push_str("} else if !location_stack.is_empty() {\n");
                    content.push_str("let (node, depth, path) = location_stack.pop_front().unwrap();\n");
                    content.push_str("let parent_location = nodes.iter().find(|n| {\n");
                    content.push_str("let mut path_elements = path.split(\".\").collect::<Vec<&str>>();\n");
                    content.push_str("path_elements.pop();\n");
                    content.push_str("let parent_path = path_elements.join(\".\");\n");
                    content.push_str("n.path == parent_path\n");
                    content.push_str("}).unwrap().location;\n");
                    content.push_str("let earliest_child_location = nodes.iter().filter(|n| n.path.starts_with(path.as_str())).min_by(|a, b| a.location.cmp(&b.location)).map(|n| n.location);\n");
                    content.push_str("let location = derive_location(&node, text.clone(), parent_location, earliest_child_location);\n");
                    content.push_str("nodes.push(NestedNode { node, depth, location, path: path.clone() });\n");
                    content.push_str("}\n");
                    content.push_str("}\n");


                    content.push_str("nodes.sort_by_key(|n| n.location);\n");
                    content.push_str("nodes");

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
                b.with_value(node.name.to_string(), None);
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
                        b.with_arm(
                            format!("NodeEnum::{}(_)", node.name.to_string()),
                            format!("SyntaxKind::{}", node.name.to_string()),
                        );
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
