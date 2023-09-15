use crate::syntax_kind_codegen::SyntaxKind;

impl SyntaxKind {
    pub fn is_opening_sibling(&self) -> bool {
        match self {
            SyntaxKind::Ascii40 => true,
            SyntaxKind::Ascii91 => true,
            SyntaxKind::Case => true,
            _ => false,
        }
    }
    pub fn is_closing_sibling(&self) -> bool {
        match self {
            SyntaxKind::Ascii41 => true,
            SyntaxKind::Ascii93 => true,
            SyntaxKind::EndP => true,
            _ => false,
        }
    }
    pub fn sibling(&self) -> Option<SyntaxKind> {
        match self {
            SyntaxKind::Case => Some(SyntaxKind::EndP),
            SyntaxKind::EndP => Some(SyntaxKind::Case),
            SyntaxKind::Ascii40 => Some(SyntaxKind::Ascii41),
            SyntaxKind::Ascii41 => Some(SyntaxKind::Ascii40),
            SyntaxKind::Ascii91 => Some(SyntaxKind::Ascii93),
            SyntaxKind::Ascii93 => Some(SyntaxKind::Ascii91),
            _ => None,
        }
    }
}
