use cstree::build::Checkpoint;
use cstree::syntax::ResolvedNode;
use cstree::text::TextSize;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use petgraph::graph::{DefaultIx, Graph, NodeIndex};
use petgraph::visit::{Bfs, Dfs};
use petgraph::Direction;
use pg_query::NodeEnum;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use std::println;
use std::sync::LazyLock;

use crate::ast_node::RawStmt;
use crate::codegen::{get_nodes, Path, SyntaxKind};
use crate::lexer::{Token, TokenType};
use crate::syntax_error::SyntaxError;
use crate::syntax_node::SyntaxNode;

static WHITESPACE_TOKENS: &[SyntaxKind] = &[
    SyntaxKind::Whitespace,
    SyntaxKind::Tab,
    SyntaxKind::Newline,
    SyntaxKind::SqlComment,
];

enum SyntaxToken {
    Required(SyntaxKind),
    Optional(SyntaxKind),
}

#[derive(Debug, Clone, Hash)]
enum TokenStatement {
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
static STATEMENT_START_TOKEN_MAPS: LazyLock<Vec<HashMap<SyntaxKind, Vec<TokenStatement>>>> =
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
                SyntaxToken::Required(SyntaxKind::Table),
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

/// Main parser that exposes the `cstree` api, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abstract syntax tree
    stmts: Vec<RawStmt>,
    /// The tokens to parse
    tokens: Vec<Token>,
    /// The current position in the token stream
    pos: usize,
    /// index from which whitespace tokens are buffered
    whitespace_token_buffer: Option<usize>,

    /// index from which tokens are buffered
    token_buffer: Option<usize>,

    depth: usize,
}

/// Result of Building
#[derive(Debug)]
pub struct Parse {
    /// The concrete syntax tree
    pub cst: ResolvedNode<SyntaxKind>,
    /// The syntax errors accumulated during parsing
    pub errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
    pub stmts: Vec<RawStmt>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            inner: GreenNodeBuilder::new(),
            errors: Vec::new(),
            stmts: Vec::new(),
            tokens,
            pos: 0,
            whitespace_token_buffer: None,
            token_buffer: None,
            depth: 0,
        }
    }

    /// start a new node of `SyntaxKind`
    fn start_node(&mut self, kind: SyntaxKind) {
        debug!("start_node: {:?}", kind);
        self.flush_token_buffer();
        self.inner.start_node(kind);
        self.depth += 1;
    }

    /// finish current node
    fn finish_node(&mut self) {
        debug!("finish_node");
        self.inner.finish_node();
        self.depth -= 1;
    }

    /// collects an SyntaxError with an `error` message at `range`
    fn error(&mut self, error: String, range: TextRange) {
        self.errors.push(SyntaxError::new(error, range));
    }

    /// collects an SyntaxError with an `error` message at `offset`
    fn error_at_offset(&mut self, error: String, offset: TextSize) {
        self.errors.push(SyntaxError::new_at_offset(error, offset));
    }

    /// collects an SyntaxError with an `error` message at `pos`
    fn error_at_pos(&mut self, error: String, pos: usize) {
        self.errors.push(SyntaxError::new_at_offset(
            error,
            self.tokens
                .get(min(self.tokens.len() - 1, pos))
                .unwrap()
                .span
                .start(),
        ));
    }

    /// collects a pg_query `stmt` at `range`
    fn stmt(&mut self, stmt: NodeEnum, range: TextRange) {
        self.stmts.push(RawStmt { stmt, range });
    }

    /// finish cstree and return `Parse`
    pub fn finish(self) -> Parse {
        let (tree, cache) = self.inner.finish();
        Parse {
            cst: SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap()),
            stmts: self.stmts,
            errors: self.errors,
        }
    }

    /// Prepare for maybe wrapping the next node with a surrounding node.
    ///
    /// The way wrapping works is that you first get a checkpoint, then you add nodes and tokens as
    /// normal, and then you *maybe* call [`start_node_at`](Parser::start_node_at).
    fn checkpoint(self) -> Checkpoint {
        self.inner.checkpoint()
    }

    /// Wrap the previous branch marked by [`checkpoint`](Parser::checkpoint) in a new
    /// branch and make it current.
    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.flush_token_buffer();
        self.inner.start_node_at(checkpoint, kind);
    }

    /// Opens a buffer for tokens. While the buffer is active, tokens are not applied to the tree.
    fn open_buffer(&mut self) {
        self.token_buffer = Some(self.pos);
    }

    /// Closes the current token buffer, resets the position to the start of the buffer and returns the range of buffered tokens.
    fn close_buffer(&mut self) -> Range<usize> {
        let token_buffer = self.token_buffer.unwrap();
        let token_range = token_buffer..self.whitespace_token_buffer.unwrap_or(self.pos);
        self.token_buffer = None;
        self.pos = token_buffer;
        token_range
    }

    /// applies token and advances
    fn advance(&mut self) {
        assert!(!self.eof());
        if WHITESPACE_TOKENS.contains(&self.nth(0, false)) {
            if self.whitespace_token_buffer.is_none() {
                self.whitespace_token_buffer = Some(self.pos);
            }
        } else {
            self.flush_token_buffer();
            if self.token_buffer.is_none() {
                let token = self.tokens.get(self.pos).unwrap();
                self.inner.token(token.kind, &token.text);
            }
        }
        self.pos += 1;
    }

    /// flush token buffer and applies all tokens
    fn flush_token_buffer(&mut self) {
        if self.whitespace_token_buffer.is_none() {
            return;
        }
        while self.whitespace_token_buffer.unwrap() < self.pos {
            let token = self
                .tokens
                .get(self.whitespace_token_buffer.unwrap())
                .unwrap();
            if self.token_buffer.is_none() {
                self.inner.token(token.kind, &token.text);
            }
            self.whitespace_token_buffer = Some(self.whitespace_token_buffer.unwrap() + 1);
        }
        self.whitespace_token_buffer = None;
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_whitespace(&mut self) {
        while WHITESPACE_TOKENS.contains(&self.nth(0, false)) {
            self.advance();
        }
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// lookahead method.
    ///
    /// if `ignore_whitespace` is true, it will skip all whitespace tokens
    fn nth(&self, lookahead: usize, ignore_whitespace: bool) -> SyntaxKind {
        if ignore_whitespace {
            let mut idx = 0;
            let mut non_whitespace_token_ctr = 0;
            loop {
                match self.tokens.get(self.pos + idx) {
                    Some(token) => {
                        if !WHITESPACE_TOKENS.contains(&token.kind) {
                            if non_whitespace_token_ctr == lookahead {
                                return token.kind;
                            }
                            non_whitespace_token_ctr += 1;
                        }
                        idx += 1;
                    }
                    None => {
                        return SyntaxKind::Eof;
                    }
                }
            }
        } else {
            self.tokens
                .get(self.pos + lookahead)
                .map_or(SyntaxKind::Eof, |it| it.kind)
        }
    }

    /// checks if the current token is any of `kinds`
    fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.iter().any(|&it| self.at(it))
    }

    /// checks if the current token is of `kind`
    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0, false) == kind
    }

    /// like at, but for multiple consecutive tokens
    fn at_all(&self, kinds: &[SyntaxKind]) -> bool {
        kinds
            .iter()
            .enumerate()
            .all(|(idx, &it)| self.nth(idx, false) == it)
    }

    /// like at_any, but for multiple consecutive tokens
    fn at_any_all(&self, kinds: &Vec<&[SyntaxKind]>) -> bool {
        kinds.iter().any(|&it| self.at_all(it))
    }

    /// Returns the statement at which the parser is currently at, if any
    fn at_stmt_start(&self) -> Option<SyntaxKind> {
        let mut options = Vec::new();
        for i in 0..STATEMENT_START_TOKEN_MAPS.len() {
            // important, else infinite loop: only ignore whitespaces after first token
            let token = self.nth(i, i != 0);
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

    fn expect(&mut self, kind: SyntaxKind) {
        if self.eat(kind) {
            return;
        }
        if self.whitespace_token_buffer.is_some() {
            self.error_at_pos(
                format!(
                    "Expected {:#?}, found {:#?}",
                    kind,
                    self.tokens[self.whitespace_token_buffer.unwrap()].kind
                ),
                self.whitespace_token_buffer.unwrap(),
            );
        } else {
            self.error_at_pos(
                format!("Expected {:#?}, found {:#?}", kind, self.nth(0, false)),
                self.pos + 1,
            );
        }
    }

    pub fn source(&mut self) {
        self.start_node(SyntaxKind::SourceFile);

        while !self.eof() {
            match self.at_stmt_start() {
                Some(stmt) => {
                    self.any_stmt(stmt);
                }
                None => {
                    self.advance();
                }
            }
        }
        self.finish_node();
    }

    fn any_stmt(&mut self, kind: SyntaxKind) {
        self.start_node(kind);

        // open buffer
        self.open_buffer();

        // advance with all start tokens of statement
        for i in 0..STATEMENT_START_TOKEN_MAPS.len() {
            self.eat_whitespace();
            let token = self.nth(0, false);
            if let Some(result) = STATEMENT_START_TOKEN_MAPS[i].get(&token) {
                let is_in_results = result
                    .iter()
                    .find(|x| match x {
                        TokenStatement::EoS(y) | TokenStatement::Any(y) => y == &kind,
                    })
                    .is_some();
                if i == 0 && !is_in_results {
                    panic!("Expected statement start");
                } else if is_in_results {
                    self.expect(token);
                } else {
                    break;
                }
            }
        }

        let mut is_parsing_sub_stmt = false;
        while !self.at(SyntaxKind::Ascii59) && !self.eof() {
            match self.nth(0, false) {
                // opening brackets "(", consume until closing bracket ")"
                SyntaxKind::Ascii40 => {
                    is_parsing_sub_stmt = true;
                    self.advance();
                }
                SyntaxKind::Ascii41 => {
                    is_parsing_sub_stmt = false;
                    self.advance();
                }
                _ => {
                    // if another stmt FIRST is encountered, break
                    // ignore if parsing sub stmt
                    if is_parsing_sub_stmt == false && self.at_stmt_start().is_some() {
                        break;
                    } else {
                        self.advance();
                    }
                }
            }
        }

        self.expect(SyntaxKind::Ascii59);

        // close buffer, get tokens and reset pos
        let token_range = self.close_buffer();
        let tokens = self.tokens[token_range.clone()].to_vec();
        match pg_query::parse(
            tokens
                .iter()
                .map(|t| t.text.clone())
                .collect::<String>()
                .as_str(),
        ) {
            Ok(result) => {
                // TODO: return syntax kind and use it in checkpoint
                self.libpg_query_node(
                    result
                        .protobuf
                        .nodes()
                        .iter()
                        .find(|n| n.1 == 1)
                        .unwrap()
                        .0
                        .to_enum(),
                    token_range.end,
                );
            }
            Err(err) => {
                println!("error: {}", err);
                self.error(
                    err.to_string(),
                    TextRange::new(
                        TextSize::from(u32::try_from(token_range.start).unwrap()),
                        TextSize::from(u32::try_from(token_range.end).unwrap()),
                    ),
                );
                while self.pos < token_range.end {
                    self.advance();
                }
            }
        };

        assert_eq!(self.pos, token_range.end);

        self.finish_node();
    }

    fn libpg_query_node(&mut self, node: NodeEnum, until: usize) {
        let mut node_graph = get_nodes(&node, self.depth);

        // I do not know if this is safe, but I could not find a way to get the index of the root
        // node
        let mut current_node = NodeIndex::<DefaultIx>::new(0);
        let mut open_nodes = Vec::<NodeIndex<DefaultIx>>::new();

        while self.pos < until {
            let current_token = self.tokens.get(self.pos).unwrap();
            if current_token.token_type == TokenType::Whitespace {
                self.advance();
                continue;
            }
            if current_token.kind == crate::codegen::SyntaxKind::Ascii46 {
                self.advance();
                continue;
            }
            if current_token.kind == crate::codegen::SyntaxKind::Ascii59 {
                self.advance();
                continue;
            }
            if let Some(idx) = node_graph[current_node]
                .properties
                .iter()
                .position(|p| cmp_tokens(p, current_token))
            {
                // token is in current node. remove and advance
                if open_nodes.iter().all(|n| *n != current_node) {
                    self.start_node(node_graph[current_node].kind);
                    open_nodes.push(current_node);
                }
                node_graph[current_node].properties.remove(idx);
                self.advance();
                continue;
            }

            // breadth-first search of token in node properties
            let mut bfs = Bfs::new(&node_graph, current_node);
            let current_node_children = node_graph
                .neighbors_directed(current_node, Direction::Outgoing)
                .collect::<Vec<NodeIndex<DefaultIx>>>();
            let mut skipped_nodes = Vec::<NodeIndex<DefaultIx>>::new();
            let start_pos = self.pos;
            while let Some(nx) = bfs.next(&node_graph) {
                // if all direct children of the current node are being skipped, break
                if current_node_children
                    .iter()
                    .all(|n| skipped_nodes.contains(&n))
                {
                    break;
                }

                // if the current node has an edge to any node that is being skipped, skip the current
                // this will ensure that we skip invalid branches entirely
                if skipped_nodes
                    .iter()
                    .any(|n| node_graph.contains_edge(nx, *n))
                {
                    skipped_nodes.push(nx);
                    continue;
                }

                let prop_idx = node_graph[nx]
                    .properties
                    .iter()
                    .position(|p| cmp_tokens(p, current_token));
                if prop_idx.is_some() {
                    // if the current node has the current token as property

                    // remove the property from the node
                    node_graph[nx].properties.remove(prop_idx.unwrap());

                    // close all nodes until the target depth is reached
                    while self.depth > node_graph[nx].depth + 1 {
                        node_graph.remove_node(open_nodes.pop().unwrap());
                        self.finish_node();
                    }

                    if open_nodes.iter().any(|n| *n == nx) {
                        // if the node is already open, advance and continue with the next token
                        self.advance();
                        break;
                    }

                    // open all nodes until the target node is reached
                    let mut nodes_to_open = Vec::<NodeIndex<DefaultIx>>::new();
                    nodes_to_open.push(nx);
                    while nodes_to_open.last().unwrap() != &current_node {
                        if let Some(parent) = node_graph
                            .neighbors_directed(*nodes_to_open.last().unwrap(), Direction::Incoming)
                            .next()
                        {
                            if parent != current_node {
                                nodes_to_open.push(parent);
                            } else {
                                break;
                            }
                        } else {
                            // no parent, reached root node
                            break;
                        }
                    }
                    nodes_to_open.iter().rev().for_each(|n| {
                        self.start_node(node_graph[*n].kind);
                        open_nodes.push(*n);
                    });

                    self.advance();

                    current_node = nx;

                    // while current node is a leaf node with no properties, close
                    while node_graph
                        .neighbors_directed(current_node, Direction::Outgoing)
                        .count()
                        == 0
                        && node_graph[current_node].properties.len() == 0
                    {
                        let parent = open_nodes.pop().unwrap();
                        node_graph.remove_node(parent);
                        self.finish_node();
                        if open_nodes.len() == 0 {
                            break;
                        }
                        current_node = open_nodes.last().unwrap().clone();
                    }

                    break;
                } else if node_graph[nx].properties.len() > 0 {
                    // if the current node has properties and the token does not match any of them, add it
                    // to the list of skipped nodes and continue
                    skipped_nodes.push(nx);
                    continue;
                }
            }

            if self.pos == start_pos {
                // did not advance, token was not found via bfs
                // FIXME: when a node has more than one non-consecutive properties / tokens we also
                // need to search upwards in the parents until the root
                // - store last opened node with remaining props
                // - search in path from that node to the current node
                // - store last opened node with remaining props
                // - if set, search in path from that node to the current node
                // - if not found, throw
                let mut nx = current_node;
                while let Some(parent) = node_graph
                    .neighbors_directed(nx, Direction::Incoming)
                    .next()
                {
                    let prop_idx = node_graph[parent]
                        .properties
                        .iter()
                        .position(|p| cmp_tokens(p, self.tokens.get(self.pos).unwrap()));
                    if prop_idx.is_none() {
                        // move up
                        nx = parent;
                        continue;
                    }
                    // search for token in properties

                    // remove the property from the node
                    node_graph[parent].properties.remove(prop_idx.unwrap());

                    // close all nodes until before the target depth is reached
                    while self.depth != node_graph[parent].depth + 1 {
                        node_graph.remove_node(open_nodes.pop().unwrap());
                        self.finish_node();
                    }

                    // do not open any new nodes because the node is already open

                    // set the current node to the deepest node (looking up from the current node) that has at least one children
                    // has_children is true if there are outgoing neighbors
                    let mut parent_with_children = parent;
                    while node_graph
                        .neighbors_directed(parent_with_children, Direction::Outgoing)
                        .count()
                        == 0
                    {
                        if let Some(parent) = node_graph
                            .neighbors_directed(parent_with_children, Direction::Incoming)
                            .next()
                        {
                            parent_with_children = parent;
                        } else {
                            // root node reached
                            break;
                        }
                    }
                    current_node = parent_with_children;

                    self.advance();
                    break;
                }
            }

            if self.pos == start_pos {
                // did not advance --> panic
                panic!(
                    "Could not find token {:?} in node graph",
                    self.tokens.get(self.pos).unwrap()
                );
            }
        }
        // close all remaining nodes
        open_nodes.drain(..).for_each(|_| {
            self.finish_node();
        });
    }
}

fn cmp_tokens(p: &crate::codegen::TokenProperty, token: &Token) -> bool {
    (!p.value.is_some() || p.value.as_ref().unwrap() == &token.text)
        && (!p.kind.is_some() || p.kind.unwrap() == token.kind)
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use std::{sync::mpsc, thread, time::Duration};

    use crate::lexer::lex;

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_playground() {
        init();

        let input = "SHOW all;";
        let parsed = pg_query::parse(input).unwrap();
        let scanned = pg_query::scan(input).unwrap();

        STATEMENT_START_TOKEN_MAPS.iter().for_each(|it| {
            //
        });
        println!("{:#?}", parsed.protobuf.nodes());
        println!("{:#?}", scanned.tokens);
    }

    #[test]
    fn test_parser_beefy() {
        init();

        panic_after(Duration::from_millis(1000), || {
            let input = "SELECT
  c.oid :: int8 AS id,
  nc.nspname AS schema,
  c.relname AS name,
  c.relrowsecurity AS rls_enabled,
  c.relforcerowsecurity AS rls_forced,
  CASE
    WHEN c.relreplident = 'd' THEN 'DEFAULT'
    WHEN c.relreplident = 'i' THEN 'INDEX'
    WHEN c.relreplident = 'f' THEN 'FULL'
    ELSE 'NOTHING'
  END AS replica_identity,
  pg_total_relation_size(format('%I.%I', nc.nspname, c.relname)) :: int8 AS bytes,
  pg_size_pretty(
    pg_total_relation_size(format('%I.%I', nc.nspname, c.relname))
  ) AS size,
  pg_stat_get_live_tuples(c.oid) AS live_rows_estimate,
  pg_stat_get_dead_tuples(c.oid) AS dead_rows_estimate,
  obj_description(c.oid) AS comment,
  coalesce(pk.primary_keys, '[]') as primary_keys,
  coalesce(
    jsonb_agg(relationships) filter (where relationships is not null),
    '[]'
  ) as relationships
FROM
  pg_namespace nc
  JOIN pg_class c ON nc.oid = c.relnamespace
  left join (
    select
      table_id,
      jsonb_agg(_pk.*) as primary_keys
    from (
      select
        n.nspname as schema,
        c.relname as table_name,
        a.attname as name,
        c.oid :: int8 as table_id
      from
        pg_index i,
        pg_class c,
        pg_attribute a,
        pg_namespace n
      where
        i.indrelid = c.oid
        and c.relnamespace = n.oid
        and a.attrelid = c.oid
        and a.attnum = any (i.indkey)
        and i.indisprimary
    ) as _pk
    group by table_id
  ) as pk
  on pk.table_id = c.oid
  left join (
    select
      c.oid :: int8 as id,
      c.conname as constraint_name,
      nsa.nspname as source_schema,
      csa.relname as source_table_name,
      sa.attname as source_column_name,
      nta.nspname as target_table_schema,
      cta.relname as target_table_name,
      ta.attname as target_column_name
    from
      pg_constraint c
    join (
      pg_attribute sa
      join pg_class csa on sa.attrelid = csa.oid
      join pg_namespace nsa on csa.relnamespace = nsa.oid
    ) on sa.attrelid = c.conrelid and sa.attnum = any (c.conkey)
    join (
      pg_attribute ta
      join pg_class cta on ta.attrelid = cta.oid
      join pg_namespace nta on cta.relnamespace = nta.oid
    ) on ta.attrelid = c.confrelid and ta.attnum = any (c.confkey)
    where
      c.contype = 'f'
  ) as relationships
  on (relationships.source_schema = nc.nspname and relationships.source_table_name = c.relname)
  or (relationships.target_table_schema = nc.nspname and relationships.target_table_name = c.relname)
WHERE
  c.relkind IN ('r', 'p')
  AND NOT pg_is_other_temp_schema(nc.oid)
  AND (
    pg_has_role(c.relowner, 'USAGE')
    OR has_table_privilege(
      c.oid,
      'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
    )
    OR has_any_column_privilege(c.oid, 'SELECT, INSERT, UPDATE, REFERENCES')
  )
group by
  c.oid,
  c.relname,
  c.relrowsecurity,
  c.relforcerowsecurity,
  c.relreplident,
  nc.nspname,
  pk.primary_keys";

            let mut p = Parser::new(lex(input));
            p.source();
            let result = p.finish();

            dbg!(&result.cst);
            println!("{:#?}", result.errors);
        })
    }

    #[test]
    fn test_parser_very_simple() {
        init();

        panic_after(Duration::from_millis(100), || {
            let input = "select * from public.contact where x = 1;";

            let mut p = Parser::new(lex(input));
            p.source();
            let result = p.finish();

            dbg!(&result.cst);
            println!("{:#?}", result.errors);
        })
    }

    #[test]
    fn test_parser_simple() {
        init();

        panic_after(Duration::from_millis(100), || {
            let input = "alter table x rename to y \n alter table x alter column z set default 1";
            // let input = "select 1; \n -- some comment \n select 2;";

            let mut p = Parser::new(lex(input));
            p.source();
            let result = p.finish();

            dbg!(&result.cst);
            println!("{:#?}", result.errors);
        })
    }

    #[test]
    fn test_graph_crate() {
        init();

        struct N {
            kind: SyntaxKind,
        }

        let mut g = Graph::<N, ()>::new();
        let idx = g.add_node(N {
            kind: SyntaxKind::SourceFile,
        });
        println!("{:#?}", idx);
    }

    fn panic_after<T, F>(d: Duration, f: F) -> T
    where
        T: Send + 'static,
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let (done_tx, done_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let val = f();
            done_tx.send(()).expect("Unable to send completion signal");
            val
        });

        match done_rx.recv_timeout(d) {
            Ok(_) => handle.join().expect("Thread panicked"),
            Err(_) => panic!("Thread took too long"),
        }
    }
}
