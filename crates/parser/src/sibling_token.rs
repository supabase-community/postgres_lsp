use crate::syntax_kind_codegen::SyntaxKind;

const SIBLINGS: [(SyntaxKind, SyntaxKind); 1] = [(SyntaxKind::Ascii40, SyntaxKind::Ascii41)];

impl SyntaxKind {
    pub fn is_closing_sibling(self) -> bool {
        SIBLINGS.iter().any(|(_, close)| *close == self)
    }

    pub fn is_opening_sibling(self) -> bool {
        SIBLINGS.iter().any(|(open, _)| *open == self)
    }

    pub fn get_closing_sibling(self) -> SyntaxKind {
        SIBLINGS
            .iter()
            .find_map(|(open, close)| if *open == self { Some(*close) } else { None })
            .unwrap()
    }

    pub fn get_opening_sibling(self) -> SyntaxKind {
        SIBLINGS
            .iter()
            .find_map(|(open, close)| if *close == self { Some(*open) } else { None })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_siblings() {
        assert_eq!(SyntaxKind::Ascii40.is_opening_sibling(), true);
        assert_eq!(
            SyntaxKind::Ascii40.get_closing_sibling(),
            SyntaxKind::Ascii41
        );
    }

    #[test]
    #[should_panic]
    fn test_mismatched_siblings() {
        SyntaxKind::Ascii41.get_closing_sibling();
    }
}
