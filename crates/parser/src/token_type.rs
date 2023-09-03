use pg_query::protobuf::ScanToken;

use crate::statement::StatementToken;
use crate::syntax_kind_codegen::SyntaxKind;

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

// Returns the token type of a `ScanToken` from the `pg_query` crate.
//
// converts the token to a `SyntaxKind` for better readability.
pub fn get_token_type_from_pg_query_token(token: &ScanToken) -> Option<TokenType> {
    println!("token: {:?}", token);

    let r = match token.keyword_kind() {
        pg_query::protobuf::KeywordKind::NoKeyword => {
            match SyntaxKind::new_from_pg_query_token(token) {
                SyntaxKind::Ascii37 => Some(TokenType::Follow),
                SyntaxKind::Ascii40 => Some(TokenType::Follow),
                SyntaxKind::Ascii41 => Some(TokenType::Follow),
                SyntaxKind::Ascii42 => Some(TokenType::Follow),
                SyntaxKind::Ascii43 => Some(TokenType::Follow),
                SyntaxKind::Ascii44 => Some(TokenType::Follow),
                SyntaxKind::Ascii45 => Some(TokenType::Follow),
                SyntaxKind::Ascii46 => Some(TokenType::Follow),
                SyntaxKind::Ascii47 => Some(TokenType::Follow),
                SyntaxKind::Ascii58 => Some(TokenType::Follow),
                // ";"
                SyntaxKind::Ascii59 => Some(TokenType::Close),
                SyntaxKind::Ascii60 => Some(TokenType::Follow),
                SyntaxKind::Ascii61 => Some(TokenType::Follow),
                SyntaxKind::Ascii62 => Some(TokenType::Follow),
                SyntaxKind::Ascii63 => Some(TokenType::Follow),
                SyntaxKind::Ascii92 => Some(TokenType::Follow),
                SyntaxKind::Ascii93 => Some(TokenType::Follow),
                SyntaxKind::Ascii94 => Some(TokenType::Follow),
                SyntaxKind::NotEquals => Some(TokenType::Follow),
                SyntaxKind::Sconst => Some(TokenType::Follow),
                _ => None,
            }
        }
        pg_query::protobuf::KeywordKind::UnreservedKeyword => {
            match SyntaxKind::new_from_pg_query_token(token) {
                SyntaxKind::AddP => Some(TokenType::Follow),
                SyntaxKind::Update => Some(TokenType::Follow),
                SyntaxKind::By => Some(TokenType::Follow),
                _ => None,
            }
        }
        pg_query::protobuf::KeywordKind::ColNameKeyword => None,
        pg_query::protobuf::KeywordKind::TypeFuncNameKeyword => None,
        pg_query::protobuf::KeywordKind::ReservedKeyword => {
            match SyntaxKind::new_from_pg_query_token(token) {
                SyntaxKind::And => Some(TokenType::Follow),
                SyntaxKind::Check => Some(TokenType::Follow),
                SyntaxKind::EndP => Some(TokenType::Follow),
                SyntaxKind::For => Some(TokenType::Follow),
                SyntaxKind::From => Some(TokenType::Follow),
                SyntaxKind::InP => Some(TokenType::Follow),
                SyntaxKind::On => Some(TokenType::Follow),
                SyntaxKind::Then => Some(TokenType::Follow),
                SyntaxKind::To => Some(TokenType::Follow),
                SyntaxKind::Using => Some(TokenType::Follow),
                SyntaxKind::Where => Some(TokenType::Follow),
                SyntaxKind::With => Some(TokenType::Follow),
                SyntaxKind::GroupP => Some(TokenType::Follow),
                SyntaxKind::As => Some(TokenType::Follow),
                _ => None,
            }
        }
    };
    r
}
