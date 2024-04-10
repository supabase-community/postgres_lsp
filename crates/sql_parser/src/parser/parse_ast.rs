mod ast_builder;
mod sql_statement_parser;
mod tree_builder;

use pg_query::NodeEnum;

use self::{sql_statement_parser::SqlStatementParser, tree_builder::TreeBuilder};

pub use ast_builder::EnrichedAst;
pub use tree_builder::Cst;

pub struct ParsedStatement {
    /// The abstract syntax tree with resolved ranges for each node
    pub ast: EnrichedAst,
    /// The concrete syntax tree
    pub cst: Cst,
}

pub fn parse_ast(sql: &str, root: &NodeEnum) -> ParsedStatement {
    let mut builder = TreeBuilder::new();

    SqlStatementParser::new(&root, sql, &mut builder).parse();

    let (cst, ast) = builder.finish();

    ParsedStatement { ast, cst }
}
