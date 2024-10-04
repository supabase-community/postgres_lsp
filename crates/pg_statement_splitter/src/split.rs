use pg_lexer::{SyntaxKind, Token, TokenType};

use crate::{data::at_statement_start, parser::Parser};

pub(crate) fn parse_source(p: &mut Parser) {
    loop {
        match p.peek() {
            Token {
                kind: SyntaxKind::Eof,
                ..
            } => {
                break;
            }
            Token {
                token_type: TokenType::Whitespace,
                ..
            } => {
                p.advance();
            }
            _ => {
                parse_statement(p);
            }
        }
    }
}

fn parse_statement(p: &mut Parser) {
    p.start_stmt();
    // todo move the below into parse_dml so that we dont have conflicts with parse stmt
    match p.peek().kind {
        SyntaxKind::With => {
            parse_cte(p);
        }
        SyntaxKind::Select => {
            parse_select(p);
        }
        SyntaxKind::Insert => {
            parse_insert(p);
        }
        SyntaxKind::Update => {
            parse_update(p);
        }
        SyntaxKind::DeleteP => {
            parse_delete(p);
        }
        _ => {
            parse_unknown(p);
        }
    }
    p.close_stmt();
}

fn parse_cte(p: &mut Parser) {
    println!("Parsing cte statement");
    p.start_stmt();

    // todo make adance and all methods that call advance ignore whitespace
    p.expect(SyntaxKind::With);

    loop {
        p.expect(SyntaxKind::Ident);
        p.expect(SyntaxKind::As);
        parse_parenthesis(p);

        // todo handle comma
        if !p.eat(SyntaxKind::Ascii00) {
            break;
        }
    }

    parse_statement(p);
}

// todo add common checker for within statements that checks for parenthesis, semicolons, statement
// starts etc and then we can add custom ones eg union for select
fn parse_select(p: &mut Parser) {
    println!("Parsing select statement");
    p.start_stmt();

    p.expect(SyntaxKind::Select);

    loop {
        println!("parse select at {:?}", p.current().kind);
        if p.eat(SyntaxKind::Ascii59) {
            break;
        }

        if p.at_double_newline() {
            break;
        }

        if p.at(SyntaxKind::Eof) {
            break;
        }

        if p.at(SyntaxKind::Ascii40) {
            parse_parenthesis(p);
        }

        if [
            SyntaxKind::Insert,
            SyntaxKind::Update,
            SyntaxKind::DeleteP,
            SyntaxKind::Select,
        ]
        .contains(&p.peek().kind)
        {
            break;
        }

        p.advance();
    }

    p.close_stmt();
}

fn parse_parenthesis(p: &mut Parser) {
    p.expect(SyntaxKind::Ascii40);

    loop {
        if p.eof() {
            p.expect(SyntaxKind::Ascii41);
            break;
        }
        if p.at(SyntaxKind::Ascii41) {
            break;
        }
    }
}

fn parse_insert(p: &mut Parser) {
    p.expect(SyntaxKind::Insert);
    p.expect(SyntaxKind::Into);
}

fn parse_update(p: &mut Parser) {
    p.expect(SyntaxKind::Update);
}

fn parse_delete(p: &mut Parser) {
    p.expect(SyntaxKind::DeleteP);
    p.expect(SyntaxKind::From);

    p.eat_whitespace();
}



