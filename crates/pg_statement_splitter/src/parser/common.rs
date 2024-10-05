use pg_lexer::{SyntaxKind, Token};

use super::{
    dml::{cte, select},
    Parser,
};

pub fn source(p: &mut Parser) {
    loop {
        // todo find a better way to handle stmt start
        // same problem as below... for the first token we need to use nth(0),
        // but for the rest we need to use peek
        p.start_stmt();
        statement(p);
        p.close_stmt();

        if p.eof(true) {
            break;
        }
    }
}

pub(crate) fn statement(p: &mut Parser) {
    // todo find a better way to handle first token
    let token = if p.pos == 0 {
        p.nth(0, true)
    } else {
        p.peek(true)
    };

    match token.kind {
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
}

pub(crate) fn parenthesis(p: &mut Parser) {
    p.expect(SyntaxKind::Ascii40, true);

    loop {
        if p.eof(true) {
            p.expect(SyntaxKind::Ascii41, true);
            break;
        }
        if p.nth(0, true).kind == SyntaxKind::Ascii41 {
            break;
        }
    }
}

pub(crate) fn unknown(p: &mut Parser) {
    loop {
        match p.peek(false) {
            t @ Token {
                kind: SyntaxKind::Newline,
                ..
            } => {
                if t.text.chars().count() > 1 {
                    p.advance(false);
                    break;
                }
            }
            Token {
                // ";"
                kind: SyntaxKind::Ascii59,
                ..
            } => {
                p.advance(false);
                break;
            }
            Token {
                kind: SyntaxKind::Eof,
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
                println!("Unknown token {:?}", t);
                p.advance(false);
            }
        }
    }
}
