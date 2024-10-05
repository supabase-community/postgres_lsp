use pg_lexer::SyntaxKind;

pub static STATEMENT_START_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::With,
    SyntaxKind::Select,
    SyntaxKind::Insert,
    SyntaxKind::Update,
    SyntaxKind::DeleteP,
    SyntaxKind::Create,
];

pub(crate) fn at_statement_start(kind: SyntaxKind) -> bool {
    STATEMENT_START_TOKENS.contains(&kind)
}
