use std::collections::VecDeque;

use cstree::text::{TextRange, TextSize};
use logos::Logos;

use crate::{
    get_nodes_codegen::get_nodes, parser::Parser, resolve_tokens::resolve_tokens,
    syntax_kind_codegen::SyntaxKind,
};

/// A super simple lexer for sql statements.
///
/// One weakness of pg_query.rs is that it does not parse whitespace or newlines. We use a very
/// simple lexer to fill the gaps.
#[derive(Logos, Debug, PartialEq)]
pub enum StatementToken {
    // comments and whitespaces
    #[regex(" +"gm)]
    Whitespace,
    #[regex("\n+"gm)]
    Newline,
    #[regex("\t+"gm)]
    Tab,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

impl StatementToken {
    /// Creates a `SyntaxKind` from a `StatementToken`.
    pub fn syntax_kind(&self) -> SyntaxKind {
        match self {
            StatementToken::Whitespace => SyntaxKind::Whitespace,
            StatementToken::Newline => SyntaxKind::Newline,
            StatementToken::Tab => SyntaxKind::Tab,
            StatementToken::Comment => SyntaxKind::Comment,
        }
    }
}

impl Parser {
    pub fn parse_statement_at(&mut self, text: &str, at_offset: Option<u32>) {
        // 1. Collect as much information as possible from pg_query.rs and `StatementToken` lexer

        // offset of the statement in the source file.
        let offset = at_offset.unwrap_or(0);

        // range of the statement in the source file.
        let range = TextRange::new(
            TextSize::from(offset),
            TextSize::from(offset + text.len() as u32),
        );

        // tokens from pg_query.rs
        let pg_query_tokens = match pg_query::scan(text) {
            Ok(scanned) => scanned.tokens,
            Err(e) => {
                self.error(e.to_string(), range);
                Vec::new()
            }
        };

        // root node of the statement, if no syntax errors
        let pg_query_root = match pg_query::parse(text) {
            Ok(parsed) => Some(
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .find(|n| n.1 == 1)
                    .unwrap()
                    .0
                    .to_enum(),
            ),
            Err(e) => {
                self.error(e.to_string(), range);
                None
            }
        };

        // ranged nodes from pg_query.rs, including the root node
        // the nodes are ordered by starting range, starting with the root node
        let mut pg_query_nodes = match &pg_query_root {
            Some(root) => resolve_tokens(
                &get_nodes(root, text.to_string(), 1),
                &pg_query_tokens,
                &text,
            )
            .into_iter()
            .peekable(),
            None => Vec::new().into_iter().peekable(),
        };

        let mut pg_query_tokens = pg_query_tokens.iter().peekable();

        let mut statement_token_lexer = StatementToken::lexer(&text);

        // 2. Setup data structures required for the parsing algorithm
        // A buffer for tokens that are not applied immediately to the cst
        let mut token_buffer: VecDeque<(SyntaxKind, String)> = VecDeque::new();
        // Keeps track of currently open nodes. Latest opened is last.
        let mut open_nodes: Vec<(SyntaxKind, TextRange, i32)> = Vec::new();

        // 3. Parse the statement

        // Handle root node
        if pg_query_nodes.len() > 0 {
            // if there are no syntax errors, use the pg_query node as the root node
            let root_node = pg_query_nodes
                .find(|n| n.inner.path == "0".to_string())
                .unwrap();
            // can only be at depth 1
            assert_eq!(
                root_node.inner.depth, 1,
                "Root node must be at depth 1, but is at depth {}",
                root_node.inner.depth
            );
            self.stmt(root_node.inner.node.to_owned(), range);
            self.start_node(SyntaxKind::new_from_pg_query_node(&root_node.inner.node));
            open_nodes.push((
                SyntaxKind::new_from_pg_query_node(&root_node.inner.node),
                range,
                1,
            ));
        } else {
            // fallback to generic node as root
            self.start_node(SyntaxKind::Stmt);
            open_nodes.push((SyntaxKind::Stmt, range, 1));
        }

        // start at 0, and increment by the length of the token
        let mut pointer: i32 = 0;

        // main loop that walks through the statement token by token
        while pointer < text.len() as i32 {
            // Check if the pointer is within a pg_query token
            let next_pg_query_token = pg_query_tokens.peek();

            let token_length = if next_pg_query_token.is_some()
                && next_pg_query_token.unwrap().start <= pointer
                && pointer <= next_pg_query_token.unwrap().end
            {
                let token = pg_query_tokens.next().unwrap();

                let token_text = text
                    .chars()
                    .skip(token.start as usize)
                    .take((token.end as usize) - (token.start as usize))
                    .collect::<String>();

                // a node can only start and end with a pg_query token, so we can handle them here

                // before applying the token, close any node that ends before the token starts
                while open_nodes.last().is_some()
                    && open_nodes.last().unwrap().1.end() <= TextSize::from(token.start as u32)
                {
                    self.finish_node();
                    open_nodes.pop();
                }

                // drain token buffer
                for (kind, text) in token_buffer.drain(0..token_buffer.len()) {
                    self.token(kind, text.as_str());
                }

                // apply the token
                self.token(SyntaxKind::new_from_pg_query_token(token), text);

                // consume all nodes that start at or before the token ends
                while pg_query_nodes.peek().is_some()
                    && pg_query_nodes.peek().unwrap().estimated_range.start()
                        <= TextSize::from(token.end as u32)
                {
                    let node = pg_query_nodes.next().unwrap();
                    self.start_node(SyntaxKind::new_from_pg_query_node(&node.inner.node));
                    open_nodes.push((
                        SyntaxKind::new_from_pg_query_node(&node.inner.node),
                        node.estimated_range,
                        node.inner.depth,
                    ));
                }

                token_text.len() as i32
            } else {
                // fallback to statement token

                // move statement token lexer to before pointer
                while (statement_token_lexer.span().end as i32) < pointer {
                    statement_token_lexer.next();
                }
                let token = statement_token_lexer.next();
                if token.is_none() || (statement_token_lexer.span().start as i32) != pointer {
                    // if the token is not at the pointer, we have a syntax error
                    panic!(
                        "Expected token for '{}' at offset {}",
                        statement_token_lexer.slice(),
                        statement_token_lexer.span().start
                    );
                }
                let token_text = statement_token_lexer.slice().to_string();
                token_buffer.push_back((token.unwrap().unwrap().syntax_kind(), token_text.clone()));
                token_text.len() as i32
            };

            pointer = pointer + token_length;
        }

        while open_nodes.last().is_some() {
            self.finish_node();
            open_nodes.pop();
        }
    }
}

// impl Parser {
//     /// The main entry point for parsing a statement `text`. `at_offset` is the offset of the statement in the source file.
//     ///
//     /// On a high level, the algorithm works as follows:
//     /// 1. Parse the statement with pg_query.rs. If the statement contains syntax errors, the parser will report the error and continue to work without information
//     ///   about the nodes. The result will be a flat list of tokens under the generic `Stmt` node.
//     ///   If successful, the first node in the ordered list will be the main node of the statement,
//     ///   and serves as a root node.
//     /// 2. Scan the statements for tokens with pg_query.rs. This will never fail, even if the statement contains syntax errors.
//     /// 3. Parse the statement with the `StatementToken` lexer. The lexer only contains the tokens
//     ///    that are not parsed by pg_query.rs, such as whitespace.
//     /// 4. Define a pointer that starts at 0 and move it along the statement.
//     ///    - first, check if the current pointer is within a pg_query token. If so, consume the
//     ///    token.
//     ///    - if not, consume the next token from the `StatementToken` lexer.
//     /// 5. Close all open nodes for that statement.
//     pub fn parse_statement(&mut self, text: &str, at_offset: Option<u32>) {
//         let offset = at_offset.unwrap_or(0);
//         let range = TextRange::new(
//             TextSize::from(offset),
//             TextSize::from(offset + text.len() as u32),
//         );
//
//         let mut pg_query_tokens = match pg_query::scan(text) {
//             Ok(scanned) => scanned.tokens,
//             Err(e) => {
//                 self.error(e.to_string(), range);
//                 Vec::new()
//             }
//         };
//
//         // Get root node with depth 1
//         // Since we are parsing only a single statement there can be only a single node at depth 1
//         let pg_query_root = match pg_query::parse(text) {
//             Ok(parsed) => Some(
//                 parsed
//                     .protobuf
//                     .nodes()
//                     .iter()
//                     .find(|n| n.1 == 1)
//                     .unwrap()
//                     .0
//                     .to_enum(),
//             ),
//             Err(e) => {
//                 self.error(e.to_string(), range);
//                 None
//             }
//         };
//
//         let mut pg_query_nodes = match &pg_query_root {
//             Some(root) => resolve_tokens(
//                 &get_nodes(root, text.to_string(), 1),
//                 &pg_query_tokens,
//                 &text,
//             )
//             .into_iter()
//             .peekable(),
//             None => Vec::new().into_iter().peekable(),
//         };
//
//         let mut pg_query_tokens = pg_query_tokens.iter().peekable();
//
//         let mut lexer = StatementToken::lexer(&text);
//
//         // parse root node if no syntax errors
//         if pg_query_root.is_some() {
//             let root_node = pg_query_root.unwrap();
//             self.stmt(root_node.to_owned(), range);
//             self.start_node_at(SyntaxKind::new_from_pg_query_node(&root_node), 1);
//         } else {
//             // fallback to generic node as root
//             self.start_node_at(SyntaxKind::Stmt, 1);
//         }
//         self.set_checkpoint();
//
//         // start at 0, and increment by the length of the token
//         let mut pointer: i32 = 0;
//
//         #[derive(Debug)]
//         struct Token {
//             syntax_kind: SyntaxKind,
//             span: Span,
//         }
//
//         while pointer < text.len() as i32 {
//             // Check if the pointer is within a pg_query token
//             let next_pg_query_token = pg_query_tokens.peek();
//             let token = if next_pg_query_token.is_some()
//                 && next_pg_query_token.unwrap().start <= pointer
//                 && pointer <= next_pg_query_token.unwrap().end
//             {
//                 let token = pg_query_tokens.next().unwrap();
//                 Token {
//                     syntax_kind: SyntaxKind::new_from_pg_query_token(&token),
//                     span: Span {
//                         start: token.start as usize,
//                         end: token.end as usize,
//                     },
//                 }
//             } else {
//                 // fallback to statement token
//
//                 // move statement token lexer to before pointer
//                 while (lexer.span().end as i32) < pointer {
//                     lexer.next();
//                 }
//                 let token = lexer.next();
//                 if token.is_none() || (lexer.span().start as i32) != pointer {
//                     // if the token is not at the pointer, we have a syntax error
//                     panic!(
//                         "Expected token for '{}' at offset {}",
//                         lexer.slice(),
//                         lexer.span().start
//                     );
//                 }
//                 Token {
//                     syntax_kind: token.unwrap().unwrap().syntax_kind(),
//                     span: lexer.span(),
//                 }
//             };
//
//             self.token(
//                 token.syntax_kind,
//                 text.chars()
//                     .skip(token.span.start)
//                     .take(token.span.end - token.span.start)
//                     .collect::<String>()
//                     .as_str(),
//             );
//
//             pointer = pointer + (token.span.end - token.span.start) as i32;
//         }
//
//         // close up nodes
//         self.close_checkpoint();
//     }
// }

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    // #[test]
    // fn test_invalid_statement() {
    //     let input = "select select;";
    //
    //     let mut parser = Parser::new();
    //     parser.parse_statement(input, None);
    //     let parsed = parser.finish();
    //
    //     assert_eq!(parsed.cst.text(), input);
    // }
    //
    // #[test]
    // fn test_create_sql_function() {
    //     let input = "CREATE FUNCTION dup(in int, out f1 int, out f2 text)
    // AS $$ SELECT $1, CAST($1 AS text) || ' is text' $$
    // LANGUAGE SQL;";
    //
    //     let mut parser = Parser::new();
    //     parser.parse_statement(input, None);
    //     let parsed = parser.finish();
    //
    //     assert_eq!(parsed.cst.text(), input);
    // }
}
