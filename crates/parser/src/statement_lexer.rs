/// A super simple lexer for sql statements to work around a weakness of pg_query.rs.
///
/// pg_query.rs only parses valid statements, and no whitespaces or newlines.
/// To circumvent this, we use a very simple lexer that just knows what kind of characters are
/// being used. all words are put into the "Word" type and will be defined in more detail by the results of pg_query.rs
use logos::{Lexer, Logos};

use crate::syntax_kind::SyntaxKind;

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

pub fn create_statement_lexer(input: &str) -> Lexer<StatementToken> {
    StatementToken::lexer(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_lexer() {
        let input = "select * from contact where id = '123 4 5';";

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
