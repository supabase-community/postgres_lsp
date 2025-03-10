use pglt_lexer::SyntaxKind;

use super::{common::unknown, Parser};

pub(crate) fn create(p: &mut Parser) {
    p.expect(SyntaxKind::Create);

    unknown(
        p,
        &[
            SyntaxKind::Insert,
            SyntaxKind::Update,
            SyntaxKind::DeleteP,
            SyntaxKind::Select,
        ],
    );
}

pub(crate) fn alter(p: &mut Parser) {
    p.expect(SyntaxKind::Alter);

    unknown(p, &[SyntaxKind::Alter]);
}
