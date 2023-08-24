use cstree::text::{TextRange, TextSize};
use log::{debug, log_enabled};
use logos::{Logos, Span};

use crate::{
    parser::Parser,
    pg_query_utils_generated::get_children,
    syntax_kind_generated::SyntaxKind,
    token_type::{
        get_token_type_from_pg_query_token, get_token_type_from_statement_token, TokenType,
    },
};

/// A super simple lexer for sql statements.
///
/// One weakness of pg_query.rs is that it does not parse whitespace or newlines. To circumvent
/// this, we use a very simple lexer that just knows what kind of characters are being used. It
/// does not know anything about postgres syntax or keywords. For example, all words such as `select` and `from` are put into the `Word` type.
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
    /// can be generated.
    pub fn syntax_kind(&self) -> SyntaxKind {
        match self {
            StatementToken::Whitespace => SyntaxKind::Whitespace,
            StatementToken::Newline => SyntaxKind::Newline,
            StatementToken::Tab => SyntaxKind::Tab,
            StatementToken::Comment => SyntaxKind::Comment,
            _ => panic!("Unknown StatementToken: {:?}", self),
        }
    }
}

impl Parser {
    /// The main entry point for parsing a statement `text`. `at_offset` is the offset of the statement in the source file.
    ///
    /// On a high level, the algorithm works as follows:
    /// 1. Parse the statement with pg_query.rs and order nodes by their position. If the
    ///   statement contains syntax errors, the parser will report the error and continue to work without information
    ///   about the nodes. The result will be a flat list of tokens under the generic `Stmt` node.
    ///   If successful, the first node in the ordered list will be the main node of the statement,
    ///   and serves as a root node.
    /// 2. Scan the statements for tokens with pg_query.rs. This will never fail, even if the statement contains syntax errors.
    /// 3. Parse the statement with the `StatementToken` lexer. The lexer will be the main vehicle
    ///    while walking the statement.
    /// 4. Walk the statement with the `StatementToken` lexer.
    ///    - at every token, consume all nodes that are within the token's range.
    ///    - if there is a pg_query token within the token's range, consume it. if not, fallback to
    ///    the StatementToken. This is the case for e.g. whitespace.
    /// 5. Close all open nodes for that statement.
    pub fn parse_statement(&mut self, text: &str, at_offset: Option<u32>) {
        let offset = at_offset.unwrap_or(0);
        let range = TextRange::new(
            TextSize::from(offset),
            TextSize::from(offset + text.len() as u32),
        );

        let mut pg_query_tokens = match pg_query::scan(text) {
            Ok(scanned) => scanned.tokens.into_iter().peekable(),
            Err(e) => {
                self.error(e.to_string(), range);
                Vec::new().into_iter().peekable()
            }
        };

        // Get root node with depth 1
        // Since we are parsing only a single statement there can be only a single node at depth 1
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

        if log_enabled!(log::Level::Debug) {
            debug!("pg_query_root: {:?}", pg_query_root);
        }

        let mut pg_query_nodes = match &pg_query_root {
            Some(root) => get_children(root, text.to_string(), 1)
                .into_iter()
                .peekable(),
            None => Vec::new().into_iter().peekable(),
        };

        if log_enabled!(log::Level::Debug) {
            println!("# Children:\n");
            &pg_query_nodes.to_owned().for_each(|n| {
                debug!("{:?}\n", n);
                // let s = text.split_at(n.location as usize);
                // debug!("Text: {:?}\npg_query_node: {:?}\n", s.1, n)
            });
        }

        let mut lexer = StatementToken::lexer(&text);

        // parse root node if no syntax errors
        if pg_query_root.is_some() {
            let root_node = pg_query_root.unwrap();
            self.stmt(root_node.to_owned(), range);
            self.start_node_at(SyntaxKind::new_from_pg_query_node(&root_node), 1);
            // if there is only one node, there are no children, and we do not need to buffer the tokens.
            self.set_checkpoint(pg_query_nodes.len() == 0);
        } else {
            // fallback to generic node as root
            self.start_node_at(SyntaxKind::Stmt, 1);
            self.set_checkpoint(true);
        }

        // start at 0, and increment by the length of the token
        let mut pointer: i32 = 0;

        #[derive(Debug)]
        struct Token {
            syntax_kind: SyntaxKind,
            span: Span,
            token_type: Option<TokenType>,
        }

        while pointer < text.len() as i32 {
            // first get token WITHOUT applying it
            // then consume pg_query nodes until there is none, or the node is outside of the current tokens' span

            // Check if the pointer is within a pg_query token
            let next_pg_query_token = pg_query_tokens.peek();
            let token = if next_pg_query_token.is_some()
                && next_pg_query_token.unwrap().start <= pointer
                && pointer <= next_pg_query_token.unwrap().end
            {
                let token = pg_query_tokens.next().unwrap();
                Token {
                    token_type: get_token_type_from_pg_query_token(&token),
                    syntax_kind: SyntaxKind::new_from_pg_query_token(&token),
                    span: Span {
                        start: token.start as usize,
                        end: token.end as usize,
                    },
                }
            } else {
                // fallback to statement token

                // move statement token lexer to before pointer
                while (lexer.span().end as i32) < pointer {
                    lexer.next();
                }
                let token = lexer.next();
                if token.is_none() || (lexer.span().start as i32) != pointer {
                    // if the token is not at the pointer, we have a syntax error
                    panic!(
                        "Expected token for '{}' at offset {}",
                        lexer.slice(),
                        lexer.span().start
                    );
                }
                Token {
                    token_type: get_token_type_from_statement_token(
                        &token.as_ref().unwrap().as_ref().unwrap(),
                    ),
                    syntax_kind: token.unwrap().unwrap().syntax_kind(),
                    span: lexer.span(),
                }
            };

            // consume pg_query nodes until there is none, or the node is outside of the current text span
            while let Some(node) = pg_query_nodes.peek() {
                if token
                    .span
                    .contains(&usize::try_from(node.location).unwrap())
                    == false
                {
                    break;
                } else {
                    // node is within span
                    let nested_node = pg_query_nodes.next().unwrap();
                    self.start_node_at(
                        SyntaxKind::new_from_pg_query_node(&nested_node.node),
                        nested_node.depth,
                    );
                }
            }

            self.token(
                token.syntax_kind,
                text.chars()
                    .skip(token.span.start)
                    .take(token.span.end - token.span.start)
                    .collect::<String>()
                    .as_str(),
                token.token_type,
            );

            pointer = pointer + (token.span.end - token.span.start) as i32;
        }

        // close up nodes
        self.close_checkpoint();
    }
}

#[cfg(test)]
mod tests {
    use log::log_enabled;
    use log::{debug, info};
    use std::assert_eq;
    use std::fs;

    use super::*;

    const VALID_STATEMENTS_PATH: &str = "test_data/statements/valid/";

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn test_valid_stmt(input: String) {
        info!("Testing: {}", input);

        let mut parser = Parser::new();
        parser.parse_statement(&input, None);
        let parsed = parser.finish();

        debug!("parsed: {}", parsed.cst.text());

        if log_enabled!(log::Level::Debug) {
            dbg!(&parsed.cst);
        }

        assert_eq!(parsed.cst.text(), input.as_str())
    }

    #[test]
    fn test_simple_statement() {
        init();
        test_valid_stmt("select 1;".to_string());
    }

    #[test]
    fn test_temp() {
        init();
        let token = pg_query::scan(
            "select coalesce(id, two) as id from contact;"
                .to_string()
                .as_str(),
        )
        .unwrap();
        dbg!(token);
    }

    #[test]
    fn test_valid_statements() {
        init();
        fs::read_dir(VALID_STATEMENTS_PATH)
            .unwrap()
            .into_iter()
            .for_each(|f| {
                let path = f.unwrap().path();

                let contents = fs::read_to_string(&path).unwrap();

                test_valid_stmt(contents);
            });
    }

    #[test]
    fn test_invalid_statement() {
        let input = "select select;";

        let mut parser = Parser::new();
        parser.parse_statement(input, None);
        let parsed = parser.finish();

        dbg!(&parsed.cst);

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_create_sql_function() {
        let input = "CREATE FUNCTION dup(in int, out f1 int, out f2 text)
    AS $$ SELECT $1, CAST($1 AS text) || ' is text' $$
    LANGUAGE SQL;";

        let mut parser = Parser::new();
        parser.parse_statement(input, None);
        let parsed = parser.finish();

        dbg!(&parsed.cst);

        assert_eq!(parsed.cst.text(), input);
    }
}
