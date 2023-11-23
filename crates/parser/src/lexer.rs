use std::{collections::VecDeque, sync::LazyLock};

use pg_query::protobuf::{KeywordKind, ScanToken};
use regex::Regex;

use cstree::text::{TextRange, TextSize};

use crate::codegen::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Whitespace,
    NoKeyword,
    UnreservedKeyword,
    ColNameKeyword,
    TypeFuncNameKeyword,
    ReservedKeyword,
}

impl From<&ScanToken> for TokenType {
    fn from(token: &ScanToken) -> TokenType {
        match token.token {
            // SqlComment
            275 => TokenType::Whitespace,
            _ => match token.keyword_kind() {
                KeywordKind::NoKeyword => TokenType::NoKeyword,
                KeywordKind::UnreservedKeyword => TokenType::UnreservedKeyword,
                KeywordKind::ColNameKeyword => TokenType::ColNameKeyword,
                KeywordKind::TypeFuncNameKeyword => TokenType::TypeFuncNameKeyword,
                KeywordKind::ReservedKeyword => TokenType::ReservedKeyword,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub text: String,
    pub span: TextRange,
    pub token_type: TokenType,
}

static PATTERN_LEXER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<whitespace> )|(?P<newline>\n)|(?P<tab>\t)").unwrap());

fn whitespace_tokens(input: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();

    for cap in PATTERN_LEXER.captures_iter(&input) {
        if let Some(whitespace) = cap.name("whitespace") {
            tokens.push_back(Token {
                token_type: TokenType::Whitespace,
                kind: SyntaxKind::Whitespace,
                text: whitespace.as_str().to_string(),
                span: TextRange::new(
                    TextSize::from(u32::try_from(whitespace.start()).unwrap()),
                    TextSize::from(u32::try_from(whitespace.end()).unwrap()),
                ),
            });
        } else if let Some(newline) = cap.name("newline") {
            tokens.push_back(Token {
                token_type: TokenType::Whitespace,
                kind: SyntaxKind::Newline,
                text: newline.as_str().to_string(),
                span: TextRange::new(
                    TextSize::from(u32::try_from(newline.start()).unwrap()),
                    TextSize::from(u32::try_from(newline.end()).unwrap()),
                ),
            });
        } else if let Some(tab) = cap.name("tab") {
            tokens.push_back(Token {
                token_type: TokenType::Whitespace,
                kind: SyntaxKind::Newline,
                text: tab.as_str().to_string(),
                span: TextRange::new(
                    TextSize::from(u32::try_from(tab.start()).unwrap()),
                    TextSize::from(u32::try_from(tab.end()).unwrap()),
                ),
            });
        } else {
            panic!("No match");
        };
    }

    tokens
}

/// Turn a string of potentially valid sql code into a list of tokens, including their range in the source text.
///
/// The implementation is primarily using libpg_querys `scan` method, and fills in the gaps with tokens that are not parsed by the library, e.g. whitespace.
pub fn lex(text: &str) -> Vec<Token> {
    let mut whitespace_tokens = whitespace_tokens(text);

    // tokens from pg_query.rs
    let mut pg_query_tokens = match pg_query::scan(text) {
        Ok(scanned) => VecDeque::from(scanned.tokens),
        // this _should_ never fail
        _ => panic!("pg_query::scan failed"),
    };

    // merge the two token lists
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos = 0;

    while pos < text.len() {
        if !pg_query_tokens.is_empty() && pg_query_tokens[0].start == i32::try_from(pos).unwrap() {
            let pg_query_token = pg_query_tokens.pop_front().unwrap();
            let token_text: String = text
                .chars()
                .skip(usize::try_from(pg_query_token.start).unwrap())
                .take(
                    usize::try_from(pg_query_token.end).unwrap()
                        - usize::try_from(pg_query_token.start).unwrap(),
                )
                .collect();
            let len = token_text.len();
            let has_whitespace = token_text.contains(" ") || token_text.contains("\n");
            tokens.push(Token {
                token_type: TokenType::from(&pg_query_token),
                kind: SyntaxKind::from(&pg_query_token),
                text: token_text,
                span: TextRange::new(
                    TextSize::try_from(u32::try_from(pg_query_token.start).unwrap()).unwrap(),
                    TextSize::try_from(u32::try_from(pg_query_token.end).unwrap()).unwrap(),
                ),
            });
            pos += len;

            if has_whitespace {
                while !whitespace_tokens.is_empty()
                    && whitespace_tokens[0].span.start()
                        < TextSize::from(u32::try_from(pos).unwrap())
                {
                    whitespace_tokens.pop_front();
                }
            }

            continue;
        }

        if !whitespace_tokens.is_empty()
            && whitespace_tokens[0].span.start() == TextSize::from(u32::try_from(pos).unwrap())
        {
            let whitespace_token = whitespace_tokens.pop_front().unwrap();
            let len = whitespace_token.text.len();
            tokens.push(whitespace_token);
            pos += len;
            continue;
        }

        panic!("No token found at position {}", pos);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_lexer() {
        init();

        let input = "select 1; \n -- some comment \n select 2";

        let tokens = lex(input);
        let mut tokens_iter = tokens.iter();

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Select);
        assert_eq!(token.text, "select");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Whitespace);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Iconst);
        assert_eq!(token.text, "1");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Ascii59);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Whitespace);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Newline);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Whitespace);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::SqlComment);
        assert_eq!(token.text, "-- some comment ");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Newline);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Whitespace);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Select);
        assert_eq!(token.text, "select");

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Whitespace);

        let token = tokens_iter.next().unwrap();
        assert_eq!(token.kind, SyntaxKind::Iconst);
        assert_eq!(token.text, "2");
    }
}
