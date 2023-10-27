use cstree::build::Checkpoint;
use cstree::syntax::ResolvedNode;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use pg_query::NodeEnum;

use crate::ast_node::RawStmt;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind_codegen::SyntaxKind;
use crate::syntax_node::SyntaxNode;

/// Main parser that exposes the `cstree` api, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
    stmts: Vec<RawStmt>,
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
    pub fn new() -> Self {
        Self {
            inner: GreenNodeBuilder::new(),
            errors: Vec::new(),
            stmts: Vec::new(),
        }
    }

    /// start a new node of `SyntaxKind`
    pub fn start_node(&mut self, kind: SyntaxKind) {
        debug!("start_node: {:?}", kind);
        self.inner.start_node(kind);
    }

    /// finish current node
    pub fn finish_node(&mut self) {
        debug!("finish_node");
        self.inner.finish_node();
    }

    /// applies token
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        debug!("token: {:?} {:?}", kind, text);
        self.inner.token(kind, text);
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
        self.inner.start_node_at(checkpoint, kind);
    }
}
