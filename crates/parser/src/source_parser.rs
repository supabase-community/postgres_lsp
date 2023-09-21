use cstree::text::{TextRange, TextSize};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{parser::Parser, syntax_kind_codegen::SyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFileToken {
    Statement,
    Newline,
    Comment,
}

// Thanks to the `regex` crate, we can precompile the regular expression
lazy_static! {
    static ref PATTERN_LEXER: Regex = Regex::new(r"(?P<statement>[a-zA-Z0-9_]+(?:'[^']*'|(?:\\$\\$[^$]*\\$\\$|[^';])+)*;)|(?P<comment>/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*)|(?P<newline>\n+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: SourceFileToken,
    text: String,
    span: TextRange,
}

/// A super simple lexer for sql files that splits the input into individual statements and
/// comments.
///
/// pg_query.rs only parses valid statements, and also fail to parse all statements if any contain syntax errors.
/// To circumvent this, we use a lexer to split the input into statements, and then parse each statement individually.
///
/// This regex-based lexer does the split.
///
/// We cannot use logos because it uses `regex-syntax`, which does not support all regex syntax.
fn tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut offset = 0;

    for cap in PATTERN_LEXER.captures_iter(&input) {
        let len: u32 = if let Some(statement) = cap.name("statement") {
            let l = u32::try_from(statement.as_str().len()).unwrap();
            tokens.push(Token {
                kind: SourceFileToken::Statement,
                text: statement.as_str().to_string(),
                span: TextRange::new(TextSize::from(offset), TextSize::from(offset + l)),
            });
            l
        } else if let Some(comment) = cap.name("comment") {
            let l = u32::try_from(comment.as_str().len()).unwrap();
            tokens.push(Token {
                kind: SourceFileToken::Comment,
                text: comment.as_str().to_string(),
                span: TextRange::new(TextSize::from(offset), TextSize::from(offset + l)),
            });
            l
        } else if let Some(newline) = cap.name("newline") {
            let l = u32::try_from(newline.as_str().len()).unwrap();
            tokens.push(Token {
                kind: SourceFileToken::Newline,
                text: newline.as_str().to_string(),
                span: TextRange::new(
                    TextSize::try_from(offset).unwrap(),
                    TextSize::from(offset + l),
                ),
            });
            l
        } else {
            panic!("No match");
        };

        offset += u32::from(len);
    }

    tokens
}

impl Parser {
    /// Parse a source
    pub fn parse_source_at(&mut self, text: &str, at_offset: Option<u32>) {
        let offset = at_offset.unwrap_or(0);

        let tokens = tokens(&text);
        let mut tokens_iter = tokens.iter();

        self.start_node_at(SyntaxKind::SourceFile, 0);
        while let Some(token) = tokens_iter.next() {
            match token.kind {
                SourceFileToken::Comment => {
                    self.token(SyntaxKind::Comment, token.text.as_str());
                }
                SourceFileToken::Newline => {
                    self.token(SyntaxKind::Newline, token.text.as_str());
                }
                SourceFileToken::Statement => {
                    self.parse_statement(
                        token.text.as_str(),
                        Some(offset + u32::from(token.span.start())),
                    );
                }
            };
        }
        self.finish_node();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_file_lexer() {
        let input = "select * from contact where id = '123';\n\n-- test comment\n\nselect wrong statement;\n\nselect id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';\n\n";

        let tokens = tokens(input);
        let mut tokens_iter = tokens.iter();

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Statement);
        assert_eq!(token.text, "select * from contact where id = '123';");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Newline);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Comment);
        assert_eq!(token.text, "-- test comment");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Newline);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Statement);
        assert_eq!(token.text, "select wrong statement;");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Newline);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SourceFileToken::Statement);
        assert_eq!(token.text, "select id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';");
    }

    #[test]
    fn test_source_file_parser() {
        let input = "select id, name from users where id = '1224';

select select;





select 1;

";

        let mut parser = Parser::new();
        parser.parse_source_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }

    #[test]
    fn test_lexer_with_nested_statements() {
        let input = "select * from test;

select 123;

CREATE FUNCTION dup(in int, out f1 int, out f2 text)
    AS $$ SELECT $1, CAST($1 AS text) || ' is text;' $$
    LANGUAGE SQL;";

        let mut parser = Parser::new();
        parser.parse_source_at(input, None);
        let parsed = parser.finish();

        assert_eq!(parsed.cst.text(), input);
    }
}
