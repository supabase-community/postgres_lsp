use pg_lexer::SyntaxKind;

// All tokens listed here must be explicitly handled in the `unknown` function to ensure that we do
// not break in the middle of another statement that contains a statement start token.
static STATEMENT_START_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::With,
    SyntaxKind::Select,
    SyntaxKind::Insert,
    SyntaxKind::Update,
    SyntaxKind::DeleteP,
    SyntaxKind::Create,
];

pub(crate) fn at_statement_start(kind: SyntaxKind) -> Option<SyntaxKind> {
    STATEMENT_START_TOKENS.iter().find(|&x| x == &kind).cloned()
}
