//! Syntax Tree library used throughout postgres_lsp.
//!
//! The library leverages pg_query.rs to do the heavy lifting of parsing, and just works around its
//! weaknesses.
//! - pg_query.rs only parses valid statements. To circumvent this, we use the very simple expr_lexer to split an
//!   input string into separate sql statements and comments.
//! - pg_query.rs does not parse tokens such aus "(" and ")", and no whitespaces or newlines. We
//! use a simple statement_lexer to parse the missing tokens.
//! - pg_query.rs It has two separate outputs: a list of tokens, and a list of nodes. The
//! statement_parser merges the results of the statement_lexer, and the tokens and nodes from pg_query.rs
//! into a single concrete syntax tree.

mod expr_lexer;
mod pg_query_utils;
mod statement_builder;
mod statement_lexer;
mod statement_parser;
mod syntax_error;
mod syntax_kind;

use cstree::testing::{GreenNode, SyntaxNode};
use syntax_error::SyntaxError;
use triomphe::Arc;

// pg_query.rs only parsed full statements
// which means that conversion between cst and ast is only possible on statement level
// we need to implement a trait for the statement level tokens

// #[derive(Debug, PartialEq, Eq)]
// pub struct Parse<T> {
//     green: GreenNode,
//     errors: Arc<Vec<SyntaxError>>,
// }
//
// impl<T> Clone for Parse<T> {
//     fn clone(&self) -> Parse<T> {
//         Parse {
//             green: self.green.clone(),
//             errors: self.errors.clone(),
//         }
//     }
// }
//
// impl<T> Parse<T> {
//     fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
//         Parse {
//             green,
//             errors: Arc::new(errors),
//         }
//     }
//
//     pub fn syntax_node(&self) -> SyntaxNode<T> {
//         SyntaxNode::new_root(self.green.clone())
//     }
//     pub fn errors(&self) -> &[SyntaxError] {
//         &self.errors
//     }
// }

// impl<T: AstNode> Parse<T> {
//     pub fn to_syntax(self) -> Parse<SyntaxNode> {
//         Parse {
//             green: self.green,
//             errors: self.errors,
//         }
//     }
//
//     pub fn tree(&self) -> T {
//         T::cast(self.syntax_node()).unwrap()
//     }
//
//     pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
//         if self.errors.is_empty() {
//             Ok(self.tree())
//         } else {
//             Err(self.errors)
//         }
//     }
// }
//
// pub use crate::syntax_kind::SyntaxKind;
//
// impl SyntaxKind::SourceFile {
//     pub fn parse(text: &str) -> Parse<SourceFile> {
//         let (green, mut errors) = parsing::parse_text(text);
//         let root = SyntaxNode::new_root(green.clone());
//
//         errors.extend(validation::validate(&root));
//
//         assert_eq!(root.kind(), SyntaxKind::SOURCE_FILE);
//         Parse {
//             green,
//             errors: Arc::new(errors),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
