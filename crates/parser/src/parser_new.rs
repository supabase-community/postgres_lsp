use cstree::build::Checkpoint;
use cstree::syntax::ResolvedNode;
use cstree::text::TextSize;
use cstree::{build::GreenNodeBuilder, text::TextRange};
use log::debug;
use pg_query::NodeEnum;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::ast_node::RawStmt;
use crate::lexer::Token;
use crate::syntax_error::SyntaxError;
use crate::syntax_kind_codegen::SyntaxKind;
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

        println!("{:#?}", vec);

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
//
// ClosePortalStmt,
//
// #[prost(message, tag="89")]
// CreateDomainStmt(::prost::alloc::boxed::Box<super::CreateDomainStmt>),
// #[prost(message, tag="90")]
// CreatedbStmt(super::CreatedbStmt),
// #[prost(message, tag="91")]
// DropdbStmt(super::DropdbStmt),
// #[prost(message, tag="92")]
// VacuumStmt(super::VacuumStmt),
// #[prost(message, tag="93")]
// ExplainStmt(::prost::alloc::boxed::Box<super::ExplainStmt>),
// #[prost(message, tag="94")]
// CreateTableAsStmt(::prost::alloc::boxed::Box<super::CreateTableAsStmt>),
// #[prost(message, tag="95")]
// CreateSeqStmt(super::CreateSeqStmt),
// #[prost(message, tag="96")]
// AlterSeqStmt(super::AlterSeqStmt),
// #[prost(message, tag="97")]
// VariableSetStmt(super::VariableSetStmt),
// #[prost(message, tag="98")]
// VariableShowStmt(super::VariableShowStmt),
// #[prost(message, tag="99")]
// DiscardStmt(super::DiscardStmt),
// #[prost(message, tag="100")]
// CreateTrigStmt(::prost::alloc::boxed::Box<super::CreateTrigStmt>),
// #[prost(message, tag="101")]
// CreatePlangStmt(super::CreatePLangStmt),
// #[prost(message, tag="102")]
// CreateRoleStmt(super::CreateRoleStmt),
// #[prost(message, tag="103")]
// AlterRoleStmt(super::AlterRoleStmt),
// #[prost(message, tag="104")]
// DropRoleStmt(super::DropRoleStmt),
// #[prost(message, tag="105")]
// LockStmt(super::LockStmt),
// #[prost(message, tag="106")]
// ConstraintsSetStmt(super::ConstraintsSetStmt),
// #[prost(message, tag="107")]
// ReindexStmt(super::ReindexStmt),
// #[prost(message, tag="108")]
// CheckPointStmt(super::CheckPointStmt),
// #[prost(message, tag="109")]
// CreateSchemaStmt(super::CreateSchemaStmt),
// #[prost(message, tag="110")]
// AlterDatabaseStmt(super::AlterDatabaseStmt),
// #[prost(message, tag="111")]
// AlterDatabaseRefreshCollStmt(super::AlterDatabaseRefreshCollStmt),
// #[prost(message, tag="112")]
// AlterDatabaseSetStmt(super::AlterDatabaseSetStmt),
// #[prost(message, tag="113")]
// AlterRoleSetStmt(super::AlterRoleSetStmt),
// #[prost(message, tag="114")]
// CreateConversionStmt(super::CreateConversionStmt),
// #[prost(message, tag="115")]
// CreateCastStmt(super::CreateCastStmt),
// #[prost(message, tag="116")]
// CreateOpClassStmt(super::CreateOpClassStmt),
// #[prost(message, tag="117")]
// CreateOpFamilyStmt(super::CreateOpFamilyStmt),
// #[prost(message, tag="118")]
// AlterOpFamilyStmt(super::AlterOpFamilyStmt),
// #[prost(message, tag="119")]
// PrepareStmt(::prost::alloc::boxed::Box<super::PrepareStmt>),
// #[prost(message, tag="120")]
// ExecuteStmt(super::ExecuteStmt),
// #[prost(message, tag="121")]
// DeallocateStmt(super::DeallocateStmt),
// #[prost(message, tag="122")]
// DeclareCursorStmt(::prost::alloc::boxed::Box<super::DeclareCursorStmt>),
// #[prost(message, tag="123")]
// CreateTableSpaceStmt(super::CreateTableSpaceStmt),
// #[prost(message, tag="124")]
// DropTableSpaceStmt(super::DropTableSpaceStmt),
// #[prost(message, tag="125")]
// AlterObjectDependsStmt(::prost::alloc::boxed::Box<super::AlterObjectDependsStmt>),
// #[prost(message, tag="126")]
// AlterObjectSchemaStmt(::prost::alloc::boxed::Box<super::AlterObjectSchemaStmt>),
// #[prost(message, tag="127")]
// AlterOwnerStmt(::prost::alloc::boxed::Box<super::AlterOwnerStmt>),
// #[prost(message, tag="128")]
// AlterOperatorStmt(super::AlterOperatorStmt),
// #[prost(message, tag="129")]
// AlterTypeStmt(super::AlterTypeStmt),
// #[prost(message, tag="130")]
// DropOwnedStmt(super::DropOwnedStmt),
// #[prost(message, tag="131")]
// ReassignOwnedStmt(super::ReassignOwnedStmt),
// #[prost(message, tag="132")]
// CompositeTypeStmt(super::CompositeTypeStmt),
// #[prost(message, tag="133")]
// CreateEnumStmt(super::CreateEnumStmt),
// #[prost(message, tag="134")]
// CreateRangeStmt(super::CreateRangeStmt),
// #[prost(message, tag="135")]
// AlterEnumStmt(super::AlterEnumStmt),
// #[prost(message, tag="136")]
// AlterTsdictionaryStmt(super::AlterTsDictionaryStmt),
// #[prost(message, tag="137")]
// AlterTsconfigurationStmt(super::AlterTsConfigurationStmt),
// #[prost(message, tag="138")]
// CreateFdwStmt(super::CreateFdwStmt),
// #[prost(message, tag="139")]
// AlterFdwStmt(super::AlterFdwStmt),
// #[prost(message, tag="140")]
// CreateForeignServerStmt(super::CreateForeignServerStmt),
// #[prost(message, tag="141")]
// AlterForeignServerStmt(super::AlterForeignServerStmt),
// #[prost(message, tag="142")]
// CreateUserMappingStmt(super::CreateUserMappingStmt),
// #[prost(message, tag="143")]
// AlterUserMappingStmt(super::AlterUserMappingStmt),
// #[prost(message, tag="144")]
// DropUserMappingStmt(super::DropUserMappingStmt),
// #[prost(message, tag="145")]
// AlterTableSpaceOptionsStmt(super::AlterTableSpaceOptionsStmt),
// #[prost(message, tag="146")]
// AlterTableMoveAllStmt(super::AlterTableMoveAllStmt),
// #[prost(message, tag="147")]
// SecLabelStmt(::prost::alloc::boxed::Box<super::SecLabelStmt>),
// #[prost(message, tag="148")]
// CreateForeignTableStmt(super::CreateForeignTableStmt),
// #[prost(message, tag="149")]
// ImportForeignSchemaStmt(super::ImportForeignSchemaStmt),
// #[prost(message, tag="150")]
// CreateExtensionStmt(super::CreateExtensionStmt),
// #[prost(message, tag="151")]
// AlterExtensionStmt(super::AlterExtensionStmt),
// #[prost(message, tag="152")]
// AlterExtensionContentsStmt(::prost::alloc::boxed::Box<super::AlterExtensionContentsStmt>),
// #[prost(message, tag="153")]
// CreateEventTrigStmt(super::CreateEventTrigStmt),
// #[prost(message, tag="154")]
// AlterEventTrigStmt(super::AlterEventTrigStmt),
// #[prost(message, tag="155")]
// RefreshMatViewStmt(super::RefreshMatViewStmt),
// #[prost(message, tag="156")]
// ReplicaIdentityStmt(super::ReplicaIdentityStmt),
// #[prost(message, tag="157")]
// AlterSystemStmt(super::AlterSystemStmt),
// #[prost(message, tag="158")]
// CreatePolicyStmt(::prost::alloc::boxed::Box<super::CreatePolicyStmt>),
// #[prost(message, tag="159")]
// AlterPolicyStmt(::prost::alloc::boxed::Box<super::AlterPolicyStmt>),
// #[prost(message, tag="160")]
// CreateTransformStmt(super::CreateTransformStmt),
// #[prost(message, tag="161")]
// CreateAmStmt(super::CreateAmStmt),
// #[prost(message, tag="162")]
// CreatePublicationStmt(super::CreatePublicationStmt),
// #[prost(message, tag="163")]
// AlterPublicationStmt(super::AlterPublicationStmt),
// #[prost(message, tag="164")]
// CreateSubscriptionStmt(super::CreateSubscriptionStmt),
// #[prost(message, tag="165")]
// AlterSubscriptionStmt(super::AlterSubscriptionStmt),
// #[prost(message, tag="166")]
// DropSubscriptionStmt(super::DropSubscriptionStmt),
// #[prost(message, tag="167")]
// CreateStatsStmt(super::CreateStatsStmt),
// #[prost(message, tag="168")]
// AlterCollationStmt(super::AlterCollationStmt),
// #[prost(message, tag="169")]
// CallStmt(::prost::alloc::boxed::Box<super::CallStmt>),
// #[prost(message, tag="170")]
// AlterStatsStmt(super::AlterStatsStmt),

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
    /// index from which tokens are buffered
    whitespace_token_buffer: Option<usize>,
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
        }
    }

    /// start a new node of `SyntaxKind`
    fn start_node(&mut self, kind: SyntaxKind) {
        debug!("start_node: {:?}", kind);
        self.flush_token_buffer();
        self.inner.start_node(kind);
    }

    /// finish current node
    fn finish_node(&mut self) {
        debug!("finish_node");
        self.inner.finish_node();
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
    fn finish(self) -> Parse {
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

    /// applies token and advances
    fn advance(&mut self) {
        assert!(!self.eof());
        if WHITESPACE_TOKENS.contains(&self.nth(0, false)) {
            if self.whitespace_token_buffer.is_none() {
                self.whitespace_token_buffer = Some(self.pos);
            }
        } else {
            self.flush_token_buffer();
            let token = self.tokens.get(self.pos).unwrap();
            self.inner.token(token.kind, &token.text);
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
            self.inner.token(token.kind, &token.text);
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

    /// lookahead method
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
                options.retain(|o| match o {
                    TokenStatement::Any(_) => false,
                    TokenStatement::EoS(_) => true,
                });
            }

            if options.len() <= 1 {
                break;
            }
        }
        if options.len() == 0 {
            None
        } else if options.len() == 1 {
            match options[0] {
                TokenStatement::Any(s) => Some(s),
                TokenStatement::EoS(s) => Some(s),
            }
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
                    // if another stmt FIRST is encountered, break ignore if parsing sub stmt
                    if is_parsing_sub_stmt == false && self.at_stmt_start().is_some() {
                        break;
                    } else {
                        self.advance();
                    }
                }
            }
        }

        self.expect(SyntaxKind::Ascii59);

        // get buffered tokens and pass to parsing fn
        self.finish_node();
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    use crate::lexer::lex;

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_playground() {
        init();

        let input = "alter table test rename column x to y; alter table test rename to test2";
        let parsed = pg_query::parse(input).unwrap();
        let scanned = pg_query::scan(input).unwrap();

        STATEMENT_START_TOKEN_MAPS.iter().for_each(|it| {
            //
        });
        println!("{:#?}", parsed.protobuf.nodes());
        println!("{:#?}", scanned.tokens);
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
