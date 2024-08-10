use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};
use text_size::{TextRange, TextSize};

use crate::{
    data::{STATEMENT_BRIDGE_DEFINITIONS, STATEMENT_DEFINITIONS},
    parser::Parser,
    tracker::Tracker,
};

pub(crate) struct StatementSplitter<'a> {
    parser: Parser,
    tracked_statements: Vec<Tracker<'a>>,
    active_bridges: Vec<Tracker<'a>>,
    sub_trx_depth: usize,
    sub_stmt_depth: usize,
}

#[derive(Debug)]
pub struct StatementPosition {
    pub kind: SyntaxKind,
    pub range: TextRange,
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

    pub fn run(&mut self) -> Vec<StatementPosition> {
        let mut ranges = Vec::new();

        while !self.parser.eof() {
            let at_token = self.parser.nth(0, false);
            println!("{:?}", at_token.kind);
            println!(
                "tracked stmts before {:?}",
                self.tracked_statements
                    .iter()
                    .map(|s| s.def.stmt)
                    .collect::<Vec<_>>()
            );
            // TODO rename vars and add helpers to make distinciton between pos and text pos clear

            if at_token.kind == SyntaxKind::BeginP {
                // self.sub_trx_depth += 1;
            } else if at_token.kind == SyntaxKind::EndP {
                // self.sub_trx_depth -= 1;
            } else if at_token.kind == SyntaxKind::Ascii40 {
                // "("
                self.sub_stmt_depth += 1;
            } else if at_token.kind == SyntaxKind::Ascii41 {
                // ")"
                self.sub_stmt_depth -= 1;
            }

            let mut removed_items = Vec::new();

            self.tracked_statements.retain_mut(|stmt| {
                let keep = stmt.advance_with(&at_token.kind);
                if !keep {
                    removed_items.push(stmt.started_at);
                }
                keep
            });

            if self.tracked_statements.len() == 0 && removed_items.len() > 0 {
                let any_stmt_after = removed_items.iter().min().unwrap();
                println!("adding any statement: {:?}", any_stmt_after,);
                ranges.push(StatementPosition {
                    kind: SyntaxKind::Any,
                    range: TextRange::new(
                        TextSize::try_from(
                            self.parser
                                .tokens
                                .get(*any_stmt_after)
                                .unwrap()
                                .span
                                .start(),
                        )
                        .unwrap(),
                        TextSize::try_from(self.parser.lookbehind(2, true).unwrap().span.end())
                            .unwrap(),
                    ),
                });
            }

            println!(
                "tracked stmts after advance {:?}",
                self.tracked_statements
                    .iter()
                    .map(|s| s.def.stmt)
                    .collect::<Vec<_>>()
            );

            if self.sub_trx_depth == 0
                && self.sub_stmt_depth == 0
                    // it onyl makes sense to start tracking new statements if at least one of the
                    // currently tracked statements could be complete. or if none are tracked yet.
                    // this is important for statements such as `explain select 1;` where `select 1`
                    // would mark a completed statement that would move `explain` into completed,
                    // even though the latter is part of the former.
                && (self.tracked_statements.len() == 0
                    || self
                        .tracked_statements
                        .iter()
                        .any(|s| s.could_be_complete()))
            {
                if let Some(stmts) = STATEMENT_DEFINITIONS.get(&at_token.kind) {
                    self.tracked_statements.append(
                        &mut stmts
                            .iter()
                            .filter_map(|stmt| {
                                if self.active_bridges.iter().any(|b| b.def.stmt == stmt.stmt) {
                                    None
                                } else {
                                    Some(Tracker::new_at(stmt, self.parser.pos))
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
                        .map(|stmt| Tracker::new_at(stmt, self.parser.pos))
                        .collect(),
                );
            }

            println!(
                "tracked stmts after {:?}",
                self.tracked_statements
                    .iter()
                    .map(|s| s.def.stmt)
                    .collect::<Vec<_>>()
            );

            if at_token.kind == SyntaxKind::Ascii59 {
                // ;
                // get earliest statement
                if let Some(earliest_complete_stmt_started_at) = self
                    .tracked_statements
                    .iter()
                    .filter(|s| s.could_be_complete())
                    .min_by_key(|stmt| stmt.started_at)
                    .map(|stmt| stmt.started_at)
                {
                    let earliest_complete_stmt = self
                        .tracked_statements
                        .iter()
                        .filter(|s| {
                            s.started_at == earliest_complete_stmt_started_at
                                && s.could_be_complete()
                        })
                        .max_by_key(|stmt| stmt.current_pos)
                        .unwrap();

                    assert_eq!(
                        1,
                        self.tracked_statements
                            .iter()
                            .filter(|s| {
                                s.started_at == earliest_complete_stmt_started_at
                                    && s.could_be_complete()
                                    && s.current_pos == earliest_complete_stmt.current_pos
                            })
                            .count(),
                        "multiple complete statements at the same position"
                    );

                    let end_pos = at_token.span.end();
                    let start_pos = TextSize::try_from(
                        self.parser
                            .tokens
                            .get(earliest_complete_stmt.started_at)
                            .unwrap()
                            .span
                            .start(),
                    )
                    .unwrap();
                    println!(
                        "adding stmt from ';': {:?}",
                        earliest_complete_stmt.def.stmt
                    );
                    ranges.push(StatementPosition {
                        kind: earliest_complete_stmt.def.stmt,
                        range: TextRange::new(start_pos, end_pos),
                    });
                }

                self.tracked_statements.clear();
                self.active_bridges.clear();
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
                    .filter(|s| {
                        s.could_be_complete() && s.started_at < latest_completed_stmt_started_at
                    })
                    .max_by_key(|stmt| stmt.started_at)
                    .map(|stmt| stmt.started_at);

                if let Some(latest_complete_before_started_at) = latest_complete_before_started_at {
                    let latest_complete_before = self
                        .tracked_statements
                        .iter()
                        .filter(|s| {
                            s.started_at == latest_complete_before_started_at
                                && s.could_be_complete()
                        })
                        .max_by_key(|stmt| stmt.current_pos)
                        .cloned()
                        .unwrap();

                    assert_eq!(
                        1,
                        self.tracked_statements
                            .iter()
                            .filter(|s| {
                                s.started_at == latest_complete_before_started_at
                                    && s.could_be_complete()
                                    && s.current_pos == latest_complete_before.current_pos
                            })
                            .count(),
                        "multiple complete statements at the same position"
                    );

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
                        // .skip(latest_completed_stmt_started_at)
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

                    println!("adding stmt: {:?}", latest_complete_before.def.stmt);

                    ranges.push(StatementPosition {
                        kind: latest_complete_before.def.stmt,
                        range: TextRange::new(
                            TextSize::try_from(
                                self.parser
                                    .tokens
                                    .get(latest_complete_before.started_at)
                                    .unwrap()
                                    .span
                                    .start(),
                            )
                            .unwrap(),
                            end_pos,
                        ),
                    });

                    // Step 4: remove all statements that started before or at the position
                    self.tracked_statements
                        .retain(|s| s.started_at > latest_complete_before.started_at);
                }
            }

            self.parser.advance();
        }

        // get the earliest statement that is complete
        if let Some(earliest_complete_stmt_started_at) = self
            .tracked_statements
            .iter()
            .filter(|s| s.could_be_complete())
            .min_by_key(|stmt| stmt.started_at)
            .map(|stmt| stmt.started_at)
        {
            let earliest_complete_stmt = self
                .tracked_statements
                .iter()
                .filter(|s| {
                    s.started_at == earliest_complete_stmt_started_at && s.could_be_complete()
                })
                .max_by_key(|stmt| stmt.current_pos)
                .unwrap();

            assert_eq!(
                1,
                self.tracked_statements
                    .iter()
                    .filter(|s| {
                        s.started_at == earliest_complete_stmt_started_at
                            && s.could_be_complete()
                            && s.current_pos == earliest_complete_stmt.current_pos
                    })
                    .count(),
                "multiple complete statements at the same position"
            );

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
                .skip(earliest_complete_stmt.started_at)
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
            println!("adding stmt at end: {:?}", earliest_complete_stmt.def.stmt);
            println!("start: {:?}, end: {:?}", start_pos, end_pos);
            ranges.push(StatementPosition {
                kind: earliest_complete_stmt.def.stmt,
                range: TextRange::new(start_pos, end_pos),
            });

            self.tracked_statements
                .retain(|s| s.started_at > earliest_complete_stmt_started_at);
        }

        if let Some(earliest_stmt_started_at) = self
            .tracked_statements
            .iter()
            .min_by_key(|stmt| stmt.started_at)
            .map(|stmt| stmt.started_at)
        {
            let start_pos = TextSize::try_from(
                self.parser
                    .tokens
                    .get(earliest_stmt_started_at)
                    .unwrap()
                    .span
                    .start(),
            );
            // end position is last non-whitespace token before or at the current position
            let end_pos = TextSize::try_from(self.parser.lookbehind(1, true).unwrap().span.end());
            println!("adding any stmt at end");
            ranges.push(StatementPosition {
                kind: SyntaxKind::Any,
                range: TextRange::new(start_pos.unwrap(), end_pos.unwrap()),
            });
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use pg_lexer::SyntaxKind;

    use crate::statement_splitter::StatementSplitter;

    #[test]
    fn test_create_or_replace() {
        let input = "CREATE OR REPLACE TRIGGER check_update
    BEFORE UPDATE OF balance ON accounts
    FOR EACH ROW
    EXECUTE FUNCTION check_account_update();\nexecute test;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "CREATE OR REPLACE TRIGGER check_update\n    BEFORE UPDATE OF balance ON accounts\n    FOR EACH ROW\n    EXECUTE FUNCTION check_account_update();",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::CreateTrigStmt, result[0].kind);
        assert_eq!("execute test;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::ExecuteStmt, result[1].kind);
    }

    #[test]
    fn test_sub_statement() {
        let input = "select 1 from (select 2 from contact) c;\nselect 4;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "select 1 from (select 2 from contact) c;",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 4;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_semicolon_precedence() {
        let input = "select 1 from ;\nselect 4;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!("select 1 from ;", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 4;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_union_with_semicolon() {
        let input = "select 1 from contact union;\nselect 4;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "select 1 from contact union;",
            input[result[0].range].to_string()
        );
        assert_eq!("select 4;", input[result[1].range].to_string());
    }

    #[test]
    fn test_union() {
        let input = "select 1 from contact union select 1;\nselect 4;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "select 1 from contact union select 1;",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 4;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_splitter() {
        let input = "select 1 from contact;\nselect 1;\nselect 4;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!("select 1 from contact;", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 1;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
        assert_eq!("select 4;", input[result[2].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[2].kind);
    }

    #[test]
    fn test_no_semicolons() {
        let input = "select 1 from contact\nselect 1\nselect 4";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!("select 1 from contact", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 1", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
        assert_eq!("select 4", input[result[2].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[2].kind);
    }

    #[test]
    fn test_explain() {
        let input = "explain select 1 from contact\nselect 1\nselect 4";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!(
            "explain select 1 from contact",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::ExplainStmt, result[0].kind);
        assert_eq!("select 1", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
        assert_eq!("select 4", input[result[2].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[2].kind);
    }

    #[test]
    fn test_explain_analyze() {
        let input = "explain analyze select 1 from contact\nselect 1\nselect 4";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!(
            "explain analyze select 1 from contact",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::ExplainStmt, result[0].kind);
        assert_eq!("select 1", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
        assert_eq!("select 4", input[result[2].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[2].kind);
    }

    #[test]
    fn test_cast() {
        let input = "SELECT CAST(42 AS float8);\nselect 1";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "SELECT CAST(42 AS float8);",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("select 1", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_create_conversion() {
        let input = "CREATE CONVERSION myconv FOR 'UTF8' TO 'LATIN1' FROM myfunc;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(
            "CREATE CONVERSION myconv FOR 'UTF8' TO 'LATIN1' FROM myfunc;",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::CreateConversionStmt, result[0].kind);
    }

    #[test]
    fn test_with_comment() {
        let input = "--\n-- ADVISORY LOCKS\n--\n\nBEGIN;\n\nSELECT\n\tpg_advisory_xact_lock(1), pg_advisory_xact_lock_shared(2),\n\tpg_advisory_xact_lock(1, 1), pg_advisory_xact_lock_shared(2, 2);\n\nSELECT locktype, classid, objid, objsubid, mode, granted\n\tFROM pg_locks WHERE locktype = 'advisory'\n\tORDER BY classid, objid, objsubid;\n\n\n-- pg_advisory_unlock_all() shouldn't release xact locks\nSELECT pg_advisory_unlock_all();\n\nSELECT count(*) FROM pg_locks WHERE locktype = 'advisory';\n\n\n-- can't unlock xact locks\nSELECT\n\tpg_advisory_unlock(1), pg_advisory_unlock_shared(2),\n\tpg_advisory_unlock(1, 1), pg_advisory_unlock_shared(2, 2);\n\n\n-- automatically release xact locks at commit\nCOMMIT;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_composite_type() {
        let input = "create type avg_state as (total bigint, count bigint);\ncreate type test;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(
            "create type avg_state as (total bigint, count bigint);",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::CompositeTypeStmt, result[0].kind);
        assert_eq!("create type test;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::DefineStmt, result[1].kind);
    }

    #[test]
    fn test_set() {
        let input = "CREATE FUNCTION test_opclass_options_func(internal)
    RETURNS void
    AS :'regresslib', 'test_opclass_options_func'
    LANGUAGE C;

SET client_min_messages TO 'warning';

DROP ROLE IF EXISTS regress_alter_generic_user1;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!(
            "CREATE FUNCTION test_opclass_options_func(internal)\n    RETURNS void\n    AS :'regresslib', 'test_opclass_options_func'\n    LANGUAGE C;",
            input[result[0].range].to_string()
        );
        assert_eq!(SyntaxKind::CreateFunctionStmt, result[0].kind);
        assert_eq!(
            "SET client_min_messages TO 'warning';",
            input[result[1].range].to_string()
        );
        assert_eq!(SyntaxKind::VariableSetStmt, result[1].kind);
        assert_eq!(
            "DROP ROLE IF EXISTS regress_alter_generic_user1;",
            input[result[2].range].to_string()
        );
        assert_eq!(SyntaxKind::DropRoleStmt, result[2].kind);
    }

    #[test]
    fn test_incomplete_statement() {
        let input = "create\nselect 1;";

        let result = StatementSplitter::new(input).run();

        for r in &result {
            println!("{:?} {:?}", r.kind, input[r.range].to_string());
        }

        assert_eq!(result.len(), 2);
        assert_eq!("create", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::Any, result[0].kind);
        assert_eq!("select 1;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_incomplete_statement_at_end() {
        let input = "select 1;\ncreate";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!("select 1;", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[0].kind);
        assert_eq!("create", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::Any, result[1].kind);
    }

    #[test]
    fn test_only_incomplete_statement() {
        let input = "   create    ";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!("create", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::Any, result[0].kind);
    }
}
