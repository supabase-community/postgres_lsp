use std::collections::VecDeque;

use cstree::text::{TextRange, TextSize};
use log::debug;
use logos::Logos;

use crate::{
    estimate_node_range::estimate_node_range, get_nodes_codegen::get_nodes, parser::Parser,
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

        debug!("pg_query_root: {:#?}", pg_query_root);

        // ranged nodes from pg_query.rs, including the root node
        // the nodes are ordered by starting range, starting with the root node
        let mut pg_query_nodes = match &pg_query_root {
            Some(root) => estimate_node_range(
                &mut get_nodes(root, text.to_string(), 1),
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
        // List of (SyntaxKind, depth) to keep track of currently open sibling tokens and their depths. Latest opened is last.
        let mut open_tokens: Vec<(SyntaxKind, i32)> = Vec::new();

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
                let token_syntax_kind = SyntaxKind::new_from_pg_query_token(token);

                let token_text = text
                    .chars()
                    .skip(token.start as usize)
                    .take((token.end as usize) - (token.start as usize))
                    .collect::<String>();

                // a node can only start and end with a pg_query token, so we can handle them here

                // if closing token, close nodes until depth of opening token before applying it
                let target_depth = if token_syntax_kind.is_closing_sibling() {
                    let opening_token = open_tokens.pop().unwrap();
                    assert_eq!(
                        opening_token.0.get_closing_sibling(),
                        token_syntax_kind,
                        "Opening token {:?} does not match closing token {:?}",
                        opening_token.0,
                        token_syntax_kind
                    );
                    Some(opening_token.1)
                } else {
                    None
                };

                // before applying the token, close any node that ends before the token starts
                while open_nodes.last().is_some()
                    && open_nodes.last().unwrap().1.end() <= TextSize::from(token.start as u32)
                    && (target_depth.is_none()
                        || open_nodes.last().unwrap().2 > target_depth.unwrap())
                {
                    self.finish_node();
                    open_nodes.pop();
                }

                // drain token buffer
                for (kind, text) in token_buffer.drain(0..token_buffer.len()) {
                    self.token(kind, text.as_str());
                }

                // consume all nodes that start before the token ends
                while pg_query_nodes.peek().is_some()
                    && pg_query_nodes.peek().unwrap().range.start()
                        < TextSize::from(token.end as u32)
                {
                    let node = pg_query_nodes.next().unwrap();
                    self.start_node(SyntaxKind::new_from_pg_query_node(&node.inner.node));
                    open_nodes.push((
                        SyntaxKind::new_from_pg_query_node(&node.inner.node),
                        node.range,
                        node.inner.depth,
                    ));
                }

                // apply the token to the cst
                self.token(token_syntax_kind, token_text.as_str());
                // save the token as an opening sibling token, if it is one
                if token_syntax_kind.is_opening_sibling() {
                    open_tokens.push((token_syntax_kind, open_nodes.last().unwrap().2));
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

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_statement() {
        init();

        let input = "select 1;";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_sibling_tokens() {
        init();

        let input = "SELECT city, count(*) FILTER (WHERE temp_lo < 45), max(temp_lo) FROM weather GROUP BY city;";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_opening_token() {
        init();

        let input = "INSERT INTO weather VALUES ('San Francisco', 46, 50, 0.25, '1994-11-27');";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_closing_token_at_last_position() {
        init();

        let input = "CREATE TABLE weather (
        city      varchar(80) references cities(name),
        temp_lo   int
);";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_select_with_alias() {
        init();

        let input = "SELECT w1.temp_lo AS low, w1.temp_hi AS high FROM weather";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_select_distinct() {
        init();

        let input = "SELECT DISTINCT city
    FROM weather
    ORDER BY city;";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_order_by() {
        init();

        let input = "SELECT sum(salary) OVER w, avg(salary) OVER w
  FROM empsalary
  WINDOW w AS (PARTITION BY depname ORDER BY salary DESC);";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_fn_call() {
        init();

        let input =
            "SELECT count(*) FILTER (WHERE i < 5) AS filtered FROM generate_series(1,10) AS s(i);";

        let mut parser = Parser::new();
        parser.parse_statement_at(input, None);
        let parsed = parser.finish();

        dbg!(&parsed.cst);

        assert_eq!(parsed.cst.text(), input);
    }

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
