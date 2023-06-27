/// A super simple lexer for sql files to work around the main weakness of pg_query.rs.
///
/// pg_query.rs only parses valid statements, and also fail to parse all statements if any contain
/// syntax errors. To circumvent this, we use a lexer to split the input into statements, and then
/// parse each statement individually.
///
/// This lexer does the split.
use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum SourceFileToken {
    // TODO: this only parses based on the semicolon, which will fail for statements that contain
    // subexperessions such as transactions or functions.
    #[regex("[a-zA-Z0-9_]+[^;]*;"gm)]
    Expr,
    #[regex("\n+"gm)]
    Newline,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

pub fn create_source_file_lexer(input: &str) -> Lexer<SourceFileToken> {
    SourceFileToken::lexer(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_lexer() {
        let input = "select * from contact where id = '123';\n\n-- test comment\n\nselect wrong statement;\n\nselect id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';\n\n";

        let mut lex = create_source_file_lexer(&input);

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Expr)));
        assert_eq!(lex.slice(), "select * from contact where id = '123';");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Comment)));
        assert_eq!(lex.slice(), "-- test comment");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Expr)));
        assert_eq!(lex.slice(), "select wrong statement;");

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Newline)));

        assert_eq!(lex.next(), Some(Ok(SourceFileToken::Expr)));
        assert_eq!(lex.slice(), "select id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';");
    }
}
