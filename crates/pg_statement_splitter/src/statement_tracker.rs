use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};

use crate::data::{StatementDefinition, SyntaxDefinition};

#[derive(Debug)]
pub struct StatementTracker<'a> {
    pub def: &'a StatementDefinition,

    /// position in the definition
    current_pos: usize,

    /// position in the global token stream
    pub started_at: usize,
}

impl<'a> StatementTracker<'a> {
    pub fn new_at(def: &'a StatementDefinition, at: usize) -> Self {
        Self {
            def,
            current_pos: 1,
            started_at: at,
        }
    }

    fn next_possible_tokens(&self) -> Vec<(usize, SyntaxKind)> {
        let mut tokens = Vec::new();

        for (pos, token) in self.def.tokens.iter().enumerate().skip(self.current_pos) {
            match token {
                SyntaxDefinition::RequiredToken(k) => {
                    tokens.push((pos, *k));
                    break;
                }
                SyntaxDefinition::OptionalToken(k) => {
                    tokens.push((pos, *k));
                }
                SyntaxDefinition::AnyTokens => {
                    //
                }
                SyntaxDefinition::AnyToken => {
                    //
                }
                SyntaxDefinition::OneOf(kinds) => {
                    tokens.extend(kinds.iter().map(|x| (pos, *x)));
                    break;
                }
            }
        }

        tokens
    }

    pub fn advance_with(&mut self, kind: &SyntaxKind) -> bool {
        if WHITESPACE_TOKENS.contains(kind) {
            return true;
        }

        let is_valid = match self.def.tokens.get(self.current_pos) {
            Some(SyntaxDefinition::RequiredToken(k)) => {
                self.current_pos += 1;
                k == kind
            }
            Some(SyntaxDefinition::OptionalToken(k)) => {
                if k == kind {
                    self.current_pos += 1;
                } else if let Some(next_token) =
                    self.next_possible_tokens().iter().find(|x| x.1 == *kind)
                {
                    self.current_pos = next_token.0 + 1;
                } else if self.def.tokens.len() - 1 == self.current_pos {
                    // if the optional token is the last one and the previous one is not optional
                    // we must be at the end of the statement
                    if let SyntaxDefinition::RequiredToken(_) =
                        self.def.tokens.get(self.current_pos - 1).unwrap()
                    {
                        return false;
                    }
                }

                true
            }
            Some(SyntaxDefinition::AnyTokens) => {
                assert!(self.next_possible_tokens().len() > 0);

                if let Some(next_token) = self.next_possible_tokens().iter().find(|x| x.1 == *kind)
                {
                    self.current_pos = next_token.0 + 1;
                }

                true
            }
            Some(SyntaxDefinition::AnyToken) => {
                self.current_pos += 1;
                true
            }
            Some(SyntaxDefinition::OneOf(kinds)) => {
                if kinds.iter().any(|x| x == kind) {
                    self.current_pos += 1;
                    true
                } else {
                    false
                }
            }
            None => true,
        };

        is_valid
    }

    pub fn could_be_complete(&self) -> bool {
        self.next_required_token().is_none()
    }

    /// returns the next "required" token we are expecting
    ///
    /// None if we are no required tokens left
    fn next_required_token(&self) -> Option<&SyntaxDefinition> {
        self.def
            .tokens
            .iter()
            .skip(self.current_pos)
            .find(|x| match x {
                SyntaxDefinition::RequiredToken(_) => true,
                SyntaxDefinition::OneOf(_) => true,
                _ => false,
            })
    }
}
