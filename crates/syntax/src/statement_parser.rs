use std::iter::Peekable;
use std::slice::Iter;

use cstree::build::GreenNodeBuilder;
use cstree::text::TextRange;
use cstree::text::TextSize;
use logos::Lexer;
use logos::Logos;
use pg_query::protobuf::ScanToken;
use pg_query::NodeRef;

use crate::pg_query_utils::get_position_for_pg_query_node;
use crate::statement_builder::StatementBuilder;
use crate::statement_lexer::StatementToken;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind::{
    convert_pg_query_node_to_syntax_kind, convert_pg_query_token_to_syntax_kind, SyntaxKind,
};

pub struct StatementParser<'input, 'builder> {
    statement_lexer: Lexer<'input, StatementToken>,
    pg_query_nodes: Peekable<Iter<'input, (NodeRef<'input>, i32, pg_query::Context)>>,
    pg_query_tokens: Peekable<Iter<'input, ScanToken>>,
    builder: StatementBuilder<'builder>,
}

pub fn parse_statement<'input, 'builder>(
    input: &'input str,
    builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
) -> Result<(), SyntaxError> {
    let parsed = match pg_query::parse(input) {
        Ok(parsed) => parsed,
        Err(e) => {
            // just return error, we might be able to work with a potentially-malformed query too
            // but this is for later
            return Err(SyntaxError::new(
                e.to_string(),
                TextRange::new(TextSize::from(0), TextSize::from(input.len() as u32)),
            ));
        }
    };

    let scanned = match pg_query::scan(input) {
        Ok(scanned) => scanned,
        Err(e) => {
            // just return error, we might be able to work with a potentially-malformed query too
            // but this is for later
            return Err(SyntaxError::new(
                e.to_string(),
                TextRange::new(TextSize::from(0), TextSize::from(input.len() as u32)),
            ));
        }
    };

    let mut sorted_nodes = parsed.protobuf.nodes();

    sorted_nodes.sort_by(|a, b| {
        get_position_for_pg_query_node(&a.0).cmp(&get_position_for_pg_query_node(&b.0))
    });

    StatementParser::new(
        StatementToken::lexer(input),
        builder,
        sorted_nodes.iter().peekable(),
        scanned.tokens.iter().peekable(),
    )
    .parse();

    return Ok(());
}

// parser for individual sql expressions, e.g. a select statement
impl<'input, 'builder> StatementParser<'input, 'builder> {
    pub fn new(
        statement_lexer: Lexer<'input, StatementToken>,
        builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
        pg_query_nodes: Peekable<Iter<'input, (NodeRef<'input>, i32, pg_query::Context)>>,
        pg_query_tokens: Peekable<Iter<'input, ScanToken>>,
    ) -> Self {
        return Self {
            statement_lexer,
            pg_query_nodes,
            pg_query_tokens,
            builder: StatementBuilder::new(builder),
        };
    }

    pub fn parse(&mut self) {
        let root_node = match self.pg_query_nodes.next() {
            Some(node) => node,
            None => {
                return;
            }
        };
        let root_syntax_kind: SyntaxKind = convert_pg_query_node_to_syntax_kind(&root_node.0);
        self.builder.start_node(root_syntax_kind, &root_node.1);
        self.consume_statement_token();
        self.builder.consume_token_buffer();
        self.builder.close_until_depth(1);
    }

    fn consume_statement_token(&mut self) {
        let statement_token = self.statement_lexer.next();

        match statement_token {
            Some(token) => {
                // consume nodes at current span
                self.consume_nodes();

                // consume pg_query token at current span
                let pg_query_token = self.next_pg_query_token_in_span();

                match pg_query_token {
                    Some(t) => {
                        self.builder.token(t, self.statement_lexer.slice());
                    }
                    None => {
                        // no token found, fallback to statement_token
                        self.builder.token(
                            SyntaxKind::from_statement_token(&token.unwrap()),
                            self.statement_lexer.slice(),
                        );
                    }
                }

                // next token
                self.consume_statement_token();
            }
            None => {
                // end of input
                return;
            }
        }
    }

    fn consume_nodes(&mut self) {
        let span = self.statement_lexer.span();
        let node = self.pg_query_nodes.peek();
        match node {
            Some((node, _, _)) => {
                let pos = get_position_for_pg_query_node(node);
                if span.contains(&usize::try_from(pos).unwrap()) {
                    // node is within span
                    let (node, depth, _) = self.pg_query_nodes.next().unwrap();
                    let kind = convert_pg_query_node_to_syntax_kind(&node);
                    self.builder.start_node(kind, depth);
                    self.consume_nodes();
                }
            }
            None => {
                // no more nodes
                return;
            }
        }
    }

    fn next_pg_query_token_in_span(&mut self) -> Option<SyntaxKind> {
        let span = self.statement_lexer.span();
        let token = self.pg_query_tokens.peek();

        match token {
            Some(token) => {
                if span.contains(&usize::try_from(token.start).unwrap())
                    || span.contains(&usize::try_from(token.end).unwrap())
                {
                    // token is within span
                    let token = self.pg_query_tokens.next().unwrap();
                    return Some(convert_pg_query_token_to_syntax_kind(token));
                } else {
                    // token is not within span
                    return None;
                }
            }
            None => {
                // no more tokens
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cstree::testing::SyntaxNode;

    use super::*;

    #[test]
    fn test_statement_parser() {
        let input = "select *,test from contact where (id = '123');";

        let mut builder = GreenNodeBuilder::new();

        parse_statement(input, &mut builder).unwrap();

        let (tree, cache) = builder.finish();
        let (tree, interner) = (tree, cache.unwrap().into_interner().unwrap());
        let root = SyntaxNode::<SyntaxKind>::new_root_with_resolver(tree, interner);

        assert_eq!(format!("{:#?}", root), "SelectStmt@0..46\n  Select@0..6 \"select\"\n  Whitespace@6..7 \" \"\n  ResTarget@7..8\n    ColumnRef@7..8\n      Ascii42@7..8 \"*\"\n  Ascii44@8..9 \",\"\n  ResTarget@9..13\n    ColumnRef@9..13\n      Ident@9..13 \"test\"\n  Whitespace@13..14 \" \"\n  From@14..18 \"from\"\n  Whitespace@18..19 \" \"\n  RangeVar@19..26\n    Ident@19..26 \"contact\"\n  Whitespace@26..27 \" \"\n  Where@27..32 \"where\"\n  Whitespace@32..33 \" \"\n  Ascii40@33..34 \"(\"\n  AExpr@34..44\n    ColumnRef@34..36\n      Ident@34..36 \"id\"\n    Whitespace@36..37 \" \"\n    Ascii61@37..38 \"=\"\n    Whitespace@38..39 \" \"\n    AConst@39..44\n      Sconst@39..44 \"'123'\"\n  Ascii41@44..45 \")\"\n  Ascii59@45..46 \";\"\n");
    }
}
