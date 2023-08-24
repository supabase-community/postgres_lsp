//! The Postgres parser.
//!
//! This crate provides a parser for the Postgres SQL dialect.
//! It is based in the pg_query.rs crate, which is a wrapper around the PostgreSQL query parser.
//! The main `Parser` struct parses a source file and individual statements.
//! The `Parse` struct contains the resulting concrete syntax tree, syntax errors, and the abtract syntax tree, which is a list of pg_query statements and their positions.
//!
//! The idea is to offload the heavy lifting to the same parser that the PostgreSQL server uses,
//! and just fill in the gaps to be able to build both cst and ast from a a source file that
//! potentially contains erroneous statements.
//!
//! The main drawbacks of the PostgreSQL query parser mitigated by this parser are:
//! - it only parsed a full source text, and if there is any syntax error in a file, it will not parse anything and return an error.
//! - it does not parse whitespaces and newlines, so it is not possible to build a concrete syntax tree build a concrete syntax tree.
//!
//! To see how these drawbacks are mitigated, see the `statement.rs` and the `source_file.rs` module.

mod ast_node;
mod parser;
mod pg_query_utils_generated;
mod pg_query_utils_generated_test;
mod pg_query_utils_manual;
mod sibling_token;
mod source_file;
mod statement;
mod syntax_error;
mod syntax_kind_generated;
mod syntax_node;
mod token_type;

pub use crate::parser::{Parse, Parser};
pub use crate::syntax_kind_generated::SyntaxKind;
pub use crate::syntax_node::{SyntaxElement, SyntaxNode, SyntaxToken};
