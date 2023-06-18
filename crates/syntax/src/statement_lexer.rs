/// A super simple lexer for sql statements to work around a weakness of pg_query.rs.
///
/// pg_query.rs only parses valid statements, and no whitespaces or newlines.
/// To circumvent this, we use a very simple lexer that just knows what kind of characters are
/// being used. all words are put into the "Word" type and will be defined in more detail by the results of pg_query.rs
use logos::{Lexer, Logos};

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
    #[regex("'([^']+)'")]
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

pub fn create_statement_lexer(input: &str) -> Lexer<StatementToken> {
    StatementToken::lexer(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_lexer() {
        let input = "select * from contact where id = '123';";

        let mut lex = create_statement_lexer(&input);

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
}
