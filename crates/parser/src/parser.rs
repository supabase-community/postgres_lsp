use cstree::build::Checkpoint;
use cstree::syntax::ResolvedNode;
use cstree::text::TextSize;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use pg_query::NodeEnum;
use std::cmp::min;
use std::ops::Range;

use crate::ast_node::RawStmt;
use crate::codegen::SyntaxKind;
use crate::lexer::{Token, TokenType};
use crate::syntax_error::SyntaxError;
use crate::syntax_node::SyntaxNode;

pub static WHITESPACE_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::Whitespace,
    SyntaxKind::Tab,
    SyntaxKind::Newline,
    SyntaxKind::SqlComment,
];

/// Main parser that exposes the `cstree` api, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abstract syntax tree
    stmts: Vec<RawStmt>,
    /// The tokens to parse
    pub tokens: Vec<Token>,
    /// The current position in the token stream
    pub pos: usize,
    /// index from which whitespace tokens are buffered
    pub whitespace_token_buffer: Option<usize>,
    /// index from which tokens are buffered
    token_buffer: Option<usize>,

    pub depth: usize,

    eof_token: Token,
}

/// Result of Building
#[derive(Debug)]
pub struct Parse {
    /// The concrete syntax tree
    pub cst: ResolvedNode<SyntaxKind>,
    /// The syntax errors accumulated during parsing
    pub errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
    pub stmts: Vec<RawStmt>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            eof_token: Token::eof(usize::from(tokens.last().unwrap().span.end())),
            inner: GreenNodeBuilder::new(),
            errors: Vec::new(),
            stmts: Vec::new(),
            tokens,
            pos: 0,
            whitespace_token_buffer: None,
            token_buffer: None,
            depth: 0,
        }
    }

    /// start a new node of `SyntaxKind`
    pub fn start_node(&mut self, kind: SyntaxKind) {
        debug!("start_node: {:?}", kind);
        self.flush_token_buffer();
        self.inner.start_node(kind);
        self.depth += 1;
    }
    /// finish current node
    pub fn finish_node(&mut self) {
        debug!("finish_node");
        self.inner.finish_node();
        self.depth -= 1;
    }

    /// collects an SyntaxError with an `error` message at `range`
    pub fn error(&mut self, error: String, range: TextRange) {
        self.errors.push(SyntaxError::new(error, range));
    }

    /// collects an SyntaxError with an `error` message at `offset`
    pub fn error_at_offset(&mut self, error: String, offset: TextSize) {
        self.errors.push(SyntaxError::new_at_offset(error, offset));
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

    /// collects a pg_query `stmt` at `range`
    pub fn stmt(&mut self, stmt: NodeEnum, range: TextRange) {
        self.stmts.push(RawStmt { stmt, range });
    }

    /// finish cstree and return `Parse`
    pub fn finish(self) -> Parse {
        let (tree, cache) = self.inner.finish();
        Parse {
            cst: SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap()),
            stmts: self.stmts,
            errors: self.errors,
        }
    }

    /// Prepare for maybe wrapping the next node with a surrounding node.
    ///
    /// The way wrapping works is that you first get a checkpoint, then you add nodes and tokens as
    /// normal, and then you *maybe* call [`start_node_at`](Parser::start_node_at).
    pub fn checkpoint(self) -> Checkpoint {
        self.inner.checkpoint()
    }

    /// Wrap the previous branch marked by [`checkpoint`](Parser::checkpoint) in a new
    /// branch and make it current.
    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.flush_token_buffer();
        self.inner.start_node_at(checkpoint, kind);
    }

    /// Opens a buffer for tokens. While the buffer is active, tokens are not applied to the tree.
    pub fn open_buffer(&mut self) {
        self.token_buffer = Some(self.pos);
    }

    /// Closes the current token buffer, resets the position to the start of the buffer and returns the range of buffered tokens.
    pub fn close_buffer(&mut self) -> Range<usize> {
        let token_buffer = self.token_buffer.unwrap();
        let token_range = token_buffer..self.whitespace_token_buffer.unwrap_or(self.pos);
        self.token_buffer = None;
        self.pos = token_buffer;
        token_range
    }

    /// applies token and advances
    pub fn advance(&mut self) {
        assert!(!self.eof());
        if self.nth(0, false).kind == SyntaxKind::Whitespace {
            if self.whitespace_token_buffer.is_none() {
                self.whitespace_token_buffer = Some(self.pos);
            }
        } else {
            self.flush_token_buffer();
            if self.token_buffer.is_none() {
                let token = self.tokens.get(self.pos).unwrap();
                self.inner.token(token.kind, &token.text);
            }
        }
        self.pos += 1;
    }

    /// flush token buffer and applies all tokens
    pub fn flush_token_buffer(&mut self) {
        if self.whitespace_token_buffer.is_none() {
            return;
        }
        while self.whitespace_token_buffer.unwrap() < self.pos {
            let token = self
                .tokens
                .get(self.whitespace_token_buffer.unwrap())
                .unwrap();
            if self.token_buffer.is_none() {
                self.inner.token(token.kind, &token.text);
            }
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

    /// checks if the current token is any of `kinds`
    pub fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.iter().any(|&it| self.at(it))
    }

    /// checks if the current token is of `kind`
    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0, false).kind == kind
    }

    /// like at, but for multiple consecutive tokens
    pub fn at_all(&self, kinds: &[SyntaxKind]) -> bool {
        kinds
            .iter()
            .enumerate()
            .all(|(idx, &it)| self.nth(idx, false).kind == it)
    }

    /// like at_any, but for multiple consecutive tokens
    pub fn at_any_all(&self, kinds: &Vec<&[SyntaxKind]>) -> bool {
        kinds.iter().any(|&it| self.at_all(it))
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

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    use crate::{lexer::lex, parse::source::source};

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parser_another() {
        init();

        let input = "SELECT 1 from contact c
        JOIN pg_class c ON nc.oid = c.relnamespace
        left join (
            select
            table_id,
            jsonb_agg(_pk.*) as primary_keys
            from (
                select 1
                from
                pg_class c,
                pg_attribute a
                where
                i.indrelid = c.oid
                and c.relnamespace = n.oid
                ) as _pk
            group by table_id
            ) as pk
            on pk.table_id = c.oid;";

        let mut p = Parser::new(lex(input));
        source(&mut p);
        let result = p.finish();

        dbg!(&result.cst);
        println!("{:#?}", result.errors);
    }

    #[test]
    fn test_parser_very_simple() {
        init();

        panic_after(Duration::from_millis(100), || {
            let input = "select * from public.contact where x = 1;";

            let mut p = Parser::new(lex(input));
            source(&mut p);
            let result = p.finish();

            dbg!(&result.cst);
            println!("{:#?}", result.errors);
        })
    }

    #[test]
    fn test_nested_substatements() {
        init();

        let input = "select is ((select true), true);\nselect isnt ((select false), true);";

        let mut p = Parser::new(lex(input));
        source(&mut p);
        let result = p.finish();

        dbg!(&result.cst);
        assert_eq!(result.stmts.len(), 2);
        println!("{:#?}", result.errors);
    }

    #[test]
    fn test_parser_simple() {
        init();

        let input = "alter table x rename to y \n\n alter table x alter column z set default 1";

        let mut p = Parser::new(lex(input));
        source(&mut p);
        let result = p.finish();

        dbg!(&result.cst);
        assert_eq!(result.stmts.len(), 2);
        result.stmts.iter().for_each(|x| {
            dbg!(&x.range);
            dbg!(&x.stmt);
        });
        println!("{:#?}", result.errors);
    }

    fn panic_after<T, F>(d: Duration, f: F) -> T
    where
        T: Send + 'static,
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let (done_tx, done_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let val = f();
            done_tx.send(()).expect("Unable to send completion signal");
            val
        });

        match done_rx.recv_timeout(d) {
            Ok(_) => handle.join().expect("Thread panicked"),
            Err(_) => panic!("Thread took too long"),
        }
    }
}
