// TODOS
// 1. parse statement ranges
// 2. parse sql statement -> if error return error, else return root node (ast) + node tree with
//    ref to ast node and range + cst

#![feature(lazy_cell, is_sorted)]

mod codegen;
mod lexer;
mod parser;
mod syntax_error;
mod syntax_node;

use text_size::TextRange;

pub fn extract_sql_statement_ranges(sql: &str) -> Vec<TextRange> {
    vec![]
}
