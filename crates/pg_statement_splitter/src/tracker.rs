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

    used_prohibited_statements: Vec<(usize, SyntaxKind)>,
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

    pub fn can_start_stmt_after(&mut self, kind: &SyntaxKind, at: usize) -> bool {
        if let Some(x) = self
            .used_prohibited_statements
            .iter()
            .find(|x| x.1 == *kind)
        {
            // we already used this prohibited statement, we we can start a new statement
            // but only if we are not at the same position as the prohibited statement
            // this is to prevent adding the second "VariableSetStmt" if the first was added to the
            // used list if both start at the same position
            return x.0 != at;
        }

        let res =
            self.could_be_complete() && self.def.prohibited_following_statements.contains(kind);

        if res {
            self.used_prohibited_statements.push((at, kind.clone()));
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

        println!(
            "advancing with {:?} and positions {:?}",
            kind,
            self.positions
                .iter()
                .map(|x| self.def.tokens.get(x.idx))
                .collect::<Vec<_>>()
        );

        for mut pos in self.positions.drain(..) {
            println!("advancing pos {:?}", pos);
            match self.def.tokens.get(pos.idx) {
                Some(SyntaxDefinition::RequiredToken(k)) => {
                    println!("required token {:?}", k);
                    pos.advance();
                    if k == kind {
                        new_positions.push(pos);
                    }
                }
                Some(SyntaxDefinition::AnyToken) => {
                    println!("any token");
                    pos.advance();
                    new_positions.push(pos);
                }
                Some(SyntaxDefinition::OneOf(kinds)) => {
                    println!("one of {:?}", kinds);
                    if kinds.iter().any(|x| x == kind) {
                        pos.advance();
                        new_positions.push(pos);
                    }
                }
                Some(SyntaxDefinition::OptionalToken(k)) => {
                    println!("optional token {:?}", k);
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
                    println!("any tokens {:?}", maybe_tokens);
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
                    println!("optional group {:?}", tokens);
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
                        println!("advancing group");
                        pos.advance_group();

                        // if we reached the end of the group, we advance the position
                        if pos.group_idx == tokens.len() {
                            println!("advancing pos after group");
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

        println!(
            "new positions {:?}",
            self.positions
                .iter()
                .map(|x| self.def.tokens.get(x.idx))
                .collect::<Vec<_>>()
        );

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
