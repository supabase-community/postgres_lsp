use pg_lexer::SyntaxKind;

use super::{
    common::{parenthesis, statement, unknown},
    Parser,
};

pub(crate) fn cte(p: &mut Parser) {
    p.expect(SyntaxKind::With, true);

    loop {
        p.expect(SyntaxKind::Ident, true);
        p.expect(SyntaxKind::As, true);
        parenthesis(p);

        if !p.eat(SyntaxKind::Ascii44, true) {
            break;
        }
    }

    statement(p);
}

pub(crate) fn select(p: &mut Parser) {
    p.expect(SyntaxKind::Select, true);

    unknown(p);
}
