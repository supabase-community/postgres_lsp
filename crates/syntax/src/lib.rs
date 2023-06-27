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

mod pg_query_utils;
mod source_file_lexer;
mod source_file_parser;
mod statement_builder;
mod statement_lexer;
mod statement_parser;
mod syntax_error;
mod syntax_kind;

use cstree::green::GreenNode;
use cstree::syntax::SyntaxNode;
use syntax_error::SyntaxError;
use syntax_kind::SyntaxKind;
use triomphe::Arc;

// /// `Parse` is the result of the parsing: a syntax tree and a collection of
// /// errors.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Parse {
//     green: GreenNode,
//     errors: Arc<Vec<SyntaxError>>,
// }
//
// impl Clone for Parse {
//     fn clone(&self) -> Parse {
//         Parse {
//             green: self.green.clone(),
//             errors: self.errors.clone(),
//         }
//     }
// }
//
// impl Parse {
//     fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse {
//         Parse {
//             green,
//             errors: Arc::new(errors),
//         }
//     }
//
//     pub fn syntax_node(&self) -> SyntaxNode<SyntaxKind> {
//         SyntaxNode::new_root(self.green.clone())
//     }
//     pub fn errors(&self) -> &[SyntaxError] {
//         &self.errors
//     }
// }
//
// impl Parse<SourceFile> {
//     pub fn debug_dump(&self) -> String {
//         let mut buf = format!("{:#?}", self.tree().syntax());
//         for err in self.errors.iter() {
//             format_to!(buf, "error {:?}: {}\n", err.range(), err);
//         }
//         buf
//     }
//
//     pub fn reparse(&self, indel: &Indel) -> Parse<SourceFile> {
//         self.incremental_reparse(indel)
//             .unwrap_or_else(|| self.full_reparse(indel))
//     }
//
//     fn incremental_reparse(&self, indel: &Indel) -> Option<Parse<SourceFile>> {
//         // FIXME: validation errors are not handled here
//         parsing::incremental_reparse(self.tree().syntax(), indel, self.errors.to_vec()).map(
//             |(green_node, errors, _reparsed_range)| Parse {
//                 green: green_node,
//                 errors: Arc::new(errors),
//                 _ty: PhantomData,
//             },
//         )
//     }
//
//     fn full_reparse(&self, indel: &Indel) -> Parse<SourceFile> {
//         let mut text = self.tree().syntax().text().to_string();
//         indel.apply(&mut text);
//         SourceFile::parse(&text)
//     }
// }
//
// /// `SourceFile` represents a parse tree for a single Rust file.
// pub use crate::ast::SourceFile;
//
// impl SourceFile {
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
