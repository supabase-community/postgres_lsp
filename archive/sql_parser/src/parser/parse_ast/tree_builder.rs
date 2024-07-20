use cstree::{build::GreenNodeBuilder, syntax::ResolvedNode};

use crate::{
    codegen::SyntaxKind,
    parser::{EventSink, ParserEvent},
    syntax_node::SyntaxNode,
};

use super::ast_builder::{AstBuilder, EnrichedAst};

pub type Cst = ResolvedNode<SyntaxKind>;

pub(super) struct TreeBuilder {
    ast_builder: AstBuilder,
    cst_builder: GreenNodeBuilder<'static, 'static, SyntaxKind>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        Self {
            ast_builder: AstBuilder::new(),
            cst_builder: GreenNodeBuilder::new(),
        }
    }

    pub fn finish(self) -> (Cst, EnrichedAst) {
        let (tree, cache) = self.cst_builder.finish();
        let ast = self.ast_builder.finish();
        (
            SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap()),
            ast,
        )
    }
}

impl EventSink for TreeBuilder {
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
