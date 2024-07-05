#![feature(lazy_cell, is_sorted)]

mod codegen;
mod lexer;
mod parser;
mod syntax_error;
mod syntax_node;

pub use codegen::{get_location, ChildrenIterator};
pub use parser::extract_sql_statement_ranges::extract_sql_statement_ranges;
pub use parser::parse_ast::{parse_ast, Cst, EnrichedAst, ParsedStatement};
pub use parser::parse_sql_statement::parse_sql_statement;
pub use pg_query::protobuf as pg_query_protobuf;
pub use pg_query::{Error as NativeError, NodeEnum as AstNode};
