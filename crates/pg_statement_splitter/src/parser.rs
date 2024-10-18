mod common;
mod data;
mod dml;

pub use common::source;

use pg_lexer::{lex, SyntaxKind, Token, WHITESPACE_TOKENS};
use text_size::{TextRange, TextSize};

use crate::syntax_error::SyntaxError;

/// Main parser that exposes the `cstree` api, and collects errors and statements
/// It is modelled after a Pratt Parser. For a gentle introduction to Pratt Parsing, see https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
pub struct Parser {
    /// The ranges of the statements
    ranges: Vec<TextRange>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The start of the current statement, if any
    current_stmt_start: Option<TextSize>,
    /// The tokens to parse
    pub tokens: Vec<Token>,

    eof_token: Token,

    last_token_end: Option<TextSize>,
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
    pub fn new(sql: &str) -> Self {
        // we dont care about whitespace tokens, except for double newlines
        // to make everything simpler, we just filter them out
        // the token holds the text range, so we dont need to worry about that
        let tokens = lex(sql)
            .iter()
            .filter(|t| {
                return !WHITESPACE_TOKENS.contains(&t.kind)
                    || (t.kind == SyntaxKind::Newline && t.text.chars().count() > 1);
            })
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        Self {
            ranges: Vec::new(),
            eof_token: Token::eof(usize::from(
                tokens
                    .first()
                    .map(|t| t.span.start())
                    .unwrap_or(TextSize::from(0)),
            )),
            errors: Vec::new(),
            current_stmt_start: None,
            tokens,
            last_token_end: None,
        }
    }

    pub fn finish(self) -> Parse {
        Parse {
            ranges: self.ranges,
            errors: self.errors,
        }
    }

    /// Start statement
    pub fn start_stmt(&mut self) -> Token {
        assert!(self.current_stmt_start.is_none());

        let token = self.peek();

        self.current_stmt_start = Some(token.span.start());

        token
    }

    /// Close statement
    pub fn close_stmt(&mut self) {
        self.ranges.push(TextRange::new(
            self.current_stmt_start.expect("Expected active statement"),
            self.last_token_end.expect("Expected last token end"),
        ));

        self.current_stmt_start = None;
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.pop().unwrap_or(self.eof_token.clone());

        self.last_token_end = Some(token.span.end());

        token
    }

    fn peek(&mut self) -> Token {
        self.tokens
            .last()
            .cloned()
            .unwrap_or(self.eof_token.clone())
    }

    /// checks if the current token is of `kind` and advances if true
    /// returns true if the current token is of `kind`
    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.peek().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        if self.eat(kind) {
            return;
        }

        self.error_at(format!("Expected {:#?}", kind));
    }

    /// collects an SyntaxError with an `error` message at the current position
    fn error_at(&mut self, error: String) {
        todo!();
    }
}
