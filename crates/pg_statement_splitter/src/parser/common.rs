use pg_lexer::{SyntaxKind, Token, TokenType};

use super::{
    data::at_statement_start,
    dml::{cte, select},
    Parser,
};

pub fn source(p: &mut Parser) {
    loop {
        match p.peek() {
            Token {
                kind: SyntaxKind::Eof,
                ..
            } => {
                break;
            }
            Token {
                token_type: TokenType::Whitespace | TokenType::NoKeyword,
                ..
            } => {
                p.advance();
            }
            _ => {
                statement(p);
            }
        }
    }
}

pub(crate) fn statement(p: &mut Parser) {
    p.start_stmt();
    match p.peek().kind {
        SyntaxKind::With => {
            cte(p);
        }
        SyntaxKind::Select => {
            select(p);
        }
        SyntaxKind::Insert => {
            todo!();
            // insert(p);
        }
        SyntaxKind::Update => {
            todo!();
            // update(p);
        }
        SyntaxKind::DeleteP => {
            todo!();
            // delete(p);
        }
        t => {
            panic!("stmt: Unknown token {:?}", t);
            // unknown(p);
        }
    }
    p.close_stmt();
}

pub(crate) fn parenthesis(p: &mut Parser) {
    p.expect(SyntaxKind::Ascii40);

    loop {
        match p.peek().kind {
            SyntaxKind::Ascii41 | SyntaxKind::Eof => {
                p.advance();
                break;
            }
            _ => {
                p.advance();
            }
        }
    }
}

pub(crate) fn unknown(p: &mut Parser) {
    loop {
        match p.peek() {
            Token {
                kind: SyntaxKind::Newline | SyntaxKind::Ascii59 | SyntaxKind::Eof,
                ..
            } => {
                break;
            }
            Token {
                kind: SyntaxKind::Ascii40,
                ..
            } => {
                parenthesis(p);
            }
            t => {
                if at_statement_start(t.kind) {
                    break;
                }

                p.advance();
            }
        }
    }
}
