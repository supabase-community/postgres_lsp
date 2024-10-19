use pg_lexer::{SyntaxKind, Token, TokenType};

use super::{
    data::at_statement_start,
    dml::{cte, delete, insert, select, update},
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
                // we might want to ignore TokenType::NoKeyword here too
                // but this will lead to invalid statements to not being picked up
                token_type: TokenType::Whitespace,
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
            insert(p);
        }
        SyntaxKind::Update => {
            update(p);
        }
        SyntaxKind::DeleteP => {
            delete(p);
        }
        _ => {
            unknown(p);
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
                kind: SyntaxKind::Ascii59,
                ..
            } => {
                p.advance();
                break;
            }
            Token {
                kind: SyntaxKind::Newline | SyntaxKind::Eof,
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
