mod ast;
mod cst;
mod parser;
mod statement_parser;
mod syntax_builder;

pub use ast::AST;
pub use cst::CST;

use statement_parser::StatementParser;
use syntax_builder::{Syntax, SyntaxBuilder};

pub fn parse_syntax(sql: &str, root: &pg_query_ext::NodeEnum) -> Syntax {
    let mut builder = SyntaxBuilder::new();

    StatementParser::new(root, sql, &mut builder).parse();

    builder.finish()
}
