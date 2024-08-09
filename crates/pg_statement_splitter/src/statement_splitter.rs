use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};
use text_size::{TextRange, TextSize};

use crate::{
    data::{STATEMENT_BRIDGE_DEFINITIONS, STATEMENT_DEFINITIONS},
    parser::Parser,
    statement_tracker::StatementTracker,
};

pub(crate) struct StatementSplitter<'a> {
    parser: Parser,
    tracked_statements: Vec<StatementTracker<'a>>,
    active_bridges: Vec<StatementTracker<'a>>,
    sub_trx_depth: usize,
    sub_stmt_depth: usize,
}

impl<'a> StatementSplitter<'a> {
    pub fn new(sql: &str) -> Self {
        Self {
            parser: Parser::new(pg_lexer::lex(sql)),
            tracked_statements: Vec::new(),
            active_bridges: Vec::new(),
            sub_trx_depth: 0,
            sub_stmt_depth: 0,
        }
    }

    pub fn run(&mut self) -> Vec<TextRange> {
        let mut ranges = Vec::new();

        while !self.parser.eof() {
            let at_token = self.parser.nth(0, false);
            // TODO rename vars and add helpers to make distinciton between pos and text pos clear

            if at_token.kind == SyntaxKind::BeginP {
                self.sub_trx_depth += 1;
            } else if at_token.kind == SyntaxKind::EndP {
                self.sub_trx_depth -= 1;
            } else if at_token.kind == SyntaxKind::Ascii40 {
                // "("
                self.sub_stmt_depth += 1;
            } else if at_token.kind == SyntaxKind::Ascii41 {
                // ")"
                self.sub_stmt_depth -= 1;
            }

            self.tracked_statements
                .retain_mut(|stmt| stmt.advance_with(&at_token.kind));

            if self.sub_trx_depth == 0 && self.sub_stmt_depth == 0 {
                if let Some(stmts) = STATEMENT_DEFINITIONS.get(&at_token.kind) {
                    self.tracked_statements.append(
                        &mut stmts
                            .iter()
                            .filter_map(|stmt| {
                                if self.active_bridges.iter().any(|b| b.def.stmt == stmt.stmt) {
                                    None
                                } else {
                                    Some(StatementTracker::new_at(stmt, self.parser.pos))
                                }
                            })
                            .collect(),
                    );
                };
            }

            self.active_bridges
                .retain_mut(|stmt| stmt.advance_with(&at_token.kind));

            if let Some(bridges) = STATEMENT_BRIDGE_DEFINITIONS.get(&at_token.kind) {
                self.active_bridges.append(
                    &mut bridges
                        .iter()
                        .map(|stmt| StatementTracker::new_at(stmt, self.parser.pos))
                        .collect(),
                );
            }

            // if a statement is complete, check if there are any complete statements that start
            // before the just completed one

            // Step 1: Find the latest completed statement
            let latest_completed_stmt_started_at = self
                .tracked_statements
                .iter()
                .filter(|s| s.could_be_complete())
                .max_by_key(|stmt| stmt.started_at)
                .map(|stmt| stmt.started_at);

            if let Some(latest_completed_stmt_started_at) = latest_completed_stmt_started_at {
                // Step 2: Find the latest complete statement before the latest completed statement
                let latest_complete_before_started_at = self
                    .tracked_statements
                    .iter()
                    .rev()
                    .filter(|s| {
                        s.could_be_complete() && s.started_at < latest_completed_stmt_started_at
                    })
                    .max_by_key(|stmt| stmt.started_at)
                    .map(|stmt| stmt.started_at);

                if let Some(latest_complete_before_started_at) = latest_complete_before_started_at {
                    let count = self
                        .tracked_statements
                        .iter()
                        .filter(|s| {
                            s.started_at == latest_complete_before_started_at
                                && s.could_be_complete()
                        })
                        .count();

                    assert_eq!(count, 1);

                    // Step 3: save range for the statement

                    // end is the last non-whitespace token before the start of the latest complete
                    // statement

                    // TODO optimize
                    let latest_text_pos = self
                        .parser
                        .tokens
                        .get(latest_completed_stmt_started_at)
                        .unwrap()
                        .span
                        .start();
                    let end_pos = self
                        .parser
                        .tokens
                        .iter()
                        .filter_map(|t| {
                            if t.span.start() < latest_text_pos
                                && !WHITESPACE_TOKENS.contains(&t.kind)
                            {
                                Some(t.span.end())
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap();

                    ranges.push(TextRange::new(
                        TextSize::try_from(
                            self.parser
                                .tokens
                                .get(latest_complete_before_started_at)
                                .unwrap()
                                .span
                                .start(),
                        )
                        .unwrap(),
                        end_pos,
                    ));

                    // Step 4: remove all statements that started before or at the position
                    self.tracked_statements
                        .retain(|s| s.started_at > latest_complete_before_started_at);
                }
            }

            self.parser.advance();
        }

        // get the earliest statement that is complete
        if let Some(earliest_complete_stmt) = self
            .tracked_statements
            .iter()
            .filter(|s| s.could_be_complete())
            .min_by_key(|stmt| stmt.started_at)
        {
            let earliest_text_pos = self
                .parser
                .tokens
                .get(earliest_complete_stmt.started_at)
                .unwrap()
                .span
                .start();
            let end_pos = self
                .parser
                .tokens
                .iter()
                .filter_map(|t| {
                    if t.span.start() > earliest_text_pos && !WHITESPACE_TOKENS.contains(&t.kind) {
                        Some(t.span.end())
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();
            let start_pos = TextSize::try_from(
                self.parser
                    .tokens
                    .get(earliest_complete_stmt.started_at)
                    .unwrap()
                    .span
                    .start(),
            )
            .unwrap();
            ranges.push(TextRange::new(start_pos, end_pos));
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use crate::statement_splitter::StatementSplitter;

    #[test]
    fn test_create_or_replace() {
        let input = "CREATE OR REPLACE TRIGGER check_update
    BEFORE UPDATE OF balance ON accounts
    FOR EACH ROW
    EXECUTE FUNCTION check_account_update();\nexecute test;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 2);
        assert_eq!(
            "CREATE OR REPLACE TRIGGER check_update\n    BEFORE UPDATE OF balance ON accounts\n    FOR EACH ROW\n    EXECUTE FUNCTION check_account_update();",
            input[ranges[0]].to_string()
        );
        assert_eq!("execute test;", input[ranges[1]].to_string());
    }

    #[test]
    fn test_sub_statement() {
        let input = "select 1 from (select 2 from contact) c;\nselect 4;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 2);
        assert_eq!(
            "select 1 from (select 2 from contact) c;",
            input[ranges[0]].to_string()
        );
        assert_eq!("select 4;", input[ranges[1]].to_string());
    }

    #[test]
    fn test_semicolon_precedence() {
        let input = "select 1 from ;\nselect 4;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 2);
        assert_eq!("select 1 from ;", input[ranges[0]].to_string());
        assert_eq!("select 4;", input[ranges[1]].to_string());
    }

    #[test]
    fn test_union_with_semicolon() {
        let input = "select 1 from contact union;\nselect 4;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 2);
        assert_eq!("select 1 from contact union;", input[ranges[0]].to_string());
        assert_eq!("select 4;", input[ranges[1]].to_string());
    }

    #[test]
    fn test_union() {
        let input = "select 1 from contact union select 1;\nselect 4;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 2);
        assert_eq!(
            "select 1 from contact union select 1;",
            input[ranges[0]].to_string()
        );
        assert_eq!("select 4;", input[ranges[1]].to_string());
    }

    #[test]
    fn test_splitter() {
        let input = "select 1 from contact;\nselect 1;\nselect 4;";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 3);
        assert_eq!("select 1 from contact;", input[ranges[0]].to_string());
        assert_eq!("select 1;", input[ranges[1]].to_string());
        assert_eq!("select 4;", input[ranges[2]].to_string());
    }

    #[test]
    fn test_no_semicolons() {
        let input = "select 1 from contact\nselect 1\nselect 4";

        let ranges = StatementSplitter::new(input).run();

        assert_eq!(ranges.len(), 3);
        assert_eq!("select 1 from contact", input[ranges[0]].to_string());
        assert_eq!("select 1", input[ranges[1]].to_string());
        assert_eq!("select 4", input[ranges[2]].to_string());
    }
}
