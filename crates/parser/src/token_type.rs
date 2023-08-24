use log::debug;
use pg_query::protobuf::ScanToken;

use crate::{statement::StatementToken, SyntaxKind};

///
///  Kind of a `SyntaxKind`
///  This is the only manual definition required for properly creating a concrete
/// syntax tree.
///  If a token is of type `Follow`, it is not immediately applied to the syntax
/// tree, but put into
///  a buffer. Before the next node is started, all buffered tokens are applied
/// to the syntax tree
///  at the depth of the node that is opened next.
///
///  For example, in `select * from contact;`, the whitespace between `*` and
/// `from` should be a direct
///  child of the `SelectStmt` node. Without this concept, it would be put into
/// the `ColumnRef`
///  node.
///
///  SelectStmt@0..22
///    Select@0..6 "select"
///    Whitespace@6..7 " "
///    ResTarget@7..8
///      ColumnRef@7..8
///        Ascii42@7..8 "*"
///    Whitespace@8..9 " "
///    From@9..13 "from"
///   Whitespace@13..14 " "
///    RangeVar@14..21
///      Ident@14..21 "contact"
///    Ascii59@21..22 ";"
#[derive(Debug)]
pub enum TokenType {
    Follow,
    Close,
}

pub fn get_token_type_from_statement_token(token: &StatementToken) -> Option<TokenType> {
    match token {
        StatementToken::Whitespace => Some(TokenType::Follow),
        StatementToken::Newline => Some(TokenType::Follow),
        _ => None,
    }
}

pub fn get_token_type_from_pg_query_token(token: &ScanToken) -> Option<TokenType> {
    let r = match token.keyword_kind() {
        pg_query::protobuf::KeywordKind::NoKeyword => match token.token {
            37 => Some(TokenType::Follow),
            40 => Some(TokenType::Follow),
            41 => Some(TokenType::Follow),
            42 => Some(TokenType::Follow),
            43 => Some(TokenType::Follow),
            44 => Some(TokenType::Follow),
            45 => Some(TokenType::Follow),
            46 => Some(TokenType::Follow),
            47 => Some(TokenType::Follow),
            58 => Some(TokenType::Follow),
            // ";"
            59 => Some(TokenType::Close),
            60 => Some(TokenType::Follow),
            61 => Some(TokenType::Follow),
            62 => Some(TokenType::Follow),
            63 => Some(TokenType::Follow),
            // 91 => Some(TokenType::Follow),
            92 => Some(TokenType::Follow),
            93 => Some(TokenType::Follow),
            94 => Some(TokenType::Follow),
            _ => None,
        },
        pg_query::protobuf::KeywordKind::UnreservedKeyword => None,
        pg_query::protobuf::KeywordKind::ColNameKeyword => None,
        pg_query::protobuf::KeywordKind::TypeFuncNameKeyword => None,
        pg_query::protobuf::KeywordKind::ReservedKeyword => match token.token {
            // End
            401 => Some(TokenType::Follow),
            // From
            429 => Some(TokenType::Follow),
            543 => Some(TokenType::Follow),
            // Then
            669 => Some(TokenType::Follow),
            673 => Some(TokenType::Follow),
            697 => Some(TokenType::Follow),
            _ => None,
        },
    };
    debug!("token: {:?}, token_type: {:?}", token, r);
    r
}
