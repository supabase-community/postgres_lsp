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

// in some edge cases such as create rule ... do also delete ... its close to
// impossible to make sure the delete statement is part of the create rule
// statement. this is why we only start new statements if the previous token
// was not one of a fixed set of tokens that can only be part of a statement
//
// FIXME: this is a workaround for the current limitations of the parser
// FIXME2: find a better name :D
pub const SPECIAL_TOKENS: [SyntaxKind; 4] = [
    SyntaxKind::Do,
    SyntaxKind::Also,
    SyntaxKind::Instead,
    SyntaxKind::As,
];

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
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Select),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::InsertStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Insert),
                SyntaxDefinition::RequiredToken(SyntaxKind::Into),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                // the minimum required tokens for an insert statement are DEFAULT VALUES
                // this is important to not conflict with a SELECT statement
                // when within an insert into table select ...
                SyntaxDefinition::AnyToken,
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DeleteStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::DeleteP),
                SyntaxDefinition::RequiredToken(SyntaxKind::From),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::UpdateStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Update),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::MergeStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Merge),
                SyntaxDefinition::RequiredToken(SyntaxKind::Into),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterTableStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::OptionalToken(SyntaxKind::Materialized),
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::Table,
                    SyntaxKind::Index,
                    SyntaxKind::View,
                ]),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::RenameStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Rename),
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::RenameStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Rename),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDomainStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::DomainP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CallStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Call),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Ident, SyntaxKind::VersionP]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDefaultPrivilegesStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Default),
                SyntaxDefinition::RequiredToken(SyntaxKind::Privileges),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ClusterStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Cluster)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CopyStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Copy)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ExecuteStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Execute),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        // TODO we might need to add new types to handle this properly
        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Global),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::OptionalToken(SyntaxKind::Unlogged),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::RequiredToken(SyntaxKind::Aggregate),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateOpClassStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::RequiredToken(SyntaxKind::Class),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::OptionalToken(SyntaxKind::Default),
                SyntaxDefinition::RequiredToken(SyntaxKind::For),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Using),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::RequiredToken(SyntaxKind::Class),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Using),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Access),
                SyntaxDefinition::RequiredToken(SyntaxKind::Method),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Trigger),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Collation),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::ConversionP),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Parser),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Dictionary),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Template),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Configuration),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Extension),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Aggregate),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::DomainP),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sequence),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Cast),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::DataP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Wrapper),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Index),
                SyntaxDefinition::OptionalToken(SyntaxKind::Concurrently),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Rule),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Op, SyntaxKind::Ident]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Routine),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Procedure),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Function),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Schema),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::View),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Language),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::OptionalToken(SyntaxKind::Procedural),
                SyntaxDefinition::RequiredToken(SyntaxKind::Language),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Cascade, SyntaxKind::Restrict]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::RequiredToken(SyntaxKind::Family),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Using),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        // CREATE TEXT SEARCH DICTIONARY alt_ts_dict1 (template=simple);
        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Dictionary),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Configuration),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Template),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TextP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Search),
                SyntaxDefinition::RequiredToken(SyntaxKind::Parser),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::RequiredToken(SyntaxKind::Aggregate),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CompositeTypeStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateEnumStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::RequiredToken(SyntaxKind::EnumP),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateRangeStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::RequiredToken(SyntaxKind::Range),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TruncateStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Truncate),
                SyntaxDefinition::OptionalToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CommentStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Comment),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::FetchStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Fetch),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VacuumStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Analyze)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::IndexStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Unique),
                SyntaxDefinition::RequiredToken(SyntaxKind::Index),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateFunctionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Function, SyntaxKind::Procedure]),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterFunctionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Function, SyntaxKind::Procedure]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DoStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Do),
                SyntaxDefinition::OptionalToken(SyntaxKind::Language),
                SyntaxDefinition::OptionalToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sconst),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::RuleStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::RequiredToken(SyntaxKind::Rule),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::Select,
                    SyntaxKind::Insert,
                    SyntaxKind::Update,
                    SyntaxKind::DeleteP,
                ]),
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Do),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::NotifyStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Notify),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ListenStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Listen),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::UnlistenStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Unlisten),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Ident, SyntaxKind::Ascii42]),
            ],
        });

        // DECLARE c CURSOR FOR SELECT ctid,cmin,* FROM combocidtest
        m.push(StatementDefinition {
            stmt: SyntaxKind::DeclareCursorStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Declare),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Cursor),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::For),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Select, SyntaxKind::With]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Savepoint),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::BeginP),
                // FIXME: without the ";", this would conflict with BEGIN ATOMIC
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii59),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::BeginP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Transaction),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Commit)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Rollback),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
                SyntaxDefinition::OptionalToken(SyntaxKind::Savepoint),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Rollback),
                // FIXME: without the ";", this would conflict with ROLLBACK TO SAVEPOINT
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii59),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ViewStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::OptionalToken(SyntaxKind::Recursive),
                SyntaxDefinition::RequiredToken(SyntaxKind::View),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::OneOf(vec![SyntaxKind::With, SyntaxKind::Select]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::LoadStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Load)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateDomainStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::DomainP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreatedbStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropdbStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VacuumStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Vacuum)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateTableAsStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Global),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::RequiredToken(SyntaxKind::Materialized),
                SyntaxDefinition::RequiredToken(SyntaxKind::View),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::OneOf(vec![SyntaxKind::With, SyntaxKind::Select]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateTableAsStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Global),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::OneOf(vec![SyntaxKind::With, SyntaxKind::Select]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ViewStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::OptionalToken(SyntaxKind::Recursive),
                SyntaxDefinition::RequiredToken(SyntaxKind::View),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::OneOf(vec![SyntaxKind::With, SyntaxKind::Select]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ExplainStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Explain),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::With,
                    SyntaxKind::Select,
                    SyntaxKind::Insert,
                    SyntaxKind::DeleteP,
                    SyntaxKind::Update,
                    SyntaxKind::Merge,
                    SyntaxKind::Execute,
                ]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateSeqStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temporary),
                SyntaxDefinition::OptionalToken(SyntaxKind::Temp),
                SyntaxDefinition::OptionalToken(SyntaxKind::Unlogged),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sequence),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterSeqStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sequence),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        // RESET SESSION AUTHORIZATION
        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Reset),
                SyntaxDefinition::RequiredToken(SyntaxKind::Session),
                SyntaxDefinition::RequiredToken(SyntaxKind::Authorization),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::RequiredToken(SyntaxKind::Role),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Reset),
                SyntaxDefinition::RequiredToken(SyntaxKind::Role),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Reset),
                SyntaxDefinition::OneOf(vec![SyntaxKind::All, SyntaxKind::Ident]),
            ],
        });

        // ref: https://www.postgresql.org/docs/current/sql-set-session-authorization.html
        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::RequiredToken(SyntaxKind::Session),
                SyntaxDefinition::RequiredToken(SyntaxKind::Authorization),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Ident, SyntaxKind::Sconst]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::OptionalToken(SyntaxKind::Session),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::OneOf(vec![SyntaxKind::To, SyntaxKind::Ascii61]),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::OptionalToken(SyntaxKind::Session),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
                SyntaxDefinition::RequiredToken(SyntaxKind::Time),
                SyntaxDefinition::RequiredToken(SyntaxKind::Zone),
                SyntaxDefinition::AnyToken,
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableShowStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Show),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Ident, SyntaxKind::All]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DiscardStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Discard),
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::All,
                    SyntaxKind::Plans,
                    SyntaxKind::Sequences,
                    SyntaxKind::Temp,
                    SyntaxKind::Temporary,
                ]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateRoleStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::Role,
                    SyntaxKind::GroupP,
                    SyntaxKind::User,
                ]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterRoleStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Role),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropRoleStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::OneOf(vec![
                    SyntaxKind::Role,
                    SyntaxKind::User,
                    SyntaxKind::GroupP,
                ]),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::LockStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::LockP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ConstraintsSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::RequiredToken(SyntaxKind::Constraints),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ReindexStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Reindex),
                SyntaxDefinition::OptionalToken(SyntaxKind::Concurrently),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CheckPointStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Checkpoint)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateSchemaStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Schema),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDatabaseStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDatabaseRefreshCollStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Refresh),
                SyntaxDefinition::RequiredToken(SyntaxKind::Collation),
                SyntaxDefinition::RequiredToken(SyntaxKind::VersionP),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDatabaseSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterDatabaseSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Database),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Reset),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateConversionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Default),
                SyntaxDefinition::RequiredToken(SyntaxKind::ConversionP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::For),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sconst),
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sconst),
                SyntaxDefinition::RequiredToken(SyntaxKind::From),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateCastStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Cast),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateOpFamilyStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::RequiredToken(SyntaxKind::Family),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterOpFamilyStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
                SyntaxDefinition::RequiredToken(SyntaxKind::Family),
                // for schemas, this should be put into all definitions...
                // SyntaxDefinition::OptionalToken(SyntaxKind::Ident),
                // SyntaxDefinition::OptionalToken(SyntaxKind::Ascii46),
                SyntaxDefinition::AnyTokens,
                // SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Using),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                // this is important to not conflict with RenameStmt
                SyntaxDefinition::OneOf(vec![SyntaxKind::Drop, SyntaxKind::AddP]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::PrepareStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Prepare),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyToken,
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::OneOf(vec![SyntaxKind::With, SyntaxKind::Select]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DeallocateStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Deallocate),
                SyntaxDefinition::OptionalToken(SyntaxKind::Prepare),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Ident, SyntaxKind::All]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateTableSpaceStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Tablespace),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Location),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropTableSpaceStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Tablespace),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterOperatorStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Operator),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterTypeStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropOwnedStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Owned),
                SyntaxDefinition::RequiredToken(SyntaxKind::By),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ReassignOwnedStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Reassign),
                SyntaxDefinition::RequiredToken(SyntaxKind::Owned),
                SyntaxDefinition::RequiredToken(SyntaxKind::By),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateFdwStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::DataP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Wrapper),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterFdwStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::DataP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Wrapper),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateForeignServerStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::DataP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Wrapper),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterForeignServerStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateUserMappingStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::User),
                SyntaxDefinition::RequiredToken(SyntaxKind::Mapping),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::For),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterUserMappingStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::User),
                SyntaxDefinition::RequiredToken(SyntaxKind::Mapping),
                SyntaxDefinition::OptionalToken(SyntaxKind::For),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Options),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropUserMappingStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::User),
                SyntaxDefinition::RequiredToken(SyntaxKind::Mapping),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::OptionalToken(SyntaxKind::For),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::SecLabelStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Security),
                SyntaxDefinition::RequiredToken(SyntaxKind::Label),
                SyntaxDefinition::OptionalToken(SyntaxKind::For),
                SyntaxDefinition::OptionalToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateForeignTableStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ImportForeignSchemaStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::ImportP),
                SyntaxDefinition::RequiredToken(SyntaxKind::Foreign),
                SyntaxDefinition::RequiredToken(SyntaxKind::Schema),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::From),
                SyntaxDefinition::RequiredToken(SyntaxKind::Server),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Into),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateExtensionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Extension),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterExtensionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Extension),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateEventTrigStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Event),
                SyntaxDefinition::RequiredToken(SyntaxKind::Trigger),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Execute),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Function, SyntaxKind::Procedure]),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterEventTrigStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Event),
                SyntaxDefinition::RequiredToken(SyntaxKind::Trigger),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::RefreshMatViewStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Refresh),
                SyntaxDefinition::RequiredToken(SyntaxKind::Materialized),
                SyntaxDefinition::RequiredToken(SyntaxKind::View),
                SyntaxDefinition::OptionalToken(SyntaxKind::Concurrently),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterSystemStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::SystemP),
                SyntaxDefinition::OneOf(vec![SyntaxKind::Set, SyntaxKind::Reset]),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreatePolicyStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Policy),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterPolicyStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Policy),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateTransformStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::RequiredToken(SyntaxKind::Transform),
                SyntaxDefinition::RequiredToken(SyntaxKind::For),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Language),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateAmStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Access),
                SyntaxDefinition::RequiredToken(SyntaxKind::Method),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::TypeP),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreatePublicationStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Publication),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterPublicationStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Publication),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateSubscriptionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Subscription),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Connection),
                SyntaxDefinition::RequiredToken(SyntaxKind::Sconst),
                SyntaxDefinition::RequiredToken(SyntaxKind::Publication),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterSubscriptionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Subscription),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DropSubscriptionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Drop),
                SyntaxDefinition::RequiredToken(SyntaxKind::Subscription),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        // GRANT ALL ON SCHEMA alt_nsp1, alt_nsp2 TO public;
        m.push(StatementDefinition {
            stmt: SyntaxKind::GrantStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Grant),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
            ],
        });

        // REVOKE ALL ON SCHEMA alt_nsp6 FROM regress_alter_generic_user6;
        m.push(StatementDefinition {
            stmt: SyntaxKind::GrantStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Revoke),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterOwnerStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Owner),
                SyntaxDefinition::RequiredToken(SyntaxKind::To),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        // ALTER AGGREGATE alt_func1(int) SET SCHEMA alt_nsp2;
        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterObjectSchemaStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::RequiredToken(SyntaxKind::Schema),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreatePlangStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::OptionalToken(SyntaxKind::Trusted),
                SyntaxDefinition::OptionalToken(SyntaxKind::Procedural),
                SyntaxDefinition::RequiredToken(SyntaxKind::Language),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::CreateStatsStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::RequiredToken(SyntaxKind::Statistics),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::On),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::From),
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

// TODO: complete the hashmap above with all statements:
// RETURN statement (inside SQL function body)
// ReturnStmt,
// SetOperationStmt,
//
// TODO: parsing ambiguity, check docs for solution
// GrantStmt(super::GrantStmt),
// GrantRoleStmt(super::GrantRoleStmt),
// ClosePortalStmt,
// CreatePlangStmt,
// AlterRoleSetStmt,
// DeclareCursorStmt,
// AlterObjectDependsStmt,
// AlterObjectSchemaStmt,
// AlterOwnerStmt,
// AlterEnumStmt,
// AlterTsdictionaryStmt,
// AlterTsconfigurationStmt,
// AlterTableSpaceOptionsStmt,
// AlterTableMoveAllStmt,
// AlterExtensionContentsStmt,
// ReplicaIdentityStmt,
//
