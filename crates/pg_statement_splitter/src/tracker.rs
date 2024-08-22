use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};

use crate::data::{StatementDefinition, SyntaxDefinition};

#[derive(Debug, Clone)]
pub struct Position {
    idx: usize,
    group_idx: Option<usize>,
}

impl Position {
    fn new(idx: usize) -> Self {
        Self {
            idx,
            group_idx: None,
        }
    }

    fn new_with_group(idx: usize) -> Self {
        Self {
            idx,
            group_idx: Some(1),
        }
    }

    fn start_group(&mut self) {
        self.group_idx = Some(0);
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.group_idx = None;
    }

    fn advance_group(&mut self) {
        assert!(self.group_idx.is_some());
        self.group_idx = Some(self.group_idx.unwrap() + 1);
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
}

impl<'a> Tracker<'a> {
    pub fn new_at(def: &'a StatementDefinition, at: usize) -> Self {
        Self {
            def,
            positions: vec![Position {
                idx: 1,
                group_idx: None,
            }],
            started_at: at,
        }
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
                    // the token in the group is stored in the group_idx
                    if pos.group_idx.is_none() {
                        pos.start_group();
                    }
                    let token = tokens.get(pos.group_idx.unwrap()).unwrap();
                    if token == kind {
                        pos.advance_group();

                        // if we reached the end of the group, we advance the position
                        if pos.group_idx.unwrap() == tokens.len() {
                            pos.advance();
                        }

                        new_positions.push(pos);
                    } else if pos.group_idx.unwrap() == 0 {
                        // if the first token in the group does not match, we move to the next
                        // possible tokens
                        new_positions.extend(Tracker::next_possible_positions_from_with(
                            self.def, &pos, kind,
                        ));
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
