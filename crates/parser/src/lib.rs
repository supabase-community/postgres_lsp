//! The SQL parser.
//!
//!
//
// TODO: implement parser similarly to rust_analyzer
// result is a stream of events (including errors) and a list of errors
//
//
//
// we can use Vec::new() in constructor and then set nodes in parse() if parsing was successful
//
//
//
//
// differences to rust_analyzer
// 1.
// since we always have to parse just text, there is no need to have lexer and parser separated
// input of the parser is a string and we always parse the full string
// syntax crate does not know about lexers and their tokens
// --> input is just a string
// 2.
// in rust_analyzer, the output is just a stream of 32-bit encoded events WITHOUT the text
// again, this extra layer of abstraction is not necessary for us, since we always parse text
// the output of the parser is pretty much the same as the input but with nodes
// --> the parser takes fn that is called for every node and token to build the tree
// so we skip the intermediate list of events and just build the tree directly
// we can define a trait that is implemented by the GreenNodeBuilder
//
//
// SyntaxNode in the syntax create is just the SyntaxKind from the parser
// cst is build with the SyntaxKind type
// in the syntax crate, the SyntaxTreeBuilder is created and the events are fed into it to build
// the three
//
//
// how does rust_analyzer know what parts of text is an error?
// errors are not added to the tree in SyntaxTreeBuilder, which means the tokens must include the
// erronous parts of the text
// but the parser output does not include text, so how does the cst can have correct text?
// easy: the tokenizer is running beforehand, so we always have the tokens, and the errors are just
// added afterwards when parsing the tokens using the grammar.
// so there is a never-failing tokenizer step which is followed by the parser that knows the
// grammar and emits errors
// --> we will do the same, but with a multi-step tokenizer and parser that fallbacks to simpler
// and simpler tokens
//
//
// api has to cover parse source file and parse statement
//
//
// we will also have to add a cache for pg_query parsing results using fingerprinting
//
// all parsers can be just a function that iterates the base lexer
// so we will have a `parse_statement` and a `parse_source_file` function
// the tree always covers all text since we use the scantokens and, if failing, the StatementTokens
// errors are added to a list, and are not part of the tree

mod ast_node;
mod parser;
mod pg_query_utils;
mod source_file;
mod statement;
mod syntax_error;
mod syntax_kind;
mod syntax_node;

pub use crate::parser::{Parse, Parser};
pub use crate::syntax_kind::SyntaxKind;
pub use crate::syntax_node::{SyntaxElement, SyntaxNode, SyntaxToken};
