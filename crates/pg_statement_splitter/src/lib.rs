///! Postgres Statement Splitter
///!
///! This crate provides a function to split a SQL source string into individual statements.
///!
///! TODO:
///! Instead of relying on statement start tokens, we need to include as many tokens as
///! possible. For example, a `CREATE TRIGGER` statement includes an `EXECUTE [ PROCEDURE |
///! FUNCTION ]` clause, but `EXECUTE` is also a statement start token for an `EXECUTE` statement.
/// We should expand the definition map to include an `Any*`, which must be followed by at least
/// one required token and allows the parser to search for the end tokens of the statement. This
/// will hopefully be enough to reduce collisions to zero.
mod is_at_stmt_start;
mod parser;
mod syntax_error;

use is_at_stmt_start::{is_at_stmt_start, TokenStatement, STATEMENT_START_TOKEN_MAPS};

use parser::{Parse, Parser};

use pg_lexer::{lex, SyntaxKind};

pub fn split(sql: &str) -> Parse {
    let mut parser = Parser::new(lex(sql));

    while !parser.eof() {
        match is_at_stmt_start(&mut parser) {
            Some(stmt) => {
                parser.start_stmt();

                // advance over all start tokens of the statement
                for i in 0..STATEMENT_START_TOKEN_MAPS.len() {
                    parser.eat_whitespace();
                    let token = parser.nth(0, false);
                    if let Some(result) = STATEMENT_START_TOKEN_MAPS[i].get(&token.kind) {
                        let is_in_results = result
                            .iter()
                            .find(|x| match x {
                                TokenStatement::EoS(y) | TokenStatement::Any(y) => y == &stmt,
                            })
                            .is_some();
                        if i == 0 && !is_in_results {
                            panic!("Expected statement start");
                        } else if is_in_results {
                            parser.expect(token.kind);
                        } else {
                            break;
                        }
                    }
                }

                // move until the end of the statement, or until the next statement start
                let mut is_sub_stmt = 0;
                let mut is_sub_trx = 0;
                let mut ignore_next_non_whitespace = false;
                while !parser.at(SyntaxKind::Ascii59) && !parser.eof() {
                    match parser.nth(0, false).kind {
                        SyntaxKind::All => {
                            // ALL is never a statement start, but needs to be skipped when combining queries
                            // (e.g. UNION ALL)
                            parser.advance();
                        }
                        SyntaxKind::BeginP => {
                            // BEGIN, consume until END
                            is_sub_trx += 1;
                            parser.advance();
                        }
                        SyntaxKind::EndP => {
                            is_sub_trx -= 1;
                            parser.advance();
                        }
                        // opening brackets "(", consume until closing bracket ")"
                        SyntaxKind::Ascii40 => {
                            is_sub_stmt += 1;
                            parser.advance();
                        }
                        SyntaxKind::Ascii41 => {
                            is_sub_stmt -= 1;
                            parser.advance();
                        }
                        SyntaxKind::As
                        | SyntaxKind::Union
                        | SyntaxKind::Intersect
                        | SyntaxKind::Except => {
                            // ignore the next non-whitespace token
                            ignore_next_non_whitespace = true;
                            parser.advance();
                        }
                        _ => {
                            // if another stmt FIRST is encountered, break
                            // ignore if parsing sub stmt
                            if ignore_next_non_whitespace == false
                                && is_sub_stmt == 0
                                && is_sub_trx == 0
                                && is_at_stmt_start(&mut parser).is_some()
                            {
                                break;
                            } else {
                                if ignore_next_non_whitespace == true && !parser.at_whitespace() {
                                    ignore_next_non_whitespace = false;
                                }
                                parser.advance();
                            }
                        }
                    }
                }

                parser.expect(SyntaxKind::Ascii59);

                parser.close_stmt();
            }
            None => {
                parser.advance();
            }
        }
    }

    parser.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitter() {
        let input = "select 1 from contact;\nselect 1;\nalter table test drop column id;";

        let res = split(input);
        assert_eq!(res.ranges.len(), 3);
        assert_eq!("select 1 from contact;", input[res.ranges[0]].to_string());
        assert_eq!("select 1;", input[res.ranges[1]].to_string());
        assert_eq!(
            "alter table test drop column id;",
            input[res.ranges[2]].to_string()
        );
    }
}
