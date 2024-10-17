use pg_lexer::{SyntaxKind, Token, TokenType, WHITESPACE_TOKENS};

pub struct Parser {
    /// The tokens to parse
    pub tokens: Vec<Token>,
    /// The current position in the token stream
    pub pos: usize,
    /// index from which whitespace tokens are buffered
    pub whitespace_token_buffer: Option<usize>,

    eof_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            eof_token: Token::eof(usize::from(tokens.last().unwrap().span.end())),
            tokens,
            pos: 0,
            whitespace_token_buffer: None,
        }
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
    pub fn lookbehind(
        &self,
        lookbehind: usize,
        ignore_whitespace: bool,
        start_before: Option<usize>,
    ) -> Option<&Token> {
        if ignore_whitespace {
            let mut idx = 0;
            let mut non_whitespace_token_ctr = 0;
            loop {
                if idx > self.pos {
                    return None;
                }
                match self.tokens.get(self.pos - start_before.unwrap_or(0) - idx) {
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
                        if (self.pos - idx - start_before.unwrap_or(0)) > 0 {
                            idx += 1;
                        } else {
                            return None;
                        }
                    }
                }
            }
        } else {
            self.tokens
                .get(self.pos - lookbehind - start_before.unwrap_or(0))
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
}
