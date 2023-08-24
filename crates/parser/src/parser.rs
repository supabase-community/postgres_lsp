use std::collections::{HashMap, VecDeque};
use std::ops::RangeBounds;

use cstree::syntax::ResolvedNode;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use pg_query::NodeEnum;

use crate::ast_node::RawStmt;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind_generated::SyntaxKind;
use crate::syntax_node::SyntaxNode;
use crate::token_type::TokenType;

/// Main parser that controls the cst building process, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// A buffer for tokens that are not yet applied to the cst
    token_buffer: VecDeque<(SyntaxKind, String)>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
    stmts: Vec<RawStmt>,
    /// The current checkpoint depth, if any
    checkpoint: Option<i32>,
    /// Whether the parser is currently parsing a flat node
    is_parsing_flat_node: bool,
    /// Keeps track of currently open nodes
    /// Latest opened is last
    open_nodes: Vec<(SyntaxKind, i32)>,
    /// Keeps track of currently open tokens (e.g. "(")
    /// Latest opened is last
    open_tokens: Vec<(SyntaxKind, String, i32)>,
}

/// Result of parsing
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
    pub fn new() -> Self {
        Self {
            inner: GreenNodeBuilder::new(),
            token_buffer: VecDeque::new(),
            errors: Vec::new(),
            stmts: Vec::new(),
            checkpoint: None,
            is_parsing_flat_node: false,
            open_nodes: Vec::new(),
            open_tokens: Vec::new(),
        }
    }

    /// close all nodes until the specified depth is reached
    pub fn close_until_depth(&mut self, depth: i32) {
        if self.open_nodes.is_empty() || self.get_current_depth() < depth {
            return;
        }
        debug!(
            "close from depth {:?} until {:?}",
            self.get_current_depth(),
            depth
        );
        loop {
            if self.open_nodes.is_empty() || self.get_current_depth() < depth {
                break;
            }
            self.finish_node();
        }
    }

    fn get_current_depth(&self) -> i32 {
        self.open_nodes[self.open_nodes.len() - 1].1
    }

    /// set a checkpoint at current depth
    ///
    /// if `is_parsing_flat_node` is true, all tokens parsed until this checkpoint is closed will be applied immediately
    pub fn set_checkpoint(&mut self, is_parsing_flat_node: bool) {
        assert!(
            self.checkpoint.is_none(),
            "Must close previouos checkpoint before setting new one"
        );
        assert!(
            self.token_buffer.is_empty(),
            "Token buffer must be empty before setting a checkpoint"
        );
        self.checkpoint = Some(self.get_current_depth());
        self.is_parsing_flat_node = is_parsing_flat_node;
    }

    /// close all nodes until checkpoint depth is reached
    pub fn close_checkpoint(&mut self) {
        self.consume_token_buffer(None);
        if self.checkpoint.is_some() {
            self.close_until_depth(self.checkpoint.unwrap());
        }
        self.checkpoint = None;
        self.is_parsing_flat_node = false;
    }

    /// start a new node of `SyntaxKind` at `depth`
    /// handles closing previous nodes if necessary
    /// and consumes token buffer before starting new node
    pub fn start_node_at(&mut self, kind: SyntaxKind, depth: i32) {
        debug!("start node {:?} at depth: {:?}", kind, depth);
        // close until target depth
        self.close_until_depth(depth);

        self.consume_token_buffer(None);

        self.open_nodes.push((kind, depth));
        self.inner.start_node(kind);
    }

    /// Applies closing sibling for open tokens at current depth
    ///
    /// FIXME: find closing token in token buffer, instead of just comparing with the first one
    fn consume_open_tokens(&mut self) {
        if self.open_tokens.is_empty() || self.token_buffer.is_empty() {
            return;
        }
        let depth = self.get_current_depth();
        debug!("close open tokens at depth: {:?}", depth);
        loop {
            if self.open_tokens.is_empty() || self.token_buffer.is_empty() {
                break;
            }
            let (token, _, token_depth) = self.open_tokens[self.open_tokens.len() - 1];
            if token_depth != depth {
                break;
            }
            println!("token: {:?}", token);
            println!("token_depth: {:?}", token_depth);
            // find closing token in token buffer
            let closing_token_pos = self
                .token_buffer
                .iter()
                .position(|t| t.0 == token.sibling().unwrap());
            if closing_token_pos.is_none() {
                break;
            }
            let closing_token_pos = closing_token_pos.unwrap();

            // drain token buffer until closing token inclusively
            self.open_tokens.pop();
            self.consume_token_buffer(Some((closing_token_pos + 1) as u32));
        }
    }

    /// finish current node
    /// applies all open tokens of current depth before
    pub fn finish_node(&mut self) {
        self.consume_open_tokens();

        let n = self.open_nodes.pop();
        if n.is_none() {
            panic!("No node to finish");
        }

        let (node, depth) = n.unwrap();
        debug!("finish node {:?} at {:?}", node, depth);
        self.inner.finish_node();
    }

    /// Drains the token buffer and applies all tokens
    pub fn consume_token_buffer(&mut self, until: Option<u32>) {
        if self.token_buffer.is_empty() {
            return;
        }
        let range = match until {
            Some(u) => 0..u as usize,
            None => 0..self.token_buffer.len(),
        };
        debug!("consume token buffer {:?}", range);
        for (kind, text) in self.token_buffer.drain(range).collect::<VecDeque<_>>() {
            debug!("consuming token: {:?} {:?}", kind, text);
            self.apply_token(kind, text.as_str());
        }
    }

    /// Applies token immediately
    /// if token is opening sibling, adds it to open tokens
    fn apply_token(&mut self, kind: SyntaxKind, text: &str) {
        self.inner.token(kind, text);
        if kind.is_opening_sibling() {
            let depth = self.get_current_depth();
            debug!("open token {:?} at depth {:?}", kind, depth);
            self.open_tokens.push((kind, text.to_string(), depth));
        }
    }

    /// applies token based on its `SyntaxKindType`
    /// if `SyntaxKindType::Close`, closes all nodes until depth 1
    /// if `SyntaxKindType::Follow`, add token to buffer and wait until next node or non-Follow token to apply token at same depth
    /// otherwise, applies token immediately
    ///
    /// if `is_parsing_flat_node` is true, applies token immediately
    pub fn token(&mut self, kind: SyntaxKind, text: &str, token_type: Option<TokenType>) {
        if self.is_parsing_flat_node {
            self.inner.token(kind, text);
            return;
        }

        match token_type {
            Some(TokenType::Close) => {
                // move up to depth 2 and consume buffered tokens before applying closing token
                self.close_until_depth(2);
                self.consume_token_buffer(None);
                self.inner.token(kind, text);
            }
            Some(TokenType::Follow) => {
                debug!("push to buffer token {:?} {:?}", kind, text);
                // wait until next node, and apply token at same depth
                self.token_buffer.push_back((kind, text.to_string()));
            }
            _ => {
                self.consume_token_buffer(None);
                debug!("apply token {:?} {:?}", kind, text);
                self.apply_token(kind, text);
            }
        }
    }

    /// collects an SyntaxError with an `error` message at `range`
    pub fn error(&mut self, error: String, range: TextRange) {
        self.errors.push(SyntaxError::new(error, range));
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
}
