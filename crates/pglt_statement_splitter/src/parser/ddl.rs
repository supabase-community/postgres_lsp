use pglt_lexer::SyntaxKind;

use super::{Parser, common::unknown};

pub(crate) fn create(p: &mut Parser) {
    p.expect(SyntaxKind::Create);

    unknown(p, &[]);
}

pub(crate) fn alter(p: &mut Parser) {
    p.expect(SyntaxKind::Alter);

    unknown(p, &[SyntaxKind::Alter]);
}
