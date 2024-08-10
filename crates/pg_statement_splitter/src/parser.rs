use std::cmp::min;

use pg_lexer::{SyntaxKind, Token, TokenType, WHITESPACE_TOKENS};
use text_size::{TextRange, TextSize};

use crate::syntax_error::SyntaxError;

/// Main parser that exposes the `cstree` api, and collects errors and statements
pub struct Parser {
    /// The ranges of the statements
    ranges: Vec<(usize, usize)>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The start of the current statement, if any
    current_stmt_start: Option<usize>,
    /// The tokens to parse
    pub tokens: Vec<Token>,
    /// The current position in the token stream
    pub pos: usize,
    /// index from which whitespace tokens are buffered
    pub whitespace_token_buffer: Option<usize>,

    eof_token: Token,
}

/// Result of Building
#[derive(Debug)]
pub struct Parse {
    /// The ranges of the errors
    pub ranges: Vec<TextRange>,
    /// The syntax errors accumulated during parsing
    pub errors: Vec<SyntaxError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            eof_token: Token::eof(usize::from(tokens.last().unwrap().span.end())),
            ranges: Vec::new(),
            errors: Vec::new(),
            current_stmt_start: None,
            tokens,
            pos: 0,
            whitespace_token_buffer: None,
        }
    }

    pub fn finish(self) -> Parse {
        Parse {
            ranges: self
                .ranges
                .iter()
                .map(|(start, end)| {
                    let from = self.tokens.get(*start);
                    let to = self.tokens.get(end - 1);
                    // get text range from token range
                    let text_start = from.unwrap().span.start();
                    let text_end = to.unwrap().span.end();

                    TextRange::new(
                        TextSize::try_from(text_start).unwrap(),
                        TextSize::try_from(text_end).unwrap(),
                    )
                })
                .collect(),
            errors: self.errors,
        }
    }

    /// collects an SyntaxError with an `error` message at `pos`
    pub fn error_at_pos(&mut self, error: String, pos: usize) {
        self.errors.push(SyntaxError::new_at_offset(
            error,
            self.tokens
                .get(min(self.tokens.len() - 1, pos))
                .unwrap()
                .span
                .start(),
        ));
    }

    /// applies token and advances
    pub fn advance(&mut self) {
        assert!(!self.eof());
        let token = self.nth(0, false);
        if token.kind == SyntaxKind::Whitespace {
            if self.whitespace_token_buffer.is_none() {
                self.whitespace_token_buffer = Some(self.pos);
            }
        } else {
            self.flush_token_buffer();
        }
        self.pos += 1;
    }

    /// flush token buffer and applies all tokens
    pub fn flush_token_buffer(&mut self) {
        if self.whitespace_token_buffer.is_none() {
            return;
        }
        while self.whitespace_token_buffer.unwrap() < self.pos {
            self.whitespace_token_buffer = Some(self.whitespace_token_buffer.unwrap() + 1);
        }
        self.whitespace_token_buffer = None;
    }

    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn at_whitespace(&self) -> bool {
        self.nth(0, false).kind == SyntaxKind::Whitespace
    }

    pub fn eat_whitespace(&mut self) {
        while self.nth(0, false).token_type == TokenType::Whitespace {
            self.advance();
        }
    }

    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// lookbehind method.
    ///
    /// if `ignore_whitespace` is true, it will skip all whitespace tokens
    pub fn lookbehind(&self, lookbehind: usize, ignore_whitespace: bool) -> Option<&Token> {
        if ignore_whitespace {
            let mut idx = 0;
            let mut non_whitespace_token_ctr = 0;
            loop {
                match self.tokens.get(self.pos - idx) {
                    Some(token) => {
                        if !WHITESPACE_TOKENS.contains(&token.kind) {
                            non_whitespace_token_ctr += 1;
                            if non_whitespace_token_ctr == lookbehind {
                                return Some(token);
                            }
                        }
                        idx += 1;
                    }
                    None => {
                        if (self.pos - idx) > 0 {
                            idx += 1;
                        } else {
                            return None;
                        }
                    }
                }
            }
        } else {
            self.tokens.get(self.pos - lookbehind)
        }
    }

    /// lookahead method.
    ///
    /// if `ignore_whitespace` is true, it will skip all whitespace tokens
    pub fn nth(&self, lookahead: usize, ignore_whitespace: bool) -> &Token {
        if ignore_whitespace {
            let mut idx = 0;
            let mut non_whitespace_token_ctr = 0;
            loop {
                match self.tokens.get(self.pos + idx) {
                    Some(token) => {
                        if !WHITESPACE_TOKENS.contains(&token.kind) {
                            if non_whitespace_token_ctr == lookahead {
                                return token;
                            }
                            non_whitespace_token_ctr += 1;
                        }
                        idx += 1;
                    }
                    None => {
                        return &self.eof_token;
                    }
                }
            }
        } else {
            match self.tokens.get(self.pos + lookahead) {
                Some(token) => token,
                None => &self.eof_token,
            }
        }
    }

    /// checks if the current token is of `kind`
    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0, false).kind == kind
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        if self.eat(kind) {
            return;
        }
        if self.whitespace_token_buffer.is_some() {
            self.error_at_pos(
                format!(
                    "Expected {:#?}, found {:#?}",
                    kind,
                    self.tokens[self.whitespace_token_buffer.unwrap()].kind
                ),
                self.whitespace_token_buffer.unwrap(),
            );
        } else {
            self.error_at_pos(
                format!("Expected {:#?}, found {:#?}", kind, self.nth(0, false)),
                self.pos + 1,
            );
        }
    }
}
