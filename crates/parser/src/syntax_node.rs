//! This module defines Concrete Syntax Tree (CST), used by postgres_lsp.
//!
//! The CST includes comments and whitespace, provides a single node type,
//! `SyntaxNode`, and a basic traversal API (parent, children, siblings).
//!
//! The *real* implementation is in the (language-agnostic) `cstree` crate, this
//! module just wraps its API.

use crate::syntax_kind::SyntaxKind;

pub type SyntaxNode = cstree::syntax::SyntaxNode<SyntaxKind>;
pub type SyntaxToken = cstree::syntax::SyntaxToken<SyntaxKind>;
pub type SyntaxElement = cstree::syntax::SyntaxElement<SyntaxKind>;
