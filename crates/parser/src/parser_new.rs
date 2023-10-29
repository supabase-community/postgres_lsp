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

static STATEMENTS: LazyLock<HashMap<SyntaxKind, &'static [SyntaxKind]>> = LazyLock::new(|| {
    let mut m: HashMap<SyntaxKind, &'static [SyntaxKind]> = HashMap::new();
    m.insert(
        SyntaxKind::InsertStmt,
        &[SyntaxKind::Insert, SyntaxKind::Into],
    );
    m.insert(
        SyntaxKind::DeleteStmt,
        &[SyntaxKind::DeleteP, SyntaxKind::From],
    );
    m.insert(SyntaxKind::UpdateStmt, &[SyntaxKind::Update]);
    m.insert(
        SyntaxKind::MergeStmt,
        &[SyntaxKind::Merge, SyntaxKind::Into],
    );
    m.insert(SyntaxKind::SelectStmt, &[SyntaxKind::Select]);
    // FIX: alter table vs alter table x rename
    m.insert(
        SyntaxKind::AlterTableStmt,
        &[SyntaxKind::Alter, SyntaxKind::Table],
    );
    // FIX: ALTER TABLE x RENAME TO y
    m.insert(
        SyntaxKind::RenameStmt,
        &[SyntaxKind::Alter, SyntaxKind::Table],
    );

    m.insert(
        SyntaxKind::AlterDomainStmt,
        &[SyntaxKind::Alter, SyntaxKind::DomainP],
    );
    m.insert(
        SyntaxKind::AlterDefaultPrivilegesStmt,
        &[
            SyntaxKind::Alter,
            SyntaxKind::Default,
            SyntaxKind::Privileges,
        ],
    );
    m.insert(SyntaxKind::ClusterStmt, &[SyntaxKind::Cluster]);
    m.insert(SyntaxKind::CopyStmt, &[SyntaxKind::Copy]);
    // FIX: CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE
    // m.insert(
    //     SyntaxKind::CreateStmt,
    //     &[SyntaxKind::Create, SyntaxKind::Table],
    // );

    // FIX: CREATE [ OR REPLACE ] AGGREGATE
    // FIX: DefineStmt has multiple definitions
    m.insert(
        SyntaxKind::DefineStmt,
        &[SyntaxKind::Create, SyntaxKind::Aggregate],
    );
    m.insert(
        SyntaxKind::DefineStmt,
        &[SyntaxKind::Create, SyntaxKind::Operator],
    );
    m.insert(
        SyntaxKind::DefineStmt,
        &[SyntaxKind::Create, SyntaxKind::TypeP],
    );

    m.insert(SyntaxKind::DropStmt, &[SyntaxKind::Drop]);
    m.insert(SyntaxKind::TruncateStmt, &[SyntaxKind::Truncate]);
    m.insert(
        SyntaxKind::CommentStmt,
        &[SyntaxKind::Comment, SyntaxKind::On],
    );
    m.insert(SyntaxKind::FetchStmt, &[SyntaxKind::Fetch]);
    // FIX: CREATE [ UNIQUE ] INDEX
    m.insert(
        SyntaxKind::IndexStmt,
        &[SyntaxKind::Create, SyntaxKind::Index],
    );

    // FIX: CREATE [ OR REPLACE ] FUNCTION
    m.insert(
        SyntaxKind::CreateFunctionStmt,
        &[SyntaxKind::Create, SyntaxKind::Function],
    );
    m.insert(
        SyntaxKind::AlterFunctionStmt,
        &[SyntaxKind::Alter, SyntaxKind::Function],
    );
    m.insert(SyntaxKind::DoStmt, &[SyntaxKind::Do]);

    // FIX: CREATE [ OR REPLACE ] RULE
    m.insert(
        SyntaxKind::RuleStmt,
        &[SyntaxKind::Create, SyntaxKind::Rule],
    );

    m.insert(SyntaxKind::NotifyStmt, &[SyntaxKind::Notify]);
    m.insert(SyntaxKind::ListenStmt, &[SyntaxKind::Listen]);
    m.insert(SyntaxKind::UnlistenStmt, &[SyntaxKind::Unlisten]);

    // FIX: TransactionStmt can be Begin or Commit
    m.insert(SyntaxKind::TransactionStmt, &[SyntaxKind::BeginP]);
    m.insert(SyntaxKind::TransactionStmt, &[SyntaxKind::Commit]);

    // FIX: CREATE [ OR REPLACE ] [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW
    m.insert(
        SyntaxKind::ViewStmt,
        &[SyntaxKind::Create, SyntaxKind::View],
    );

    m.insert(SyntaxKind::LoadStmt, &[SyntaxKind::Load]);

    m
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

static STATEMENT_FIRSTS: LazyLock<Vec<&[SyntaxKind]>> =
    LazyLock::new(|| STATEMENTS.values().cloned().collect());

/// Main parser that exposes the `cstree` api, and collects errors and statements
#[derive(Debug)]
pub struct Parser {
    /// The cst builder
    inner: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    /// The syntax errors accumulated during parsing
    errors: Vec<SyntaxError>,
    /// The pg_query statements representing the abtract syntax tree
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
        if WHITESPACE_TOKENS.contains(&self.nth(0)) {
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

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> SyntaxKind {
        self.tokens
            .get(self.pos + lookahead)
            .map_or(SyntaxKind::Eof, |it| it.kind)
    }

    /// checks if the current token is any of `kinds`
    fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.iter().any(|&it| self.at(it))
    }

    /// checks if the current token is of `kind`
    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0) == kind
    }

    /// like at, but for multiple consecutive tokens
    fn at_all(&self, kinds: &[SyntaxKind]) -> bool {
        kinds
            .iter()
            .enumerate()
            .all(|(idx, &it)| self.nth(idx) == it)
    }

    /// like at_any, but for multiple consecutive tokens
    fn at_any_all(&self, kinds: &Vec<&[SyntaxKind]>) -> bool {
        kinds.iter().any(|&it| self.at_all(it))
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
                format!("Expected {:#?}, found {:#?}", kind, self.nth(0)),
                self.pos,
            );
        }
    }

    pub fn source(&mut self) {
        self.start_node(SyntaxKind::SourceFile);

        while !self.eof() {
            let stm_pos = STATEMENT_FIRSTS.iter().position(|&it| self.at_all(it));
            match stm_pos {
                Some(pos) => {
                    self.any_stmt(*STATEMENTS.keys().nth(pos).unwrap());
                }
                None => self.advance(),
            }
        }

        self.finish_node();
    }

    fn any_stmt(&mut self, kind: SyntaxKind) {
        let starts = STATEMENTS.get(&kind).unwrap();
        assert!(self.at_all(starts));
        self.start_node(kind);

        // somehow buffer the tokens
        starts.iter().for_each(|&it| self.expect(it));

        let mut is_parsing_sub_stmt = false;
        while !self.at(SyntaxKind::Ascii59) && !self.eof() {
            match self.nth(0) {
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
                    if is_parsing_sub_stmt == false && self.at_any_all(&STATEMENT_FIRSTS) {
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
    use crate::lexer::lex;

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_playground() {
        init();

        let input = "BEGIN;
UPDATE accounts SET balance = balance - 100.00
    WHERE name = 'Alice';
-- etc etc
COMMIT;";
        let parsed = pg_query::parse(input).unwrap();
        let scanned = pg_query::scan(input).unwrap();
        println!("{:#?}", parsed.protobuf.nodes());
        println!("{:#?}", scanned.tokens);
    }

    #[test]
    fn test_parser_simple() {
        init();

        let input = "select 1 \n -- some comment \n select 2";

        let mut p = Parser::new(lex(input));
        p.source();
        let result = p.finish();

        dbg!(&result.cst);
        println!("{:#?}", result.errors);
    }
}
