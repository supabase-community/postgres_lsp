use cstree::syntax::ResolvedNode;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use pg_query::NodeEnum;

use crate::ast_node::RawStmt;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind_codegen::SyntaxKind;
use crate::syntax_node::SyntaxNode;

/// Main parser that controls the cst building process, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
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
            errors: Vec::new(),
            stmts: Vec::new(),
            checkpoint: None,
            is_parsing_flat_node: false,
            open_nodes: Vec::new(),
        }
    }

    /// close all nodes until the specified depth is reached
    pub fn close_until_depth(&mut self, depth: i32) {
        debug!("close until depth {}", depth);
        if self.open_nodes.is_empty() || self.get_current_depth() < depth {
            return;
        }
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
    pub fn set_checkpoint(&mut self) {
        assert!(
            self.checkpoint.is_none(),
            "Must close previouos checkpoint before setting new one"
        );
        self.checkpoint = Some(self.get_current_depth());
    }

    /// close all nodes until checkpoint depth is reached
    pub fn close_checkpoint(&mut self) {
        if self.checkpoint.is_some() {
            self.close_until_depth(self.checkpoint.unwrap());
        }
        self.checkpoint = None;
        self.is_parsing_flat_node = false;
    }

    /// start a new node of `SyntaxKind` at `depth`
    /// handles closing previous nodes if necessary
    pub fn start_node_at(&mut self, kind: SyntaxKind, depth: i32) {
        debug!("starting node at depth {} {:?}", depth, kind);
        // close until target depth
        self.close_until_depth(depth);

        self.open_nodes.push((kind, depth));
        debug!("start node {:?}", kind);
        self.inner.start_node(kind);
    }

    /// finish current node
    pub fn finish_node(&mut self) {
        debug!("finish_node");

        let n = self.open_nodes.pop();
        if n.is_none() {
            panic!("No node to finish");
        }

        debug!("finish node {:?}", n.unwrap().0);
        self.inner.finish_node();
    }

    /// applies token
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
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
}
