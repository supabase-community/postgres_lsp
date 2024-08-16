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
                    println!(
                        "adding stmts: {:?}, completed are {:?}",
                        stmts.iter().map(|s| s.stmt).collect::<Vec<_>>(),
                        self.tracked_statements
                            .iter()
                            .filter(|s| s.could_be_complete())
                            .map(|s| s.def.stmt)
                            .collect::<Vec<_>>()
                    );

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
    use pg_lexer::{lex, SyntaxKind};

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
    fn test_only_incomplete_statement_semicolon() {
        let input = "create;\nselect 1;";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!("create", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::Any, result[0].kind);
        assert_eq!("select 1;", input[result[1].range].to_string());
        assert_eq!(SyntaxKind::SelectStmt, result[1].kind);
    }

    #[test]
    fn test_only_incomplete_statement() {
        let input = "   create    ";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!("create", input[result[0].range].to_string());
        assert_eq!(SyntaxKind::Any, result[0].kind);
    }

    #[test]
    fn test_reset() {
        let input = "
DROP ROLE IF EXISTS regress_alter_generic_user3;

RESET client_min_messages;

CREATE USER regress_alter_generic_user3;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!(SyntaxKind::DropRoleStmt, result[0].kind);
        assert_eq!(SyntaxKind::VariableSetStmt, result[1].kind);
        assert_eq!(SyntaxKind::CreateRoleStmt, result[2].kind);
    }

    #[test]
    fn test_grant_and_set_session_auth() {
        let input = "
CREATE SCHEMA alt_nsp2;

GRANT ALL ON SCHEMA alt_nsp1, alt_nsp2 TO public;

SET search_path = alt_nsp1, public;

SET SESSION AUTHORIZATION regress_alter_generic_user1;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 4);
        assert_eq!(SyntaxKind::CreateSchemaStmt, result[0].kind);
        assert_eq!(SyntaxKind::GrantStmt, result[1].kind);
        assert_eq!(SyntaxKind::VariableSetStmt, result[2].kind);
        assert_eq!(SyntaxKind::VariableSetStmt, result[3].kind);
    }

    #[test]
    fn test_create_fn_and_agg() {
        let input = "
CREATE FUNCTION alt_func1(int) RETURNS int LANGUAGE sql
  AS 'SELECT $1 + 1';
CREATE FUNCTION alt_func2(int) RETURNS int LANGUAGE sql
  AS 'SELECT $1 - 1';
CREATE AGGREGATE alt_agg1 (
  sfunc1 = int4pl, basetype = int4, stype1 = int4, initcond = 0
);
CREATE AGGREGATE alt_agg2 (
  sfunc1 = int4mi, basetype = int4, stype1 = int4, initcond = 0
);
";
        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 4);
        assert_eq!(SyntaxKind::CreateFunctionStmt, result[0].kind);
        assert_eq!(SyntaxKind::CreateFunctionStmt, result[1].kind);
        assert_eq!(SyntaxKind::DefineStmt, result[2].kind);
        assert_eq!(SyntaxKind::DefineStmt, result[3].kind);
    }

    #[test]
    fn test_create_alter_agg() {
        let input = "
CREATE AGGREGATE alt_agg2 (
  sfunc1 = int4mi, basetype = int4, stype1 = int4, initcond = 0
);
ALTER AGGREGATE alt_func1(int) RENAME TO alt_func3;
ALTER AGGREGATE alt_func1(int) OWNER TO regress_alter_generic_user3;
ALTER AGGREGATE alt_func1(int) SET SCHEMA alt_nsp2;
";
        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 4);
        assert_eq!(SyntaxKind::DefineStmt, result[0].kind);
        assert_eq!(SyntaxKind::RenameStmt, result[1].kind);
        assert_eq!(SyntaxKind::AlterOwnerStmt, result[2].kind);
        assert_eq!(SyntaxKind::AlterObjectSchemaStmt, result[3].kind);
    }

    #[test]
    fn test_reset_session() {
        let input = "
ALTER AGGREGATE alt_agg2(int) SET SCHEMA alt_nsp2;

RESET SESSION AUTHORIZATION;
";
        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(SyntaxKind::AlterObjectSchemaStmt, result[0].kind);
        assert_eq!(SyntaxKind::VariableSetStmt, result[1].kind);
    }

    #[test]
    fn test_rename_fdw() {
        let input = "
CREATE SERVER alt_fserv2 FOREIGN DATA WRAPPER alt_fdw2;

ALTER FOREIGN DATA WRAPPER alt_fdw1 RENAME TO alt_fdw2;
ALTER FOREIGN DATA WRAPPER alt_fdw1 RENAME TO alt_fdw3;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 3);
        assert_eq!(SyntaxKind::CreateForeignServerStmt, result[0].kind);
        assert_eq!(SyntaxKind::RenameStmt, result[1].kind);
        assert_eq!(SyntaxKind::RenameStmt, result[2].kind);
    }

    #[test]
    fn test_ops() {
        let input = "
ALTER OPERATOR FAMILY alt_opf4 USING btree DROP
  -- int4 vs int2
  OPERATOR 1 (int4, int2) ,
  OPERATOR 2 (int4, int2) ,
  OPERATOR 3 (int4, int2) ,
  OPERATOR 4 (int4, int2) ,
  OPERATOR 5 (int4, int2) ,
  FUNCTION 1 (int4, int2) ;
DROP OPERATOR FAMILY alt_opf4 USING btree;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(SyntaxKind::AlterOpFamilyStmt, result[0].kind);
        assert_eq!(SyntaxKind::DropStmt, result[1].kind);
    }

    #[test]
    fn test_temp_table() {
        let input = "
CREATE TEMP TABLE foo (f1 int, f2 int, f3 int, f4 int);

CREATE INDEX fooindex ON foo (f1 desc, f2 asc, f3 nulls first, f4 nulls last);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(SyntaxKind::CreateStmt, result[0].kind);
        assert_eq!(SyntaxKind::IndexStmt, result[1].kind);
    }

    #[test]
    fn test_create_table_as() {
        let input = "
CREATE TEMP TABLE point_tbl AS SELECT * FROM public.point_tbl;
";
        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::CreateTableAsStmt, result[0].kind);
    }

    #[test]
    fn test_analyze() {
        let input = "
ANALYZE array_op_test;
INSERT INTO arrtest (a[1:5], b[1:1][1:2][1:2], c, d, f, g)
   VALUES ('{1,2,3,4,5}', '{{{0,0},{1,2}}}', '{}', '{}', '{}', '{}');
";
        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(SyntaxKind::VacuumStmt, result[0].kind);
        assert_eq!(SyntaxKind::InsertStmt, result[1].kind);
    }

    #[test]
    fn test_drop_operator() {
        let input = "
DROP OPERATOR === (boolean, boolean);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DropStmt, result[0].kind);
    }

    #[test]
    fn test_language() {
        let input = "
CREATE LANGUAGE alt_lang1 HANDLER plpgsql_call_handler;
CREATE LANGUAGE alt_lang2 HANDLER plpgsql_call_handler;

ALTER LANGUAGE alt_lang1 OWNER TO regress_alter_generic_user1;
ALTER LANGUAGE alt_lang2 OWNER TO regress_alter_generic_user2;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 4);
        assert_eq!(SyntaxKind::CreatePlangStmt, result[0].kind);
        assert_eq!(SyntaxKind::CreatePlangStmt, result[1].kind);
        assert_eq!(SyntaxKind::AlterOwnerStmt, result[2].kind);
        assert_eq!(SyntaxKind::AlterOwnerStmt, result[3].kind);
    }

    #[test]
    fn test_alter_op_family() {
        let input = "
ALTER OPERATOR FAMILY alt_opf1 USING hash OWNER TO regress_alter_generic_user1;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::AlterOwnerStmt, result[0].kind);
    }

    #[test]
    fn test_drop_op_family() {
        let input = "
DROP OPERATOR FAMILY alt_opf4 USING btree;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DropStmt, result[0].kind);
    }

    #[test]
    fn test_set_role() {
        let input = "
SET ROLE regress_alter_generic_user5;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::VariableSetStmt, result[0].kind);
    }

    #[test]
    fn test_revoke() {
        let input = "
CREATE ROLE regress_alter_generic_user6;
CREATE SCHEMA alt_nsp6;
REVOKE ALL ON SCHEMA alt_nsp6 FROM regress_alter_generic_user6;
CREATE OPERATOR FAMILY alt_nsp6.alt_opf6 USING btree;
SET ROLE regress_alter_generic_user6;
ALTER OPERATOR FAMILY alt_nsp6.alt_opf6 USING btree ADD OPERATOR 1 < (int4, int2);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 6);
        assert_eq!(SyntaxKind::CreateRoleStmt, result[0].kind);
        assert_eq!(SyntaxKind::CreateSchemaStmt, result[1].kind);
        assert_eq!(SyntaxKind::GrantStmt, result[2].kind);
        assert_eq!(SyntaxKind::CreateOpFamilyStmt, result[3].kind);
        assert_eq!(SyntaxKind::VariableSetStmt, result[4].kind);
        assert_eq!(SyntaxKind::AlterOpFamilyStmt, result[5].kind);
    }

    #[test]
    fn test_alter_op_family_2() {
        let input = "
CREATE OPERATOR FAMILY alt_opf4 USING btree;
ALTER OPERATOR FAMILY schema.alt_opf4 USING btree ADD
  -- int4 vs int2
  OPERATOR 1 < (int4, int2) ,
  OPERATOR 2 <= (int4, int2) ,
  OPERATOR 3 = (int4, int2) ,
  OPERATOR 4 >= (int4, int2) ,
  OPERATOR 5 > (int4, int2) ,
  FUNCTION 1 btint42cmp(int4, int2);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 2);
        assert_eq!(SyntaxKind::CreateOpFamilyStmt, result[0].kind);
        assert_eq!(SyntaxKind::AlterOpFamilyStmt, result[1].kind);
    }

    #[test]
    fn test_create_stat() {
        let input = "
CREATE STATISTICS alt_stat1 ON a, b FROM alt_regress_1;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::CreateStatsStmt, result[0].kind);
    }

    #[test]
    fn test_create_text_search_dictionary() {
        let input = "
CREATE TEXT SEARCH DICTIONARY alt_ts_dict1 (template=simple);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DefineStmt, result[0].kind);
    }

    #[test]
    fn test_create_text_search_configuration() {
        let input = "
CREATE TEXT SEARCH CONFIGURATION alt_ts_conf1 (copy=english);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DefineStmt, result[0].kind);
    }

    #[test]
    fn test_alter_operator() {
        let input = "
ALTER OPERATOR === (boolean, boolean) SET (RESTRICT = NONE);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::AlterOperatorStmt, result[0].kind);
    }

    #[test]
    fn test_drop_fdw() {
        let input = "
DROP FOREIGN DATA WRAPPER alt_fdw2 CASCADE;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DropStmt, result[0].kind);
    }

    #[test]
    fn test_insert_select() {
        let input = "
insert into src select string_agg(random()::text,'') from generate_series(1,10000);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::InsertStmt, result[0].kind);
    }

    #[test]
    fn test_on_conflict() {
        let input = "
insert into arr_pk_tbl values (1, '{3,4,5}') on conflict (pk)\n  do update set f1[1] = excluded.f1[1], f1[3] = excluded.f1[3]\n  returning pk, f1;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::InsertStmt, result[0].kind);
    }

    #[test]
    fn test_alter_index() {
        let input = "
ALTER INDEX btree_tall_idx2 ALTER COLUMN id SET (n_distinct=100);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::AlterTableStmt, result[0].kind);
    }

    #[test]
    fn test_update_set() {
        let input = "
UPDATE CASE_TBL\n  SET i = CASE WHEN i >= 3 THEN (- i)\n                ELSE (2 * i) END;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::UpdateStmt, result[0].kind);
    }

    #[test]
    fn test_savepoint() {
        let input = "
SAVEPOINT s1;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::TransactionStmt, result[0].kind);
    }

    #[test]
    fn test_declare_cursor() {
        let input = "
DECLARE c CURSOR FOR SELECT ctid,cmin,* FROM combocidtest;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::DeclareCursorStmt, result[0].kind);
    }

    #[test]
    fn test_create_empty_table() {
        let input = "
CREATE TABLE IF NOT EXISTS testcase(
);
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::CreateStmt, result[0].kind);
    }

    #[test]
    fn test_rollback_to() {
        let input = "
ROLLBACK TO SAVEPOINT subxact;
";

        let result = StatementSplitter::new(input).run();

        assert_eq!(result.len(), 1);
        assert_eq!(SyntaxKind::TransactionStmt, result[0].kind);
    }

    #[allow(clippy::must_use)]
    fn debug(input: &str) {
        for s in input.split(';').filter_map(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim())
            }
        }) {
            println!("Statement: '{:?}'", s);

            let res = pg_query::parse(s)
                .map(|parsed| {
                    parsed
                        .protobuf
                        .nodes()
                        .iter()
                        .find(|n| n.1 == 1)
                        .unwrap()
                        .0
                        .to_enum()
                })
                .unwrap();
            println!("Result: {:?}", res);
        }

        let result = StatementSplitter::new(input).run();

        for r in &result {
            println!("{:?} {:?}", r.kind, input[r.range].to_string());
        }

        assert!(false);
    }
}
