use logos::Logos;

use crate::{parser::Parser, syntax_kind::SyntaxKind};

/// A super simple lexer for sql files that splits the input into indivudual statements and
/// comments.
///
/// pg_query.rs only parses valid statements, and also fail to parse all statements if any contain syntax errors.
/// To circumvent this, we use a lexer to split the input into statements, and then parse each statement individually.
///
/// This regex-based lexer does the split.
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum SourceFileToken {
    #[regex("[a-zA-Z0-9_]+(?:'[^']*'|(?:\\$\\$[^$]*\\$\\$|[^';])+)*;"gm)]
    Statement,
    #[regex("\n+"gm)]
    Newline,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

impl Parser {
    /// Parse a source file
    ///
    /// TODO: rename to `parse_source_at(text: &str, at: Option<u32>)`, and allow parsing substatements, e.g. bodies of create
    /// function statements.
    pub fn parse_source_file(&mut self, text: &str) {
        let mut lexer = SourceFileToken::lexer(text);

        self.start_node_at(SyntaxKind::SourceFile, Some(0));
        while let Some(token) = lexer.next() {
            match token {
                Ok(token) => {
                    match token {
                        SourceFileToken::Comment => {
                            self.token(SyntaxKind::Comment, lexer.slice());
                        }
                        SourceFileToken::Newline => {
                            self.token(SyntaxKind::Newline, lexer.slice());
                        }
                        SourceFileToken::Statement => {
                            self.parse_statement(lexer.slice(), Some(lexer.span().start as u32));
                        }
                    };
                }
                Err(_) => panic!("Unknown SourceFileToken: {:?}", lexer.span()),
            }
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

        let mut lex = SourceFileToken::lexer(&input);

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Statement)));
        assert_eq!(lex.slice(), "select * from contact where id = '123';");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Comment)));
        assert_eq!(lex.slice(), "-- test comment");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Statement)));
        assert_eq!(lex.slice(), "select wrong statement;");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Statement)));
        assert_eq!(lex.slice(), "select id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';");
    }

    #[test]
    fn test_source_file_parser() {
        let input = "select id, name from users where id = '1224';

select select;





select 1;

";

        let mut parser = Parser::new();
        println!("input {:?}", input);
        parser.parse_source_file(input);
        let parsed = parser.finish();

        dbg!(parsed.errors);

        dbg!(&parsed.cst);

        assert_eq!(parsed.cst.text(), input);
    }
}
