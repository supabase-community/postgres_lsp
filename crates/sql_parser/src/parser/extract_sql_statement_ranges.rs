use std::ops::Range;

use text_size::TextRange;

use crate::{codegen::SyntaxKind, lexer::lex, syntax_error::SyntaxError};

use super::{
    is_at_statement_start::{is_at_stmt_start, TokenStatement, STATEMENT_START_TOKEN_MAPS},
    Parser,
};

pub struct StatementRangesResult {
    ranges: Vec<TextRange>,
    errors: Vec<SyntaxError>,
}

pub fn extract_sql_statement_ranges(sql: &str) -> StatementRangesResult {
    let mut parser = Parser::new(lex(sql));
    parser.start_node(SyntaxKind::SourceFile);

    let mut ranges = vec![];

    while !parser.eof() {
        match is_at_stmt_start(&mut parser) {
            Some(stmt) => {
                let range = collect_statement_token_range(&mut parser, stmt);

                let from = parser.tokens.get(range.start);
                let to = parser.tokens.get(range.end - 1);
                // get text range from token range
                let start = from.unwrap().span.start();
                let end = to.unwrap().span.end();

                ranges.push(TextRange::new(
                    text_size::TextSize::from(u32::from(start)),
                    text_size::TextSize::from(u32::from(end)),
                ));

                while parser.pos < range.end {
                    parser.advance();
                }
            }
            None => {
                parser.advance();
            }
        }
    }

    parser.finish_node();

    StatementRangesResult {
        ranges,
        errors: parser.errors,
    }
}

fn collect_statement_token_range(parser: &mut Parser, kind: SyntaxKind) -> Range<usize> {
    parser.open_buffer();

    // advance with all start tokens of statement
    advance_over_start_tokens(parser, kind);

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
            SyntaxKind::As | SyntaxKind::Union | SyntaxKind::Intersect | SyntaxKind::Except => {
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
                    && is_at_stmt_start(parser).is_some()
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

    // close buffer, get tokens and reset pos
    parser.close_buffer()
}

/// advance with all start tokens of statement
fn advance_over_start_tokens(parser: &mut Parser, kind: SyntaxKind) {
    for i in 0..STATEMENT_START_TOKEN_MAPS.len() {
        parser.eat_whitespace();
        let token = parser.nth(0, false);
        if let Some(result) = STATEMENT_START_TOKEN_MAPS[i].get(&token.kind) {
            let is_in_results = result
                .iter()
                .find(|x| match x {
                    TokenStatement::EoS(y) | TokenStatement::Any(y) => y == &kind,
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
}
