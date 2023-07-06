/// A super simple lexer for sql files to work around the main weakness of pg_query.rs.
///
/// pg_query.rs only parses valid statements, and also fail to parse all statements if any contain
/// syntax errors. To circumvent this, we use a lexer to split the input into statements, and then
/// parse each statement individually.
///
/// This lexer does the split.
use logos::Logos;

use crate::{parser::Parser, syntax_kind::SyntaxKind};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum SourceFileToken {
    // TODO: this only parses based on the semicolon, which will fail for statements that contain
    // subexperessions such as transactions or functions.
    #[regex("[a-zA-Z0-9_]+[^;]*;"gm)]
    Statement,
    #[regex("\n+"gm)]
    Newline,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

impl Parser {
    pub fn parse_source_file(&mut self, text: &str) {
        let mut lexer = SourceFileToken::lexer(text);

        self.start_node(SyntaxKind::SourceFile, &0);
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
}
