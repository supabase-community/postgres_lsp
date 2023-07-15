use cstree::syntax::ResolvedNode;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use pg_query::NodeEnum;

use crate::ast_node::RawStmt;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind::{SyntaxKind, SyntaxKindType};
use crate::syntax_node::SyntaxNode;

/// Main parser that controls the cst building process, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// A buffer for tokens that are not yet applied to the cst
    token_buffer: Vec<(SyntaxKind, String)>,
    /// The current depth of the cst
    curr_depth: i32,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
    stmts: Vec<RawStmt>,
    /// The current checkpoint depth, if any
    checkpoint: Option<i32>,
    /// Whether the parser is currently parsing a flat node
    is_parsing_flat_node: bool,
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
            curr_depth: -1,
            inner: GreenNodeBuilder::new(),
            token_buffer: Vec::new(),
            errors: Vec::new(),
            stmts: Vec::new(),
            checkpoint: None,
            is_parsing_flat_node: false,
        }
    }

    /// close all nodes until the specified depth is reached
    pub fn close_until_depth(&mut self, depth: i32) {
        while self.curr_depth >= depth {
            self.finish_node();
            self.curr_depth -= 1;
        }
    }

    /// set a checkpoint at current depth
    ///
    /// if `is_parsing_flat_node` is true, all tokens will be applied immediately
    pub fn set_checkpoint(&mut self, is_parsing_flat_node: bool) {
        assert!(
            self.checkpoint.is_none(),
            "Must close previouos checkpoint before setting new one"
        );
        assert!(
            self.token_buffer.is_empty(),
            "Token buffer must be empty before setting a checkpoint"
        );
        self.checkpoint = Some(self.curr_depth);
        self.is_parsing_flat_node = is_parsing_flat_node;
    }

    /// close all nodes until checkpoint depth is reached
    pub fn close_checkpoint(&mut self) {
        self.consume_token_buffer();
        if self.checkpoint.is_some() {
            self.close_until_depth(self.checkpoint.unwrap());
        }
        self.checkpoint = None;
        self.is_parsing_flat_node = false;
    }

    /// start a new node of `SyntaxKind`
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.inner.start_node(kind);
    }

    /// start a new node of `SyntaxKind` at `depth`
    /// handles closing previous nodes if necessary
    /// and consumes token buffer before starting new node
    ///
    /// if `SyntaxKind` is `SyntaxKind::AnyStatement`, sets `is_parsing_erronous_node` to true
    pub fn start_node_at(&mut self, kind: SyntaxKind, depth: Option<i32>) {
        let depth = depth.unwrap_or(self.curr_depth + 1);
        // close until target depth
        self.close_until_depth(depth);

        self.consume_token_buffer();

        self.curr_depth = depth;
        self.start_node(kind);
    }

    /// finish current node
    pub fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    /// Drains the token buffer and applies all tokens
    pub fn consume_token_buffer(&mut self) {
        for (kind, text) in self.token_buffer.drain(..) {
            self.inner.token(kind, &text);
        }
    }

    /// applies token based on its `SyntaxKindType`
    /// if `SyntaxKindType::Close`, closes all nodes until depth 1
    /// if `SyntaxKindType::Follow`, add token to buffer and wait until next node to apply token at same depth
    /// otherwise, applies token immediately
    ///
    /// if `is_parsing_erronous_node` is true, applies token immediately
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        if self.is_parsing_flat_node {
            self.inner.token(kind, text);
            return;
        }

        match kind.get_type() {
            Some(SyntaxKindType::Close) => {
                // move up to depth 2 and consume buffered tokens before applying closing token
                self.close_until_depth(2);
                self.consume_token_buffer();
                self.inner.token(kind, text);
            }
            Some(SyntaxKindType::Follow) => {
                // wait until next node, and apply token at same depth
                self.token_buffer.push((kind, text.to_string()));
            }
            _ => {
                self.inner.token(kind, text);
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
