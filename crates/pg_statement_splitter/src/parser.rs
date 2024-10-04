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

                    TextRange::new(text_start, text_end)
                })
                .collect(),
            errors: self.errors,
        }
    }

    /// Start statement at last non-whitespace token
    pub fn start_stmt(&mut self) {
        assert!(self.current_stmt_start.is_none());

        if let Some(whitespace_token_buffer) = self.whitespace_token_buffer {
            self.current_stmt_start = Some(whitespace_token_buffer);
        } else {
            self.current_stmt_start = Some(self.pos);
        }
    }

    /// Close statement at last non-whitespace token
    pub fn close_stmt(&mut self) {
        assert!(self.current_stmt_start.is_some());

        self.ranges.push((
            self.current_stmt_start.unwrap(),
            self.whitespace_token_buffer.unwrap_or(self.pos),
        ));

        self.current_stmt_start = None;
    }

    /// applies token and advances
    pub fn advance(&mut self) {
        assert!(!self.eof());

        if self.nth(0).kind == SyntaxKind::Whitespace {
            if self.whitespace_token_buffer.is_none() {
                self.whitespace_token_buffer = Some(self.pos);
            }
        } else {
            self.flush_token_buffer();
        }
        self.pos += 1;
    }

    /// checks if the current token is of `kind` and advances if true
    /// returns true if the current token is of `kind`
    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.nth(0).kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn at_whitespace(&self) -> bool {
        self.nth(0).kind == SyntaxKind::Whitespace
    }

    pub fn peek(&self) -> &Token {
        self.nth(1)
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        if self.nth(0).kind == kind {
            return;
        }

        self.error_at(format!("Expected {:#?}", kind));
    }

    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// flush token buffer and applies all tokens
    fn flush_token_buffer(&mut self) {
        if self.whitespace_token_buffer.is_none() {
            return;
        }
        while self.whitespace_token_buffer.unwrap() < self.pos {
            self.whitespace_token_buffer = Some(self.whitespace_token_buffer.unwrap() + 1);
        }
        self.whitespace_token_buffer = None;
    }

    pub fn next(&mut self) -> &Token {
        loop {
            if self.at_whitespace() {
                self.advance();
                continue;
            }
            break;
        }

        self.nth(0)
    }

    /// collects an SyntaxError with an `error` message at the current position
    fn error_at(&mut self, error: String) {
        self.errors.push(SyntaxError::new_at_offset(
            error,
            self.tokens
                .get(min(
                    self.tokens.len() - 1,
                    self.whitespace_token_buffer.unwrap_or(self.pos),
                ))
                .unwrap()
                .span
                .start(),
        ));
    }

    /// lookahead method.
    fn nth(&self, lookahead: usize) -> &Token {
        match self.tokens.get(self.pos + lookahead) {
            Some(token) => token,
            None => &self.eof_token,
        }
    }
}

