use parser::SyntaxKind;
use tower_lsp::lsp_types::SemanticTokenType;

/// Semantic token types that are used for highlighting
pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    // For identifiers that declare or reference a namespace, module, or package.
    SemanticTokenType::NAMESPACE,
    // For identifiers that declare or reference a class type.
    SemanticTokenType::CLASS,
    // For identifiers that declare or reference an enumeration type.
    SemanticTokenType::ENUM,
    // For identifiers that declare or reference an interface type.
    // SemanticTokenType::INTERFACE,
    // For identifiers that declare or reference a struct type.
    // SemanticTokenType::STRUCT,
    // For identifiers that declare or reference a type parameter.
    SemanticTokenType::TYPE_PARAMETER,
    // For identifiers that declare or reference a type that is not covered above.
    SemanticTokenType::TYPE,
    // For identifiers that declare or reference a function or method parameters.
    SemanticTokenType::PARAMETER,
    // For identifiers that declare or reference a local or global variable.
    SemanticTokenType::VARIABLE,
    // For identifiers that declare or reference a member property, member field, or member variable.
    SemanticTokenType::PROPERTY,
    // For identifiers that declare or reference an enumeration property, constant, or member.
    SemanticTokenType::ENUM_MEMBER,
    // For identifiers that declare or reference decorators and annotations.
    // SemanticTokenType::DECORATOR,
    // For identifiers that declare an event property.
    SemanticTokenType::EVENT,
    // For identifiers that declare a function.
    SemanticTokenType::FUNCTION,
    // For identifiers that declare a member function or method.
    SemanticTokenType::METHOD,
    // For identifiers that declare a macro.
    // SemanticTokenType::MACRO,
    // For tokens that represent a comment.
    SemanticTokenType::COMMENT,
    // For tokens that represent a string literal.
    SemanticTokenType::STRING,
    // For tokens that represent a language keyword.
    SemanticTokenType::KEYWORD,
    // For tokens that represent a number literal.
    SemanticTokenType::NUMBER,
    // For tokens that represent a regular expression literal.
    SemanticTokenType::REGEXP,
    // For tokens that represent an operator.
    SemanticTokenType::OPERATOR,
];

#[derive(Debug, Clone)]
pub struct ImCompleteSemanticToken {
    pub start: usize,
    pub length: usize,
    pub token_type: usize,
}

pub fn semantic_token_from_syntax_kind(syntax: SyntaxKind) -> Option<usize> {
    let token_type = match syntax {
        SyntaxKind::Ascii37 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii42 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii43 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii44 => Some(SemanticTokenType::PROPERTY),
        SyntaxKind::Ascii45 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii47 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii60 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii62 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Ascii63 => Some(SemanticTokenType::OPERATOR),
        SyntaxKind::Sconst => Some(SemanticTokenType::STRING),
        SyntaxKind::Comment => Some(SemanticTokenType::COMMENT),
        SyntaxKind::Select => Some(SemanticTokenType::KEYWORD),
        SyntaxKind::From => Some(SemanticTokenType::KEYWORD),
        SyntaxKind::Where => Some(SemanticTokenType::KEYWORD),
        SyntaxKind::ColumnRef => Some(SemanticTokenType::PROPERTY),
        SyntaxKind::RangeVar => Some(SemanticTokenType::CLASS),
        _ => None,
    };
    if let Some(token_type) = token_type {
        return LEGEND_TYPE.iter().position(|item| item == &token_type);
    }
    None
}
