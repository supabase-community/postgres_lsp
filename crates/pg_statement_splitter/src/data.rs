use pg_lexer::SyntaxKind;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
pub enum SyntaxDefinition {
    RequiredToken(SyntaxKind),
    OptionalToken(SyntaxKind),
    AnyTokens,
    AnyToken,
    OneOf(Vec<SyntaxKind>),
}

#[derive(Debug)]
pub struct StatementDefinition {
    pub stmt: SyntaxKind,
    pub tokens: Vec<SyntaxDefinition>,
}

pub static STATEMENT_BRIDGE_DEFINITIONS: LazyLock<HashMap<SyntaxKind, Vec<StatementDefinition>>> =
    LazyLock::new(|| {
        let mut m: Vec<StatementDefinition> = Vec::new();

        m.push(StatementDefinition {
            stmt: SyntaxKind::SelectStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Union),
                SyntaxDefinition::OptionalToken(SyntaxKind::All),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::SelectStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Intersect),
                SyntaxDefinition::OptionalToken(SyntaxKind::All),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::SelectStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Except),
                SyntaxDefinition::OptionalToken(SyntaxKind::All),
            ],
        });

        let mut stmt_starts: HashMap<SyntaxKind, Vec<StatementDefinition>> = HashMap::new();

        for stmt in m {
            let first_token = stmt.tokens.get(0).unwrap();
            if let SyntaxDefinition::RequiredToken(kind) = first_token {
                stmt_starts.entry(*kind).or_insert(Vec::new()).push(stmt);
            } else {
                panic!("Expected RequiredToken as first token in bridge definition");
            }
        }

        stmt_starts
    });

pub static STATEMENT_DEFINITIONS: LazyLock<HashMap<SyntaxKind, Vec<StatementDefinition>>> =
    LazyLock::new(|| {
        let mut m: Vec<StatementDefinition> = Vec::new();

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateTrigStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::OptionalToken(SyntaxKind::Constraint),
                SyntaxDefinition::RequiredToken(SyntaxKind::Trigger),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Execute),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Function, SyntaxKind::Procedure]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::SelectStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Select)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ExecuteStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Execute),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        let mut stmt_starts: HashMap<SyntaxKind, Vec<StatementDefinition>> = HashMap::new();

        for stmt in m {
            let first_token = stmt.tokens.get(0).unwrap();
            if let SyntaxDefinition::RequiredToken(kind) = first_token {
                stmt_starts.entry(*kind).or_insert(Vec::new()).push(stmt);
            } else {
                panic!("Expected RequiredToken as first token in statement definition");
            }
        }

        stmt_starts
    });
