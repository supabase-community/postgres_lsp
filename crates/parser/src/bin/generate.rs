use pg_query_proto_parser::{ProtoFile, ProtoParser};
use sourcegen::{Attribute, Builder, Comment, Enum, Function, Implementation, Match, SourceFile};
use std::fs;

fn main() {
    let parser = ProtoParser::new("./proto/source.proto");
    let file = parser.parse();

    fs::write(
        "./src/syntax_kind_generated.rs",
        generate_syntax_kind(&file),
    )
    .unwrap();
}

fn generate_pg_query_utils(f: &ProtoFile) -> String {
    SourceFile::new()
        .add_comment(
            Comment::new("//!".to_string())
                .with_text("Utilities for working with pg_query.rs".to_string())
                .with_text("This file is generated from the libg_query proto".to_string())
                .finish(),
        )
        .add_block(
            Function::new("get_position_for_pg_query_node".to_string())
                .public()
                .with_parameter("node".to_string(), "&NodeRef".to_string())
                .with_return_type("i32".to_string())
                .with_body(
                    Match::new("node".to_string())
                        .with_arm(pattern, body)
                        .finish(),
                )
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
        Enum::new("SyntaxKind".to_string())
        .public()
        .with_comment("An u32 enum of all valid syntax elements (nodes and tokens) of the postgres sql dialect, and a few custom ones that are not parsed by pg_query.rs, such as `Whitespace`.".to_string())
        .with_attribute(
            Attribute::new("derive".to_string())
            .with_argument("Copy".to_string(), None)
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
        Implementation::new("SyntaxKind".to_string())
        .add_block(
            Function::new("new_from_pg_query_node".to_string())
            .public()
            .with_comment(
                Comment::new("//".to_string())
                .with_text("Converts a `pg_query` node to a `SyntaxKind`".to_string())
                .finish()
            )
            .with_return_type("Self".to_string())
            .with_parameter("node".to_string(), "&NodeRef".to_string())
            .with_body(
                Match::new("node.kind()".to_string())
                .with(|b| {
                    f.nodes.iter().for_each(|node| {
                        b.with_arm(
                            format!("NodeRef::{}(_)", node.name.to_string()),
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
                Comment::new("//".to_string())
                .with_text("Converts a `pg_query` token to a `SyntaxKind`".to_string())
                .finish()
            )
            .with_return_type("Self".to_string())
            .with_parameter("token".to_string(), "&ScanToken".to_string())
            .with_body(
                Match::new("token.token".to_string())
               .with(|b| {
                   f.tokens.iter().for_each(|token| {
                       b.with_arm(
                           token.value.to_string(),
                           format!("SyntaxKind::{}", token.name.to_string()),
                       );
                   });
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
