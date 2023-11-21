//! The Postgres parser.
//!
//! This crate provides a parser for the Postgres SQL dialect.
//! It is based in the pg_query.rs crate, which is a wrapper around the PostgreSQL query parser.
//! The main `Parser` struct parses a source file and individual statements.
//! The `Parse` result struct contains the resulting concrete syntax tree, syntax errors, and the abtract syntax tree, which is a list of pg_query statements and their positions.
//!
//! The idea is to offload the heavy lifting to the same parser that the PostgreSQL server uses,
//! and just fill in the gaps to be able to build both cst and ast from a source file that
//! potentially contains erroneous statements.
//!
//! The main drawbacks of the PostgreSQL query parser mitigated by this parser are:
//! - it only parsed a full source text, and if there is any syntax error in a file, it will not parse anything and return an error.
//! - it does not parse whitespaces and newlines, and it only returns ast nodes. The concrete syntax tree has to be reverse-engineered.
//!
//! To see how these drawbacks are mitigated, see the `statement_parser.rs` and the `source_parser.rs` module.

#![feature(lazy_cell, is_sorted)]

mod ast_node;
mod codegen;
mod lexer;
mod parse;
mod parser;
mod sibling_token;
mod syntax_error;
mod syntax_node;

use lexer::lex;
use parse::source::source;

pub use crate::codegen::SyntaxKind;
pub use crate::parser::{Parse, Parser};
pub use crate::syntax_node::{SyntaxElement, SyntaxNode, SyntaxToken};

// TODO: I think we should add some kind of `EntryPoint` enum and make the api more flexible
// maybe have an intermediate struct that takes &str inputs, lexes the input and then calls the parser
pub fn parse_source(text: &str) -> Parse {
    let mut p = Parser::new(lex(text));
    source(&mut p);
    p.finish()
}
