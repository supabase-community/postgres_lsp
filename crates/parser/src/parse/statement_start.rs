use std::collections::HashMap;
use std::sync::LazyLock;

use crate::codegen::SyntaxKind;
use crate::Parser;

pub enum SyntaxToken {
    Required(SyntaxKind),
    Optional(SyntaxKind),
}

#[derive(Debug, Clone, Hash)]
pub enum TokenStatement {
    // The respective token is the last token of the statement
    EoS(SyntaxKind),
    Any(SyntaxKind),
}

impl TokenStatement {
    fn is_eos(&self) -> bool {
        match self {
            TokenStatement::EoS(_) => true,
            _ => false,
        }
    }

    fn kind(&self) -> SyntaxKind {
        match self {
            TokenStatement::EoS(k) => k.to_owned(),
            TokenStatement::Any(k) => k.to_owned(),
        }
    }
}

impl PartialEq for TokenStatement {
    fn eq(&self, other: &Self) -> bool {
        let a = match self {
            TokenStatement::EoS(s) => s,
            TokenStatement::Any(s) => s,
        };

        let b = match other {
            TokenStatement::EoS(s) => s,
            TokenStatement::Any(s) => s,
        };

        return a == b;
    }
}

// vector of hashmaps, where each hashmap returns the list of possible statements for a token at
// the respective index.
//
// For example, at idx 0, the hashmap contains a superset of
// ```
//{
//     Create: [
//         IndexStmt,
//         CreateFunctionStmt,
//         CreateStmt,
//         ViewStmt,
//     ],
//     Select: [
//         SelectStmt,
//     ],
// },
// ```
//
// the idea is to trim down the possible options for each token, until only one statement is left.
//
// The vector is lazily constructed out of another vector of tuples, where each tuple contains a
// statement, and a list of `SyntaxToken`s that are to be found at the start of the statement.
pub static STATEMENT_START_TOKEN_MAPS: LazyLock<Vec<HashMap<SyntaxKind, Vec<TokenStatement>>>> =
    LazyLock::new(|| {
        let mut m: Vec<(SyntaxKind, &'static [SyntaxToken])> = Vec::new();

        m.push((
            SyntaxKind::InsertStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Insert),
                SyntaxToken::Required(SyntaxKind::Into),
            ],
        ));

        m.push((
            SyntaxKind::DeleteStmt,
            &[
                SyntaxToken::Required(SyntaxKind::DeleteP),
                SyntaxToken::Required(SyntaxKind::From),
            ],
        ));

        m.push((
            SyntaxKind::UpdateStmt,
            &[SyntaxToken::Required(SyntaxKind::Update)],
        ));

        m.push((
            SyntaxKind::MergeStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Merge),
                SyntaxToken::Required(SyntaxKind::Into),
            ],
        ));

        m.push((
            SyntaxKind::SelectStmt,
            &[SyntaxToken::Required(SyntaxKind::Select)],
        ));

        m.push((
            SyntaxKind::AlterTableStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Table),
                SyntaxToken::Optional(SyntaxKind::IfP),
                SyntaxToken::Optional(SyntaxKind::Exists),
                SyntaxToken::Optional(SyntaxKind::Only),
                SyntaxToken::Required(SyntaxKind::Ident),
            ],
        ));

        // ALTER TABLE x RENAME ... is different to e.g. alter table alter column...
        m.push((
            SyntaxKind::RenameStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Table),
                SyntaxToken::Optional(SyntaxKind::IfP),
                SyntaxToken::Optional(SyntaxKind::Exists),
                SyntaxToken::Optional(SyntaxKind::Only),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::Rename),
            ],
        ));

        m.push((
            SyntaxKind::AlterDomainStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::DomainP),
            ],
        ));

        m.push((
            SyntaxKind::AlterDefaultPrivilegesStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Default),
                SyntaxToken::Required(SyntaxKind::Privileges),
            ],
        ));

        m.push((
            SyntaxKind::ClusterStmt,
            &[SyntaxToken::Required(SyntaxKind::Cluster)],
        ));

        m.push((
            SyntaxKind::CopyStmt,
            &[SyntaxToken::Required(SyntaxKind::Copy)],
        ));

        // CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE
        // this is overly simplified, but it should be good enough for now
        m.push((
            SyntaxKind::CreateStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Global),
                SyntaxToken::Optional(SyntaxKind::Local),
                SyntaxToken::Optional(SyntaxKind::Temporary),
                SyntaxToken::Optional(SyntaxKind::Temp),
                SyntaxToken::Optional(SyntaxKind::Unlogged),
                SyntaxToken::Optional(SyntaxKind::IfP),
                SyntaxToken::Optional(SyntaxKind::Not),
                SyntaxToken::Optional(SyntaxKind::Exists),
                SyntaxToken::Required(SyntaxKind::Table),
                SyntaxToken::Required(SyntaxKind::Ident),
            ],
        ));

        // CREATE [ OR REPLACE ] AGGREGATE
        m.push((
            SyntaxKind::DefineStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::Aggregate),
            ],
        ));

        // CREATE [ OR REPLACE ] OPERATOR
        m.push((
            SyntaxKind::DefineStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::Operator),
            ],
        ));

        // CREATE [ OR REPLACE ] TYPE
        m.push((
            SyntaxKind::DefineStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::TypeP),
            ],
        ));

        m.push((
            SyntaxKind::DropStmt,
            &[SyntaxToken::Required(SyntaxKind::Drop)],
        ));

        m.push((
            SyntaxKind::TruncateStmt,
            &[SyntaxToken::Required(SyntaxKind::Truncate)],
        ));

        m.push((
            SyntaxKind::CommentStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Comment),
                SyntaxToken::Required(SyntaxKind::On),
            ],
        ));

        m.push((
            SyntaxKind::FetchStmt,
            &[SyntaxToken::Required(SyntaxKind::Fetch)],
        ));

        // CREATE [ UNIQUE ] INDEX
        m.push((
            SyntaxKind::IndexStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Unique),
                SyntaxToken::Required(SyntaxKind::Index),
            ],
        ));

        // CREATE [ OR REPLACE ] FUNCTION
        m.push((
            SyntaxKind::CreateFunctionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::Function),
            ],
        ));

        m.push((
            SyntaxKind::AlterFunctionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Function),
            ],
        ));

        m.push((SyntaxKind::DoStmt, &[SyntaxToken::Required(SyntaxKind::Do)]));

        // CREATE [ OR REPLACE ] RULE
        m.push((
            SyntaxKind::RuleStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::Rule),
            ],
        ));

        m.push((
            SyntaxKind::NotifyStmt,
            &[SyntaxToken::Required(SyntaxKind::Notify)],
        ));
        m.push((
            SyntaxKind::ListenStmt,
            &[SyntaxToken::Required(SyntaxKind::Listen)],
        ));
        m.push((
            SyntaxKind::UnlistenStmt,
            &[SyntaxToken::Required(SyntaxKind::Unlisten)],
        ));

        // TransactionStmt can be Begin or Commit
        m.push((
            SyntaxKind::TransactionStmt,
            &[SyntaxToken::Required(SyntaxKind::BeginP)],
        ));
        m.push((
            SyntaxKind::TransactionStmt,
            &[SyntaxToken::Required(SyntaxKind::Commit)],
        ));

        // CREATE [ OR REPLACE ] [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW
        // this is overly simplified, but it should be good enough for now
        m.push((
            SyntaxKind::ViewStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Optional(SyntaxKind::Temporary),
                SyntaxToken::Optional(SyntaxKind::Temp),
                SyntaxToken::Optional(SyntaxKind::Recursive),
                SyntaxToken::Required(SyntaxKind::View),
            ],
        ));

        m.push((
            SyntaxKind::LoadStmt,
            &[SyntaxToken::Required(SyntaxKind::Load)],
        ));

        m.push((
            SyntaxKind::CreateDomainStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::DomainP),
            ],
        ));

        m.push((
            SyntaxKind::CreatedbStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Database),
            ],
        ));

        m.push((
            SyntaxKind::DropdbStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::Database),
            ],
        ));

        m.push((
            SyntaxKind::VacuumStmt,
            &[SyntaxToken::Required(SyntaxKind::Vacuum)],
        ));

        m.push((
            SyntaxKind::ExplainStmt,
            &[SyntaxToken::Required(SyntaxKind::Explain)],
        ));

        // CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } ] TABLE AS
        // this is overly simplified, but it should be good enough for now
        m.push((
            SyntaxKind::CreateTableAsStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Global),
                SyntaxToken::Optional(SyntaxKind::Local),
                SyntaxToken::Optional(SyntaxKind::Temporary),
                SyntaxToken::Optional(SyntaxKind::Temp),
                SyntaxToken::Required(SyntaxKind::Table),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::As),
            ],
        ));

        m.push((
            SyntaxKind::CreateSeqStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Temporary),
                SyntaxToken::Optional(SyntaxKind::Temp),
                SyntaxToken::Optional(SyntaxKind::Unlogged),
                SyntaxToken::Required(SyntaxKind::Sequence),
            ],
        ));

        m.push((
            SyntaxKind::AlterSeqStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Sequence),
            ],
        ));

        m.push((
            SyntaxKind::VariableSetStmt,
            &[SyntaxToken::Required(SyntaxKind::Set)],
        ));

        m.push((
            SyntaxKind::VariableShowStmt,
            &[SyntaxToken::Required(SyntaxKind::Show)],
        ));

        m.push((
            SyntaxKind::DiscardStmt,
            &[SyntaxToken::Required(SyntaxKind::Discard)],
        ));

        // CREATE [ OR REPLACE ] [ CONSTRAINT ] TRIGGER
        m.push((
            SyntaxKind::CreateTrigStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Optional(SyntaxKind::Constraint),
                SyntaxToken::Required(SyntaxKind::Trigger),
            ],
        ));

        m.push((
            SyntaxKind::CreateRoleStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Role),
            ],
        ));

        m.push((
            SyntaxKind::AlterRoleStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Role),
            ],
        ));

        m.push((
            SyntaxKind::DropRoleStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::Role),
            ],
        ));

        m.push((
            SyntaxKind::LockStmt,
            &[SyntaxToken::Required(SyntaxKind::LockP)],
        ));

        m.push((
            SyntaxKind::ConstraintsSetStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Set),
                SyntaxToken::Required(SyntaxKind::Constraints),
            ],
        ));

        m.push((
            SyntaxKind::ReindexStmt,
            &[SyntaxToken::Required(SyntaxKind::Reindex)],
        ));

        m.push((
            SyntaxKind::CheckPointStmt,
            &[SyntaxToken::Required(SyntaxKind::Checkpoint)],
        ));

        m.push((
            SyntaxKind::CreateSchemaStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Schema),
            ],
        ));

        m.push((
            SyntaxKind::AlterDatabaseStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Database),
                SyntaxToken::Required(SyntaxKind::Ident),
            ],
        ));

        m.push((
            SyntaxKind::AlterDatabaseRefreshCollStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Database),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::Refresh),
                SyntaxToken::Required(SyntaxKind::Collation),
                SyntaxToken::Required(SyntaxKind::VersionP),
            ],
        ));

        m.push((
            SyntaxKind::AlterDatabaseSetStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Database),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::Set),
            ],
        ));

        m.push((
            SyntaxKind::AlterDatabaseSetStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Database),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::Reset),
            ],
        ));

        m.push((
            SyntaxKind::CreateConversionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Default),
                SyntaxToken::Required(SyntaxKind::ConversionP),
            ],
        ));

        m.push((
            SyntaxKind::CreateCastStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Cast),
            ],
        ));

        m.push((
            SyntaxKind::CreateOpClassStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Operator),
                SyntaxToken::Required(SyntaxKind::Class),
            ],
        ));

        m.push((
            SyntaxKind::CreateOpFamilyStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Operator),
                SyntaxToken::Required(SyntaxKind::Family),
            ],
        ));

        m.push((
            SyntaxKind::AlterOpFamilyStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Operator),
                SyntaxToken::Required(SyntaxKind::Family),
            ],
        ));

        m.push((
            SyntaxKind::PrepareStmt,
            &[SyntaxToken::Required(SyntaxKind::Prepare)],
        ));

        m.push((
            SyntaxKind::ExecuteStmt,
            &[SyntaxToken::Required(SyntaxKind::Execute)],
        ));

        m.push((
            SyntaxKind::DeallocateStmt,
            &[SyntaxToken::Required(SyntaxKind::Deallocate)],
        ));

        m.push((
            SyntaxKind::CreateTableSpaceStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Tablespace),
            ],
        ));

        m.push((
            SyntaxKind::DropTableSpaceStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::Tablespace),
            ],
        ));

        m.push((
            SyntaxKind::AlterOperatorStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Operator),
            ],
        ));

        m.push((
            SyntaxKind::AlterTypeStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::TypeP),
            ],
        ));

        m.push((
            SyntaxKind::DropOwnedStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::Owned),
                SyntaxToken::Required(SyntaxKind::By),
            ],
        ));

        m.push((
            SyntaxKind::ReassignOwnedStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Reassign),
                SyntaxToken::Required(SyntaxKind::Owned),
                SyntaxToken::Required(SyntaxKind::By),
            ],
        ));

        m.push((
            SyntaxKind::CreateEnumStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::TypeP),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::As),
                SyntaxToken::Required(SyntaxKind::EnumP),
            ],
        ));

        m.push((
            SyntaxKind::CreateRangeStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::TypeP),
                SyntaxToken::Required(SyntaxKind::Ident),
                SyntaxToken::Required(SyntaxKind::As),
                SyntaxToken::Required(SyntaxKind::Range),
            ],
        ));

        m.push((
            SyntaxKind::CreateFdwStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Foreign),
                SyntaxToken::Required(SyntaxKind::DataP),
                SyntaxToken::Required(SyntaxKind::Wrapper),
            ],
        ));

        m.push((
            SyntaxKind::AlterFdwStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Foreign),
                SyntaxToken::Required(SyntaxKind::DataP),
                SyntaxToken::Required(SyntaxKind::Wrapper),
            ],
        ));

        m.push((
            SyntaxKind::CreateForeignServerStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Server),
            ],
        ));

        m.push((
            SyntaxKind::AlterForeignServerStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Server),
            ],
        ));

        m.push((
            SyntaxKind::CreateUserMappingStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::User),
                SyntaxToken::Required(SyntaxKind::Mapping),
            ],
        ));

        m.push((
            SyntaxKind::AlterUserMappingStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::User),
                SyntaxToken::Required(SyntaxKind::Mapping),
                SyntaxToken::Required(SyntaxKind::For),
            ],
        ));

        m.push((
            SyntaxKind::DropUserMappingStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::User),
                SyntaxToken::Required(SyntaxKind::Mapping),
            ],
        ));

        m.push((
            SyntaxKind::SecLabelStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Security),
                SyntaxToken::Required(SyntaxKind::Label),
            ],
        ));

        m.push((
            SyntaxKind::CreateForeignTableStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Foreign),
                SyntaxToken::Required(SyntaxKind::Table),
            ],
        ));

        m.push((
            SyntaxKind::ImportForeignSchemaStmt,
            &[
                SyntaxToken::Required(SyntaxKind::ImportP),
                SyntaxToken::Required(SyntaxKind::Foreign),
                SyntaxToken::Required(SyntaxKind::Schema),
            ],
        ));

        m.push((
            SyntaxKind::CreateExtensionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Extension),
            ],
        ));

        m.push((
            SyntaxKind::AlterExtensionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Extension),
            ],
        ));

        m.push((
            SyntaxKind::CreateEventTrigStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Event),
                SyntaxToken::Required(SyntaxKind::Trigger),
            ],
        ));

        m.push((
            SyntaxKind::AlterEventTrigStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Event),
                SyntaxToken::Required(SyntaxKind::Trigger),
            ],
        ));

        m.push((
            SyntaxKind::RefreshMatViewStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Refresh),
                SyntaxToken::Required(SyntaxKind::Materialized),
                SyntaxToken::Required(SyntaxKind::View),
            ],
        ));

        m.push((
            SyntaxKind::AlterSystemStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::SystemP),
            ],
        ));

        m.push((
            SyntaxKind::CreatePolicyStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Policy),
            ],
        ));

        m.push((
            SyntaxKind::AlterPolicyStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Policy),
            ],
        ));

        m.push((
            SyntaxKind::CreateTransformStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Optional(SyntaxKind::Or),
                SyntaxToken::Optional(SyntaxKind::Replace),
                SyntaxToken::Required(SyntaxKind::Transform),
                SyntaxToken::Required(SyntaxKind::For),
            ],
        ));

        m.push((
            SyntaxKind::CreateAmStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Access),
                SyntaxToken::Required(SyntaxKind::Method),
            ],
        ));

        m.push((
            SyntaxKind::CreatePublicationStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Publication),
            ],
        ));

        m.push((
            SyntaxKind::AlterPublicationStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Publication),
            ],
        ));

        m.push((
            SyntaxKind::CreateSubscriptionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Subscription),
            ],
        ));

        m.push((
            SyntaxKind::AlterSubscriptionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Subscription),
            ],
        ));

        m.push((
            SyntaxKind::DropSubscriptionStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Drop),
                SyntaxToken::Required(SyntaxKind::Subscription),
            ],
        ));

        m.push((
            SyntaxKind::CreateStatsStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Create),
                SyntaxToken::Required(SyntaxKind::Statistics),
            ],
        ));

        m.push((
            SyntaxKind::AlterCollationStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Collation),
            ],
        ));

        m.push((
            SyntaxKind::CallStmt,
            &[SyntaxToken::Required(SyntaxKind::Call)],
        ));

        m.push((
            SyntaxKind::AlterStatsStmt,
            &[
                SyntaxToken::Required(SyntaxKind::Alter),
                SyntaxToken::Required(SyntaxKind::Statistics),
            ],
        ));

        let mut vec: Vec<HashMap<SyntaxKind, Vec<TokenStatement>>> = Vec::new();

        m.iter().for_each(|(statement, tokens)| {
            let mut left_pull: usize = 0;
            tokens.iter().enumerate().for_each(|(idx, token)| {
                if vec.len() <= idx {
                    vec.push(HashMap::new());
                }

                let is_last = idx == tokens.len() - 1;

                match token {
                    SyntaxToken::Required(t) => {
                        for i in (idx - left_pull)..(idx + 1) {
                            let list_entry = vec[i].entry(t.to_owned());
                            list_entry
                                .and_modify(|list| {
                                    list.push(if is_last {
                                        TokenStatement::EoS(statement.to_owned())
                                    } else {
                                        TokenStatement::Any(statement.to_owned())
                                    });
                                })
                                .or_insert(vec![if is_last {
                                    TokenStatement::EoS(statement.to_owned())
                                } else {
                                    TokenStatement::Any(statement.to_owned())
                                }]);
                        }
                    }
                    SyntaxToken::Optional(t) => {
                        if is_last {
                            panic!("Optional token cannot be last token");
                        }
                        for i in (idx - left_pull)..(idx + 1) {
                            let list_entry = vec[i].entry(t.to_owned());
                            list_entry
                                .and_modify(|list| {
                                    list.push(TokenStatement::Any(statement.to_owned()));
                                })
                                .or_insert(vec![TokenStatement::Any(statement.to_owned())]);
                        }
                        left_pull += 1;
                    }
                }
            });
        });

        vec
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
// CompositeTypeStmt,
// AlterEnumStmt,
// AlterTsdictionaryStmt,
// AlterTsconfigurationStmt,
// AlterTableSpaceOptionsStmt,
// AlterTableMoveAllStmt,
// AlterExtensionContentsStmt,
// ReplicaIdentityStmt,
//

/// Returns the statement at which the parser is currently at, if any
pub fn is_at_stmt_start(parser: &mut Parser) -> Option<SyntaxKind> {
    let mut options = Vec::new();
    for i in 0..STATEMENT_START_TOKEN_MAPS.len() {
        // important, else infinite loop: only ignore whitespaces after first token
        let token = parser.nth(i, i != 0).kind;
        if let Some(result) = STATEMENT_START_TOKEN_MAPS[i].get(&token) {
            if i == 0 {
                options = result.clone();
            } else {
                options = result
                    .iter()
                    .filter(|o| options.contains(o))
                    .cloned()
                    .collect();
            }
        } else if options.len() > 1 {
            // no result is found, and there is currently more than one option
            // filter the options for all statements that are complete at this point
            options.retain(|o| o.is_eos());
        }

        if options.len() == 0 {
            break;
        } else if options.len() == 1 && options.get(0).unwrap().is_eos() {
            break;
        }
    }
    if options.len() == 0 {
        None
    } else if options.len() == 1 && options.get(0).unwrap().is_eos() {
        Some(options.get(0).unwrap().kind())
    } else {
        panic!("Ambiguous statement");
    }
}
