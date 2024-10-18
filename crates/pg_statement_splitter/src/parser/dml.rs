use pg_lexer::SyntaxKind;

use super::{
    common::{parenthesis, statement, unknown},
    Parser,
};

pub(crate) fn cte(p: &mut Parser) {
    p.expect(SyntaxKind::With);

    loop {
        p.expect(SyntaxKind::Ident);
        p.expect(SyntaxKind::As);
        parenthesis(p);

        if !p.eat(SyntaxKind::Ascii44) {
            break;
        }
    }

    statement(p);
}

pub(crate) fn select(p: &mut Parser) {
    p.expect(SyntaxKind::Select);

    unknown(p);
}
