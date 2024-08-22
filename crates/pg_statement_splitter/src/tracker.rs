use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};

use crate::data::{StatementDefinition, SyntaxDefinition};

#[derive(Debug, Clone)]
pub struct Position {
    idx: usize,
    group_idx: usize,
}

impl Position {
    fn new(idx: usize) -> Self {
        Self { idx, group_idx: 0 }
    }

    fn new_with_group(idx: usize) -> Self {
        Self { idx, group_idx: 1 }
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.group_idx = 0;
    }

    fn advance_group(&mut self) {
        self.group_idx += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Tracker<'a> {
    pub def: &'a StatementDefinition,

    /// position in the definition, and for each position we track the current token for that
    /// position. required for groups.
    pub positions: Vec<Position>,

    /// position in the global token stream
    pub started_at: usize,

    used_prohibited_statements: Vec<SyntaxKind>,
}

impl<'a> Tracker<'a> {
    pub fn new_at(def: &'a StatementDefinition, at: usize) -> Self {
        Self {
            def,
            positions: vec![Position::new(1)],
            started_at: at,
            used_prohibited_statements: Vec::new(),
        }
    }

    pub fn can_start_stmt_after(&mut self, kind: &SyntaxKind) -> bool {
        if self.used_prohibited_statements.contains(&kind) {
            // we already used this prohibited statement, we we can start a new statement
            return true;
        }

        let res =
            self.could_be_complete() && self.def.prohibited_following_statements.contains(kind);

        if res {
            self.used_prohibited_statements.push(kind.clone());
            return false;
        }

        true
    }

    pub fn max_pos(&self) -> usize {
        self.positions.iter().max_by_key(|p| p.idx).unwrap().idx
    }

    pub fn current_positions(&self) -> Vec<usize> {
        self.positions.iter().map(|x| x.idx).collect()
    }

    fn next_possible_positions_from_with(
        def: &StatementDefinition,
        pos: &Position,
        kind: &SyntaxKind,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        for (pos, token) in def.tokens.iter().enumerate().skip(pos.idx.to_owned()) {
            match token {
                SyntaxDefinition::RequiredToken(k) => {
                    if k == kind {
                        positions.push(Position::new(pos + 1));
                    }
                    break;
                }
                SyntaxDefinition::OptionalToken(k) => {
                    if k == kind {
                        positions.push(Position::new(pos + 1));
                    }
                }
                SyntaxDefinition::AnyTokens(_) => {
                    //
                }
                SyntaxDefinition::AnyToken => {
                    //
                }
                SyntaxDefinition::OneOf(kinds) => {
                    if kinds.iter().any(|x| x == kind) {
                        positions.push(Position::new(pos + 1));
                    }
                    break;
                }
                SyntaxDefinition::OptionalGroup(t) => {
                    let first_token = t.first().unwrap();
                    if first_token == kind {
                        positions.push(Position::new_with_group(pos + 1));
                    }
                }
            }
        }

        positions
    }

    pub fn advance_with(&mut self, kind: &SyntaxKind) -> bool {
        if WHITESPACE_TOKENS.contains(kind) {
            return true;
        }

        let mut new_positions = Vec::with_capacity(self.positions.len());

        for mut pos in self.positions.drain(..) {
            match self.def.tokens.get(pos.idx) {
                Some(SyntaxDefinition::RequiredToken(k)) => {
                    pos.advance();
                    if k == kind {
                        new_positions.push(pos);
                    }
                }
                Some(SyntaxDefinition::AnyToken) => {
                    pos.advance();
                    new_positions.push(pos);
                }
                Some(SyntaxDefinition::OneOf(kinds)) => {
                    if kinds.iter().any(|x| x == kind) {
                        pos.advance();
                        new_positions.push(pos);
                    }
                }
                Some(SyntaxDefinition::OptionalToken(k)) => {
                    if k == kind {
                        pos.advance();
                        new_positions.push(pos);
                    } else {
                        new_positions.extend(Tracker::next_possible_positions_from_with(
                            self.def, &pos, kind,
                        ));
                    }
                }
                Some(SyntaxDefinition::AnyTokens(maybe_tokens)) => {
                    let next_positions =
                        Tracker::next_possible_positions_from_with(self.def, &pos, kind);

                    if next_positions.is_empty() {
                        // we only keep the current position if we either dont care about the
                        // tokens or the token is in the list of possible tokens
                        if let Some(tokens) = maybe_tokens {
                            if tokens.iter().any(|x| x == kind) {
                                new_positions.push(pos);
                            }
                        } else {
                            new_positions.push(pos);
                        }
                    } else {
                        new_positions.extend(next_positions);
                    }
                }
                Some(SyntaxDefinition::OptionalGroup(tokens)) => {
                    if pos.group_idx == 0 {
                        // if we are at the beginning of the group, we also need to spawn new
                        // trackers for every possible next token
                        new_positions.extend(Tracker::next_possible_positions_from_with(
                            self.def, &pos, kind,
                        ));
                    }

                    // advance group
                    let token = tokens.get(pos.group_idx).unwrap();
                    if token == kind {
                        pos.advance_group();

                        // if we reached the end of the group, we advance the position
                        if pos.group_idx == tokens.len() {
                            pos.advance();
                        }

                        new_positions.push(pos);
                    }
                }
                None => {
                    // if we reached the end of the definition, we do nothing but keep the position
                    new_positions.push(pos);
                }
            };
        }

        self.positions = new_positions;

        self.positions.len() != 0
    }

    pub fn could_be_complete(&self) -> bool {
        self.def
            .tokens
            .iter()
            .skip(
                self.positions
                    .iter()
                    .max_by_key(|p| p.idx)
                    .unwrap()
                    .to_owned()
                    .idx,
            )
            .all(|x| match x {
                SyntaxDefinition::RequiredToken(_) => false,
                SyntaxDefinition::OneOf(_) => false,
                SyntaxDefinition::AnyToken => false,
                _ => true,
            })
    }
}
