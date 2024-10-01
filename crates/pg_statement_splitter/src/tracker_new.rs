use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};

use crate::data::{StatementDefinition, SyntaxDefinition};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementPosition {
    at: usize,

    group_idx: Option<usize>,
}

impl StatementPosition {
    pub fn new(at: usize) -> Self {
        StatementPosition {
            at,
            group_idx: None,
        }
    }

    fn new_within_group(at: usize, group_idx: usize) -> Self {
        StatementPosition {
            at,
            group_idx: Some(group_idx),
        }
    }

    fn group_idx(&self) -> usize {
        self.group_idx
            .expect("Expected position pointing to a group to have a group index")
    }
}

#[derive(Debug)]
pub struct StatementTracker<'a> {
    pub def: &'a StatementDefinition,

    /// position in the global token stream
    pub started_at: usize,

    used_prohibited_statements: Vec<(usize, SyntaxKind)>,

    positions: Vec<StatementPosition>,
}

impl<'a> StatementTracker<'a> {
    pub fn new_at(def: &'a StatementDefinition, started_at: usize) -> Self {
        StatementTracker {
            def,
            started_at,
            used_prohibited_statements: vec![],
            positions: vec![StatementPosition::new(0)],
        }
    }

    pub fn advance_with(&mut self, kind: &SyntaxKind) -> bool {
        println!("advance with ${:?}", kind);
        if WHITESPACE_TOKENS.contains(kind) {
            return true;
        }

        if self.def.prohibited_tokens.contains(kind) {
            return false;
        }

        let mut new_positions = Vec::new();

        for pos in &self.positions {
            let syntax = self.def.tokens.get(pos.at).expect("invalid position");
            match syntax {
                def @ SyntaxDefinition::OptionalRepeatedGroup(defs) => {
                    if pos.group_idx() == defs.len() - 1 {
                        // if we are at the end of a repeated group, check next positions
                        new_positions.extend(next_positions(&self.def.tokens, pos.at, kind));
                        // also check if we can restart
                        if def.first_required_tokens().iter().any(|x| x == &kind) {
                            new_positions.push(StatementPosition::new_within_group(pos.at, 0));
                        }
                    } else {
                        // if we are within a repeated group, we need to check if we can advance within
                        let next_group_positions = next_positions(&defs, pos.group_idx(), kind);

                        for next_pos in next_group_positions {
                            new_positions
                                .push(StatementPosition::new_within_group(pos.at, next_pos.at));
                        }
                    }
                }
                SyntaxDefinition::OptionalGroup(tokens) => {
                    if pos.group_idx() == tokens.len() - 1 {
                        // if we are at the end of a group, check next positions
                        new_positions.extend(next_positions(&self.def.tokens, pos.at, kind));
                    } else {
                        // if we are within a group, we need to check if we can advance within
                        if tokens[pos.group_idx() + 1] == *kind {
                            new_positions.push(StatementPosition::new_within_group(
                                pos.at,
                                pos.group_idx() + 1,
                            ));
                        }
                    }
                }
                SyntaxDefinition::AnyTokens(allowed) => {
                    let next_pos = next_positions(&self.def.tokens, pos.at, kind);

                    // if within allowed or no next position, keep position
                    if (allowed.is_some() && allowed.as_ref().unwrap().contains(kind))
                        || next_pos.is_empty()
                    {
                        new_positions.push(StatementPosition::new(pos.at));
                    }

                    // next positions
                    new_positions.extend(next_pos);
                }
                _ => {
                    new_positions.extend(next_positions(&self.def.tokens, pos.at, kind));
                }
            }
        }

        self.positions = new_positions;

        !self.positions.is_empty()
    }

    pub fn can_start_stmt_after(
        &mut self,
        kind: &SyntaxKind,
        at: usize,
        ignore_if_prohibited: bool,
    ) -> bool {
        if let Some(x) = self
            .used_prohibited_statements
            .iter()
            .find(|x| x.1 == *kind)
        {
            println!("used prohibited: {:?} at {}", x, at);
            // we already used this prohibited statement, we we can start a new statement
            // but only if we are not at the same position as the prohibited statement
            // this is to prevent adding the second "VariableSetStmt" if the first was added to the
            // used list if both start at the same position
            return x.0 != at;
        }

        let res =
            self.could_be_complete() && self.def.prohibited_following_statements.contains(kind);

        println!("prohibited: res {} for {:?} at {}", res, kind, at);
        if res {
            if !ignore_if_prohibited {
                self.used_prohibited_statements.push((at, kind.clone()));
            }
            return false;
        }

        true
    }

    pub fn current_positions(&self) -> Vec<usize> {
        self.positions.iter().map(|x| x.at).collect()
    }

    /// Returns the max idx of all tracked positions while ignoring non-required tokens
    pub fn max_pos(&self) -> usize {
        self.positions
            .iter()
            .map(|p| {
                // substract non-required tokens from the position count
                (0..p.at).fold(0, |acc, idx| {
                    let token = self.def.tokens.get(idx);
                    match token {
                        Some(SyntaxDefinition::RequiredToken(_)) => acc + 1,
                        Some(SyntaxDefinition::OneOf(_)) => acc + 1,
                        Some(SyntaxDefinition::AnyToken) => acc + 1,
                        _ => acc,
                    }
                })
            })
            .max()
            .unwrap()
    }

    pub fn could_be_complete(&self) -> bool {
        let res = self._could_be_complete();
        // println!(
        //     "{:?} could be complete: {} with {:?}",
        //     self.def.stmt, res, self.def.tokens
        // );
        res
    }

    pub fn _could_be_complete(&self) -> bool {
        let max_pos = self.positions.iter().map(|p| p.at).max().unwrap();
        // println!("tokens: {:?}", self.def.tokens);
        // println!("max pos: {} at {:?}", max_pos, self.def.tokens.get(max_pos));

        // if max pos is at group and not at last group idx, we can't be complete
        match self.def.tokens.get(max_pos) {
            Some(SyntaxDefinition::OptionalGroup(tokens)) => {
                if self
                    .positions
                    .iter()
                    .all(|x| x.group_idx() < tokens.len() - 1)
                {
                    return false;
                }
            }
            Some(SyntaxDefinition::OptionalRepeatedGroup(tokens)) => {
                if self
                    .positions
                    .iter()
                    .all(|x| x.group_idx() < tokens.len() - 1)
                {
                    return false;
                }
            }
            _ => {}
        }
        //
        // println!(
        //     "checking tokens after: {:?}",
        //     self.def.tokens.iter().skip(max_pos + 1).collect::<Vec<_>>()
        // );

        self.def.tokens.iter().skip(max_pos + 1).all(|x| match x {
            SyntaxDefinition::RequiredToken(_) => false,
            SyntaxDefinition::OneOf(_) => false,
            SyntaxDefinition::AnyToken => false,
            _ => true,
        })
    }
}

fn next_positions(
    tokens: &Vec<SyntaxDefinition>,
    pos: usize,
    kind: &SyntaxKind,
) -> Vec<StatementPosition> {
    let mut new_positions = Vec::new();

    for (pos, token) in tokens.iter().enumerate().skip(pos + 1) {
        match token {
            SyntaxDefinition::RequiredToken(k) => {
                if k == kind {
                    new_positions.push(StatementPosition::new(pos));
                }
                break;
            }
            SyntaxDefinition::OptionalToken(k) => {
                if k == kind {
                    new_positions.push(StatementPosition::new(pos));
                }
            }
            SyntaxDefinition::AnyTokens(expected) => {
                if expected.is_none() || expected.as_ref().unwrap().contains(kind) {
                    new_positions.push(StatementPosition::new(pos));
                }
            }
            SyntaxDefinition::AnyToken => {
                new_positions.push(StatementPosition::new(pos));
                break;
            }
            SyntaxDefinition::OneOf(kinds) => {
                if kinds.iter().any(|x| x == kind) {
                    new_positions.push(StatementPosition::new(pos));
                }
                break;
            }
            SyntaxDefinition::OptionalGroup(t) => {
                let first_token = t.first().unwrap();
                if first_token == kind {
                    new_positions.push(StatementPosition::new_within_group(pos, 0));
                }
            }
            def @ SyntaxDefinition::OptionalRepeatedGroup(_) => {
                if def.first_required_tokens().iter().any(|x| x == &kind) {
                    new_positions.push(StatementPosition::new_within_group(pos, 0));
                }
            }
        }
    }

    new_positions
}

#[cfg(test)]
mod tests {
    use pg_lexer::{lex, SyntaxKind, WHITESPACE_TOKENS};

    use crate::{
        data::{SyntaxDefinition, STATEMENT_DEFINITIONS},
        tracker_new::StatementPosition,
    };

    use super::StatementTracker;

    #[test]
    fn test_optional_repeated_group() {
        let input = "
WITH t1 AS (
    SELECT 1
), t2 AS (
    SELECT 2
)
SELECT 's';
            ";

        let stmt_def = STATEMENT_DEFINITIONS
            .get(&SyntaxKind::With)
            .unwrap()
            .first()
            .unwrap();

        // TODO only go to any tokens if there is no other position!
        println!("{:#?}", stmt_def.tokens);

        let lexed = lex(input);

        let tokens = lexed
            .iter()
            .filter(|x| !WHITESPACE_TOKENS.contains(&x.kind))
            .collect::<Vec<_>>();
        let mut tokens_iter = tokens.iter();

        while tokens_iter.next().unwrap().kind != SyntaxKind::With {
            // skip until WITH
        }

        let mut tracker = StatementTracker::new_at(stmt_def, 1);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 0,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 2,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 3,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 4,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 5,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 5,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 6,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 8,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 8,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 8,
                group_idx: None
            }]
        );

        tracker.advance_with(&tokens_iter.next().unwrap().kind);

        assert_eq!(
            tracker.positions,
            vec![StatementPosition {
                at: 8,
                group_idx: None
            }]
        );

        println!(
            "{:?}",
            tracker
                .positions
                .iter()
                .map(|x| stmt_def.tokens.get(x.at))
                .collect::<Vec<_>>()
        );

        // tracker.advance_with(&SyntaxKind::Ascii42);
        //
        // assert_eq!(tracker.positions.len(), 1);
        //
        // assert_eq!(
        //     tracker.positions[0],
        //     StatementPosition {
        //         at: 1,
        //         group_idx: None
        //     }
        // );
        //
        // tracker.advance_with(&SyntaxKind::Whitespace);
        //
        // assert_eq!(tracker.positions.len(), 1);
        //
        // assert_eq!(
        //     tracker.positions[0],
        //     StatementPosition {
        //         at: 1,
        //         group_idx: None
        //     }
        // );
        //
        // tracker.advance_with(&SyntaxKind::From);
        //
        // assert_eq!(tracker.positions.len(), 1);
        //
        // assert_eq!(
        //     tracker.positions[0],
        //     StatementPosition {
        //         at: 2,
        //         group_idx: None
        //     }
        // );
        //
        // tracker.advance_with(&SyntaxKind::Whitespace);
        //
        // assert_eq!(tracker.positions.len(), 1);
        //
        // assert_eq!(
        //     tracker.positions[0],
        //     StatementPosition {
        //         at: 2,
        //         group_idx: None
        //     }
        // );
        //
        // tracker.advance_with(&SyntaxKind::Ident);
        //
        // assert_eq!(tracker.positions.len(), 1);
        //
        // assert_eq!(
        //     tracker.positions[0],
        //     StatementPosition {
        //         at: 2,
        //         group_idx: None
        //     }
        // );
    }

    #[test]
    fn test_advance_with() {
        let new_stmts = STATEMENT_DEFINITIONS.get(&SyntaxKind::Select).unwrap();

        let mut tracker = StatementTracker::new_at(new_stmts.first().unwrap(), 0);

        tracker.advance_with(&SyntaxKind::Whitespace);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 0,
                group_idx: None
            }
        );

        tracker.advance_with(&SyntaxKind::Ascii42);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 1,
                group_idx: None
            }
        );

        tracker.advance_with(&SyntaxKind::Whitespace);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 1,
                group_idx: None
            }
        );

        tracker.advance_with(&SyntaxKind::From);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 2,
                group_idx: None
            }
        );

        tracker.advance_with(&SyntaxKind::Whitespace);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 2,
                group_idx: None
            }
        );

        tracker.advance_with(&SyntaxKind::Ident);

        assert_eq!(tracker.positions.len(), 1);

        assert_eq!(
            tracker.positions[0],
            StatementPosition {
                at: 2,
                group_idx: None
            }
        );
    }
}
