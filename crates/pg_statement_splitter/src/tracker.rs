use pg_lexer::{SyntaxKind, WHITESPACE_TOKENS};

use crate::data::{StatementDefinition, SyntaxDefinition};

#[derive(Debug)]
pub struct TokenTracker<'a> {
    pub tokens: &'a Vec<SyntaxDefinition>,

    /// position in the definition, and for each position we track the current token for that
    /// position. required for groups.
    pub positions: Vec<Position>,

    /// only for RepeatedGroup
    child: Option<Box<TokenTracker<'a>>>,
}

impl<'a> TokenTracker<'a> {
    pub fn new(tokens: &'a Vec<SyntaxDefinition>) -> Self {
        Self {
            tokens,
            positions: vec![Position::new(1)],
            child: None,
        }
    }

    pub fn advance_with(&mut self, kind: &SyntaxKind) -> bool {
        let mut new_positions = Vec::with_capacity(self.positions.len());

        for mut pos in self.positions.drain(..) {
            match self.tokens.get(pos.idx) {
                Some(SyntaxDefinition::OptionalRepeatedGroup(definitions)) => {
                    // if child does not exist, create it
                    if self.child.is_none() {
                        // check if we can spawn a new position for the next token
                        new_positions.extend(TokenTracker::next_possible_positions_from_with(
                            &self.tokens,
                            &pos,
                            kind,
                        ));
                        self.child = Some(Box::new(TokenTracker::new(definitions)));
                        new_positions.push(pos);
                    } else if self.child.as_mut().unwrap().advance_with(kind) {
                        if self.child.as_ref().unwrap().could_be_complete() {
                            new_positions.extend(TokenTracker::next_possible_positions_from_with(
                                &self.tokens,
                                &pos,
                                kind,
                            ));
                        }
                        // and advance it with the current token
                        new_positions.push(pos);
                    }
                }
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
                        new_positions.extend(TokenTracker::next_possible_positions_from_with(
                            &self.tokens,
                            &pos,
                            kind,
                        ));
                    }
                }
                Some(SyntaxDefinition::AnyTokens(maybe_tokens)) => {
                    let next_positions =
                        TokenTracker::next_possible_positions_from_with(&self.tokens, &pos, kind);

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
                        new_positions.extend(TokenTracker::next_possible_positions_from_with(
                            &self.tokens,
                            &pos,
                            kind,
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

                    // TODO the problem with removing as position when there is no token anymore is
                    // that we will return false AT the last token, since the last token does not
                    // have any following tokens. even if the statement is complete at this point
                    // and still valid until the next token is added.
                    //
                    // i think to fix this, we need to track the CURRENT positions and not all
                    // possible NEXT positions.
                }
            };
        }

        self.positions = new_positions;

        self.positions.len() != 0
    }

    fn next_possible_positions_from_with(
        tokens: &Vec<SyntaxDefinition>,
        pos: &Position,
        kind: &SyntaxKind,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        for (pos, token) in tokens.iter().enumerate().skip(pos.idx.to_owned()) {
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
                SyntaxDefinition::OptionalRepeatedGroup(t) => {
                    let first_def = t.first().unwrap();
                    match first_def {
                        SyntaxDefinition::RequiredToken(k) => {
                            if k == kind {
                                positions.push(Position::new(pos + 1));
                            }
                        }
                        SyntaxDefinition::OneOf(kinds) => {
                            if kinds.iter().any(|x| x == kind) {
                                positions.push(Position::new(pos + 1));
                            }
                        }
                        _ => {
                            panic!("OptionalRepeatedGroup must start with RequiredToken or OneOf");
                        }
                    }
                }
            }
        }

        positions
    }

    pub fn could_be_complete(&self) -> bool {
        self.tokens
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
                SyntaxDefinition::OptionalRepeatedGroup(_) => {
                    if self.child.is_none() {
                        true
                    } else {
                        self.child.as_ref().unwrap().could_be_complete()
                    }
                }
                _ => true,
            })
    }

    pub fn current_positions(&self) -> Vec<usize> {
        self.positions.iter().map(|x| x.idx).collect()
    }

    /// Returns the max idx of all tracked positions while ignoring non-required tokens
    pub fn max_pos(&self) -> usize {
        self.positions
            .iter()
            .map(|p| {
                // substract non-required tokens from the position count
                (0..p.idx).fold(0, |acc, idx| {
                    let token = self.tokens.get(idx);
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
}

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

#[derive(Debug)]
pub struct Tracker<'a> {
    pub def: &'a StatementDefinition,

    /// position in the definition, and for each position we track the current token for that
    /// position. required for groups.
    // pub positions: Vec<Position>,

    /// position in the global token stream
    pub started_at: usize,

    used_prohibited_statements: Vec<(usize, SyntaxKind)>,

    token_tracker: TokenTracker<'a>,
}

impl<'a> Tracker<'a> {
    pub fn new_at(def: &'a StatementDefinition, at: usize) -> Self {
        Self {
            def,
            // positions: vec![Position::new(1)],
            started_at: at,
            used_prohibited_statements: Vec::new(),
            token_tracker: TokenTracker::new(&def.tokens),
        }
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
            // we already used this prohibited statement, we we can start a new statement
            // but only if we are not at the same position as the prohibited statement
            // this is to prevent adding the second "VariableSetStmt" if the first was added to the
            // used list if both start at the same position
            println!("used prohibited statement: {:?}", x);
            return x.0 != at;
        }

        let res =
            self.could_be_complete() && self.def.prohibited_following_statements.contains(kind);

        if res {
            if !ignore_if_prohibited {
                println!("prohibited statement: {:?}", kind);
                self.used_prohibited_statements.push((at, kind.clone()));
            }
            return false;
        }

        true
    }

    /// Returns the max idx of all tracked positions while ignoring non-required tokens
    pub fn max_pos(&self) -> usize {
        self.token_tracker.max_pos()
    }

    pub fn current_positions(&self) -> Vec<usize> {
        self.token_tracker.current_positions()
    }

    pub fn advance_with(&mut self, kind: &SyntaxKind) -> bool {
        if WHITESPACE_TOKENS.contains(kind) {
            return true;
        }

        if self.def.prohibited_tokens.contains(kind) {
            return false;
        }

        self.token_tracker.advance_with(kind)
    }

    pub fn could_be_complete(&self) -> bool {
        self.token_tracker.could_be_complete()
    }
}
