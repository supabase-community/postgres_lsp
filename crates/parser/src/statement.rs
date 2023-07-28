use cstree::text::{TextRange, TextSize};
use logos::Logos;
use regex::Regex;

use crate::{
    parser::Parser, pg_query_utils::get_position_for_pg_query_node, syntax_kind::SyntaxKind,
};

/// A super simple lexer for sql statements.
///
/// One weakness of pg_query.rs is that it does not parse whitespace or newlines. To circumvent
/// this, we use a very simple lexer that just knows what kind of characters are being used. It
/// does not know anything about postgres syntax or keywords. For example, all words such as `select` and `from` are put into the `Word` type.
#[derive(Logos, Debug, PartialEq)]
pub enum StatementToken {
    // copied from protobuf::Token. can be generated later
    #[token("%")]
    Ascii37,
    #[token("(")]
    Ascii40,
    #[token(")")]
    Ascii41,
    #[token("*")]
    Ascii42,
    #[token("+")]
    Ascii43,
    #[token(",")]
    Ascii44,
    #[token("-")]
    Ascii45,
    #[token(".")]
    Ascii46,
    #[token("/")]
    Ascii47,
    #[token(":")]
    Ascii58,
    #[token(";")]
    Ascii59,
    #[token("<")]
    Ascii60,
    #[token("=")]
    Ascii61,
    #[token(">")]
    Ascii62,
    #[token("?")]
    Ascii63,
    #[token("[")]
    Ascii91,
    #[token("\\")]
    Ascii92,
    #[token("]")]
    Ascii93,
    #[token("^")]
    Ascii94,
    // comments, whitespaces and keywords
    // FIXME: nexted and named dollar quoted strings do not work yet
    #[regex("'([^']+)'|\\$(\\w)?\\$.*\\$(\\w)?\\$")]
    Sconst,
    #[regex("(\\w+)"gm)]
    Word,
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
            StatementToken::Ascii37 => SyntaxKind::Ascii37,
            StatementToken::Ascii40 => SyntaxKind::Ascii40,
            StatementToken::Ascii41 => SyntaxKind::Ascii41,
            StatementToken::Ascii42 => SyntaxKind::Ascii42,
            StatementToken::Ascii43 => SyntaxKind::Ascii43,
            StatementToken::Ascii44 => SyntaxKind::Ascii44,
            StatementToken::Ascii45 => SyntaxKind::Ascii45,
            StatementToken::Ascii46 => SyntaxKind::Ascii46,
            StatementToken::Ascii47 => SyntaxKind::Ascii47,
            StatementToken::Ascii58 => SyntaxKind::Ascii58,
            StatementToken::Ascii59 => SyntaxKind::Ascii59,
            StatementToken::Ascii60 => SyntaxKind::Ascii60,
            StatementToken::Ascii61 => SyntaxKind::Ascii61,
            StatementToken::Ascii62 => SyntaxKind::Ascii62,
            StatementToken::Ascii63 => SyntaxKind::Ascii63,
            StatementToken::Ascii91 => SyntaxKind::Ascii91,
            StatementToken::Ascii92 => SyntaxKind::Ascii92,
            StatementToken::Ascii93 => SyntaxKind::Ascii93,
            StatementToken::Ascii94 => SyntaxKind::Ascii94,
            StatementToken::Word => SyntaxKind::Word,
            StatementToken::Whitespace => SyntaxKind::Whitespace,
            StatementToken::Newline => SyntaxKind::Newline,
            StatementToken::Tab => SyntaxKind::Tab,
            StatementToken::Sconst => SyntaxKind::Sconst,
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

        let parsed = pg_query::parse(text);
        let proto;
        let mut nodes;
        let mut pg_query_nodes = match parsed {
            Ok(parsed) => {
                proto = parsed.protobuf;

                nodes = proto.nodes();

                nodes.sort_by(|a, b| {
                    get_position_for_pg_query_node(&a.0).cmp(&get_position_for_pg_query_node(&b.0))
                });

                nodes.into_iter().peekable()
            }
            Err(e) => {
                self.error(e.to_string(), range);
                Vec::new().into_iter().peekable()
            }
        };

        let mut lexer = StatementToken::lexer(&text);

        // parse root node if no syntax errors
        if pg_query_nodes.peek().is_some() {
            let (node, depth, _) = pg_query_nodes.next().unwrap();
            self.stmt(node.to_enum(), range);
            self.start_node_at(SyntaxKind::from_pg_query_node(&node), Some(depth));
            // if there is only one node, there are no children, and we do not need to buffer the
            // tokens. this happens for example with create or alter function statements.
            self.set_checkpoint(pg_query_nodes.peek().is_none());
        } else {
            // fallback to generic node as root
            self.start_node_at(SyntaxKind::Stmt, None);
            self.set_checkpoint(true);
        }

        // FIXME: the lexer, for some reason, does not parse dollar quoted string
        // so we check if the error is one
        while let Some(token) = lexer.next() {
            let t: Option<StatementToken> = match token {
                Ok(token) => Some(token),
                Err(_) => {
                    if Regex::new("'([^']+)'|\\$(\\w)?\\$.*\\$(\\w)?\\$")
                        .unwrap()
                        .is_match_at(lexer.slice(), 0)
                    {
                        Some(StatementToken::Sconst)
                    } else {
                        None
                    }
                }
            };

            match t {
                Some(token) => {
                    let span = lexer.span();

                    // consume pg_query nodes until there is none, or the node is outside of the current text span
                    while let Some(node) = pg_query_nodes.peek() {
                        let pos = get_position_for_pg_query_node(&node.0);
                        if span.contains(&usize::try_from(pos).unwrap()) == false {
                            break;
                        } else {
                            // node is within span
                            let (node, depth, _) = pg_query_nodes.next().unwrap();
                            self.start_node_at(SyntaxKind::from_pg_query_node(&node), Some(depth));
                        }
                    }

                    // consume pg_query token if it is within the current text span
                    let next_pg_query_token = pg_query_tokens.peek();
                    if next_pg_query_token.is_some()
                        && (span.contains(
                            &usize::try_from(next_pg_query_token.unwrap().start).unwrap(),
                        ) || span
                            .contains(&usize::try_from(next_pg_query_token.unwrap().end).unwrap()))
                    {
                        // TODO: if within function declaration and current token is Sconst, its
                        // the function body. it should be passed into parse_source_file.
                        self.token(
                            SyntaxKind::from_pg_query_token(&pg_query_tokens.next().unwrap()),
                            lexer.slice(),
                        );
                    } else {
                        // fallback to statement token
                        self.token(token.syntax_kind(), lexer.slice());
                    }
                }
                None => panic!("Unknown StatementToken: {:?}", lexer.slice()),
            }
        }

        // close up nodes
        self.close_checkpoint();
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use std::fs;
    use std::path::Path;

    use super::*;

    const VALID_STATEMENTS_PATH: &str = "test_data/statements/valid/";

    #[test]
    fn test_statement_lexer() {
        let input = "select * from contact where id = '123 4 5';";

        let mut lex = StatementToken::lexer(&input);

        assert_eq!(lex.next(), Some(Ok(StatementToken::Word)));
        assert_eq!(lex.slice(), "select");

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Ascii42)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Word)));
        assert_eq!(lex.slice(), "from");

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Word)));
        assert_eq!(lex.slice(), "contact");

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Word)));
        assert_eq!(lex.slice(), "where");

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Word)));
        assert_eq!(lex.slice(), "id");

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Ascii61)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Whitespace)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Sconst)));

        assert_eq!(lex.next(), Some(Ok(StatementToken::Ascii59)));
    }

    #[test]
    fn test_valid_statements() {
        let p = Path::new(VALID_STATEMENTS_PATH);
        println!("path: {:?}", p.display());
        fs::read_dir(VALID_STATEMENTS_PATH)
            .unwrap()
            .into_iter()
            .for_each(|f| {
                let path = f.unwrap().path();

                let contents = fs::read_to_string(&path).unwrap();

                println!("testing {}:\n'{}'", path.display(), contents);

                let mut parser = Parser::new();
                parser.parse_statement(&contents, None);
                let parsed = parser.finish();

                let fail = parsed.cst.text() != contents.as_str();

                if fail == true {
                    dbg!(&parsed.cst);
                    let parsed = pg_query::parse(contents.as_str());
                    match parsed {
                        Ok(n) => {
                            let proto = n.protobuf;
                            proto.nodes().iter().for_each(|node| {
                                println!("####");
                                println!("{:?}", node);
                            });
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                }

                assert_eq!(parsed.cst.text(), contents.as_str())
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
