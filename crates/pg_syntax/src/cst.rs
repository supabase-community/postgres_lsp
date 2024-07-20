use cstree::syntax::ResolvedNode;
use pg_lexer::SyntaxKind;

pub type CST = ResolvedNode<SyntaxKind>;

pub type SyntaxNode = cstree::syntax::SyntaxNode<SyntaxKind>;
#[allow(dead_code)]
pub type SyntaxToken = cstree::syntax::SyntaxToken<SyntaxKind>;
#[allow(dead_code)]
pub type SyntaxElement = cstree::syntax::SyntaxElement<SyntaxKind>;
