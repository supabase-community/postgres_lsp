use crate::codegen::SyntaxKind;
use crate::Parser;

use super::statement::statement;
use super::statement_start::is_at_stmt_start;

pub fn source(parser: &mut Parser) {
    parser.start_node(SyntaxKind::SourceFile);

    while !parser.eof() {
        match is_at_stmt_start(parser) {
            Some(stmt) => {
                statement(parser, stmt);
            }
            None => {
                parser.advance();
            }
        }
    }

    parser.finish_node();
}
