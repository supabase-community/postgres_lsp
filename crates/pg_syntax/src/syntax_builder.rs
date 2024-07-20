use cstree::build::GreenNodeBuilder;

use crate::cst::SyntaxNode;
use crate::parser::{EventSink, ParserEvent};

use pg_lexer::SyntaxKind;

use super::ast::{builder::AstBuilder, AST};
use super::cst::CST;

pub struct Syntax {
    /// The abstract syntax tree with resolved ranges for each node
    pub ast: AST,
    /// The concrete syntax tree
    pub cst: CST,
}

pub(super) struct SyntaxBuilder {
    ast_builder: AstBuilder,
    cst_builder: GreenNodeBuilder<'static, 'static, SyntaxKind>,
}

impl SyntaxBuilder {
    pub fn new() -> Self {
        Self {
            ast_builder: AstBuilder::new(),
            cst_builder: GreenNodeBuilder::new(),
        }
    }

    pub fn finish(self) -> Syntax {
        let (tree, cache) = self.cst_builder.finish();
        let ast = self.ast_builder.finish();
        Syntax {
            cst: SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap()),
            ast,
        }
    }
}

impl EventSink for SyntaxBuilder {
    fn push(&mut self, event: ParserEvent) {
        match event {
            ParserEvent::StartNode(node) => {
                self.cst_builder.start_node(SyntaxKind::from(&node));
                self.ast_builder.start_node(node);
            }
            ParserEvent::FinishNode => {
                self.cst_builder.finish_node();
                self.ast_builder.finish_node();
            }
            ParserEvent::Token(token) => {
                self.cst_builder.token(token.kind, token.text.as_str());
                self.ast_builder.token(token.text.as_str());
            }
        }
    }
}
