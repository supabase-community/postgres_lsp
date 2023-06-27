use std::ops::Range;

use crate::{
    event_buffer::EventBuffer, event_sink::EventSink,
    pg_query_utils::get_position_for_pg_query_node, statement_lexer::StatementToken,
    syntax_kind::SyntaxKind,
};
use logos::Logos;

pub fn parse_statement<T: EventSink>(input: &str, offset: u32, sink: &mut T) {
    let mut buffer = EventBuffer::new(sink);

    let mut pg_query_tokens = match pg_query::scan(input) {
        Ok(scanned) => scanned.tokens.into_iter().peekable(),
        Err(e) => {
            buffer.error(
                e.to_string(),
                Range {
                    start: offset as usize,
                    end: offset as usize + input.len(),
                },
            );
            Vec::new().into_iter().peekable()
        }
    };

    let parsed = pg_query::parse(input);
    let proto;
    let nodes;
    let mut pg_query_nodes = match parsed {
        Ok(parsed) => {
            proto = parsed.protobuf;
            nodes = proto.nodes();
            nodes.into_iter().peekable()
        }
        Err(e) => {
            buffer.error(
                e.to_string(),
                Range {
                    start: offset as usize,
                    end: offset as usize + input.len(),
                },
            );
            Vec::new().into_iter().peekable()
        }
    };

    let mut lexer = StatementToken::lexer(&input);

    // parse root node if no syntax errors
    if pg_query_nodes.peek().is_some() {
        let (node, depth, _) = pg_query_nodes.next().unwrap();
        buffer.start_node(SyntaxKind::from_pg_query_node(&node), &depth);
    }

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                let span = lexer.span();

                // consume pg_query nodes until there is none, or the node is outside of the current text span
                while let Some(node) = pg_query_nodes.peek() {
                    let pos = get_position_for_pg_query_node(&node.0);
                    if span.contains(&usize::try_from(pos).unwrap()) == false {
                        break;
                    } else {
                        // node is within span
                        let (node, depth, _) = pg_query_nodes.next().unwrap();
                        buffer.start_node(SyntaxKind::from_pg_query_node(&node), &depth);
                    }
                }

                // consume pg_query token if it is within the current text span
                let next_pg_query_token = pg_query_tokens.peek();
                if next_pg_query_token.is_some()
                    && (span
                        .contains(&usize::try_from(next_pg_query_token.unwrap().start).unwrap())
                        || span
                            .contains(&usize::try_from(next_pg_query_token.unwrap().end).unwrap()))
                {
                    buffer.token(
                        SyntaxKind::from_pg_query_token(&pg_query_tokens.next().unwrap()),
                        lexer.slice(),
                    );
                } else {
                    // fallback to statement token
                    buffer.token(token.syntax_kind(), lexer.slice());
                }
            }
            Err(_) => panic!("Unknown SourceFileToken: {:?}", lexer.span()),
        }
    }

    // close up nodes
    buffer.consume_token_buffer();
    buffer.close_until_depth(1);
}
