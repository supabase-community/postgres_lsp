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
            stmt: SyntaxKind::InsertStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Insert),
                SyntaxDefinition::RequiredToken(SyntaxKind::Into),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::OptionalToken(SyntaxKind::Only),
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
                SyntaxDefinition::OptionalToken(SyntaxKind::IfP),
                SyntaxDefinition::OptionalToken(SyntaxKind::Not),
                SyntaxDefinition::OptionalToken(SyntaxKind::Exists),
                SyntaxDefinition::RequiredToken(SyntaxKind::Table),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DefineStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Execute),
                SyntaxDefinition::OptionalToken(SyntaxKind::Or),
                SyntaxDefinition::OptionalToken(SyntaxKind::Replace),
                SyntaxDefinition::RequiredToken(SyntaxKind::Aggregate),
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
            stmt: SyntaxKind::Drop,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Drop)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::Truncate,
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
            stmt: SyntaxKind::IndexStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Create),
                SyntaxDefinition::OptionalToken(SyntaxKind::Unique),
                SyntaxDefinition::RequiredToken(SyntaxKind::Index),
                SyntaxDefinition::OptionalToken(SyntaxKind::Concurrently),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Function),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii40),
                SyntaxDefinition::AnyTokens,
                SyntaxDefinition::RequiredToken(SyntaxKind::Ascii41),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::AlterFunctionStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Alter),
                SyntaxDefinition::RequiredToken(SyntaxKind::Function),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::DoStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Do)],
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

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::BeginP)],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::TransactionStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Commit)],
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
            stmt: SyntaxKind::ExplainStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Explain)],
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
            ],
        });

        m.push(StatementDefinition {
            stmt: SyntaxKind::ExplainStmt,
            tokens: vec![SyntaxDefinition::RequiredToken(SyntaxKind::Explain)],
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

        m.push(StatementDefinition {
            stmt: SyntaxKind::VariableSetStmt,
            tokens: vec![
                SyntaxDefinition::RequiredToken(SyntaxKind::Set),
                SyntaxDefinition::OptionalToken(SyntaxKind::Session),
                SyntaxDefinition::OptionalToken(SyntaxKind::Local),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Role),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Role),
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
                SyntaxDefinition::OptionalToken(SyntaxKind::Schema),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::As),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
                SyntaxDefinition::RequiredToken(SyntaxKind::Using),
                SyntaxDefinition::RequiredToken(SyntaxKind::Ident),
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
                SyntaxDefinition::RequiredToken(SyntaxKind::Import),
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
