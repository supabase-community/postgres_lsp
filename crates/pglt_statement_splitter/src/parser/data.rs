use pglt_lexer::SyntaxKind;

// All tokens listed here must be explicitly handled in the `unknown` function to ensure that we do
// not break in the middle of another statement that contains a statement start token.
//
// All of these statements must have a dedicated parser function called from the `statement` function
static STATEMENT_START_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::With,
    SyntaxKind::Select,
    SyntaxKind::Insert,
    SyntaxKind::Update,
    SyntaxKind::DeleteP,
    SyntaxKind::Create,
    SyntaxKind::Alter,
];

pub(crate) fn at_statement_start(kind: SyntaxKind, exclude: &[SyntaxKind]) -> Option<&SyntaxKind> {
    STATEMENT_START_TOKENS
        .iter()
        .filter(|&x| !exclude.contains(x))
        .find(|&x| x == &kind)
}
