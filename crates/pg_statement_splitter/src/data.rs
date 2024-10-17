use pg_lexer::SyntaxKind;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug)]
pub enum SyntaxDefinition {
    RequiredToken(SyntaxKind),                    // A single required token
    OptionalToken(SyntaxKind),                    // A single optional token
    OptionalGroup(Vec<SyntaxKind>), // A group of tokens that are required if the group is present
    AnyToken,                       // Any single token
    AnyTokens(Option<Vec<SyntaxKind>>), // A sequence of 0 or more tokens, of which any can be present
    OneOf(Vec<SyntaxKind>),             // One of the specified tokens
    OptionalRepeatedGroup(Vec<SyntaxDefinition>), // A group of tokens that can be repeated
}

impl SyntaxDefinition {
    pub fn is_group(&self) -> bool {
        match self {
            SyntaxDefinition::OptionalGroup(_) => true,
            SyntaxDefinition::OptionalRepeatedGroup(_) => true,
            _ => false,
        }
    }

    pub fn first_required_tokens(&self) -> Vec<&SyntaxKind> {
        match self {
            SyntaxDefinition::RequiredToken(k) => vec![k],
            SyntaxDefinition::OneOf(kinds) => kinds.iter().collect(),
            _ => vec![],
        }
    }
}

#[derive(Debug)]
pub struct SyntaxBuilder {
    parts: Vec<SyntaxDefinition>,
    is_complete: bool,
}

impl SyntaxBuilder {
    // Start a new builder, which will automatically create a Group
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            is_complete: false,
        }
    }

    pub fn new_complete() -> Self {
        Self {
            parts: Vec::new(),
            is_complete: true,
        }
    }

    pub fn any_token(mut self) -> Self {
        self.parts.push(SyntaxDefinition::AnyToken);
        self
    }

    /// The name of an object is almost always an `Ident` token, but due to naming conflicts it can
    /// also be a set of other tokens. This function adds those tokens to the list of possible
    /// tokens.
    pub fn ident_like(mut self) -> Self {
        self.parts.push(SyntaxDefinition::OneOf(vec![
            SyntaxKind::Ident,
            SyntaxKind::VersionP,
            SyntaxKind::Cursor,
            SyntaxKind::Simple,
            SyntaxKind::Set,
            SyntaxKind::Leakproof,
        ]));
        self
    }

    pub fn any_tokens(mut self, tokens: Option<Vec<SyntaxKind>>) -> Self {
        self.parts.push(SyntaxDefinition::AnyTokens(tokens));
        self
    }

    pub fn required_token(mut self, token: SyntaxKind) -> Self {
        self.parts.push(SyntaxDefinition::RequiredToken(token));
        self
    }

    pub fn optional_token(mut self, token: SyntaxKind) -> Self {
        self.parts.push(SyntaxDefinition::OptionalToken(token));
        self
    }

    pub fn optional_schema_name_group(self) -> Self {
        self.optional_group(vec![SyntaxKind::Ident, SyntaxKind::Ascii46])
    }

    pub fn optional_if_exists_group(self) -> Self {
        self.optional_group(vec![SyntaxKind::IfP, SyntaxKind::Exists])
    }

    pub fn optional_if_not_exists_group(self) -> Self {
        self.optional_group(vec![SyntaxKind::IfP, SyntaxKind::Not, SyntaxKind::Exists])
    }

    pub fn optional_or_replace_group(self) -> Self {
        self.optional_group(vec![SyntaxKind::Or, SyntaxKind::Replace])
    }

    pub fn one_of(mut self, tokens: Vec<SyntaxKind>) -> Self {
        self.parts.push(SyntaxDefinition::OneOf(tokens));
        self
    }

    pub fn optional_group(mut self, tokens: Vec<SyntaxKind>) -> Self {
        self.parts.push(SyntaxDefinition::OptionalGroup(tokens));
        self
    }

    pub fn optional_repeated_group(mut self, builder: SyntaxBuilder) -> Self {
        let res = builder.build();
        match res.first() {
            Some(SyntaxDefinition::RequiredToken(_)) => {}
            Some(SyntaxDefinition::OneOf(_)) => {}
            _ => panic!("First token in repeated group must be required or one of"),
        }
        self.parts
            .push(SyntaxDefinition::OptionalRepeatedGroup(res));
        self
    }

    pub fn cte(mut self) -> Self {
        self.parts.extend(
            SyntaxBuilder::new()
                .required_token(SyntaxKind::With)
                .optional_token(SyntaxKind::Recursive)
                .ident_like()
                .required_token(SyntaxKind::As)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41)
                .optional_repeated_group(
                    SyntaxBuilder::new()
                        .required_token(SyntaxKind::Ascii44)
                        .ident_like()
                        .required_token(SyntaxKind::As)
                        .required_token(SyntaxKind::Ascii40)
                        .any_tokens(None)
                        .required_token(SyntaxKind::Ascii41),
                )
                .build(),
        );
        self
    }

    pub fn build(mut self) -> Vec<SyntaxDefinition> {
        if !self.is_complete {
            self.parts.push(SyntaxDefinition::AnyTokens(None));
        } else {
            self.parts
                .push(SyntaxDefinition::OptionalToken(SyntaxKind::Ascii59));
        }
        self.parts
    }
}

#[derive(Debug)]
pub struct StatementDefinition {
    pub stmt: SyntaxKind,
    pub tokens: Vec<SyntaxDefinition>,
    pub prohibited_following_statements: Vec<SyntaxKind>,
    pub prohibited_tokens: Vec<SyntaxKind>,
    pub ignore_if_prohibited: bool,
}

impl StatementDefinition {
    fn new(stmt: SyntaxKind, b: SyntaxBuilder) -> Self {
        Self {
            stmt,
            tokens: b.build(),
            prohibited_following_statements: Vec::new(),
            prohibited_tokens: Vec::new(),
            ignore_if_prohibited: false,
        }
    }

    fn with_prohibited_tokens(mut self, prohibited: Vec<SyntaxKind>) -> Self {
        self.prohibited_tokens = prohibited;
        self
    }

    fn with_prohibited_following_statements(mut self, prohibited: Vec<SyntaxKind>) -> Self {
        self.prohibited_following_statements = prohibited;
        self
    }

    fn with_ignore_if_prohibited(mut self) -> Self {
        self.ignore_if_prohibited = true;
        self
    }
}

pub static STATEMENT_BRIDGE_DEFINITIONS: LazyLock<HashMap<SyntaxKind, Vec<StatementDefinition>>> =
    LazyLock::new(|| {
        let mut m: Vec<StatementDefinition> = Vec::new();

        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Intersect)
                .optional_token(SyntaxKind::All),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Union)
                .optional_token(SyntaxKind::All),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Except)
                .optional_token(SyntaxKind::All),
        ));

        let mut stmt_starts: HashMap<SyntaxKind, Vec<StatementDefinition>> = HashMap::new();

        for stmt in m {
            let first_token = stmt
                .tokens
                .first()
                .expect("Expected first token to be present");

            if let SyntaxDefinition::RequiredToken(token) = first_token {
                stmt_starts.entry(*token).or_insert(Vec::new()).push(stmt);
            } else {
                panic!("Expected first token to be a required token");
            }
        }

        stmt_starts
    });

pub static STATEMENT_DEFINITIONS: LazyLock<HashMap<SyntaxKind, Vec<StatementDefinition>>> =
    LazyLock::new(|| {
        let mut m: Vec<StatementDefinition> = Vec::new();

        m.push(StatementDefinition::new(
            SyntaxKind::CreateTrigStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_token(SyntaxKind::Or)
                .optional_token(SyntaxKind::Replace)
                .optional_token(SyntaxKind::Constraint)
                .required_token(SyntaxKind::Trigger)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::On)
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::Execute)
                .one_of(vec![SyntaxKind::Function, SyntaxKind::Procedure])
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Select)
                .any_token(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .cte()
                .required_token(SyntaxKind::Select)
                .any_token(),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::SelectStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Ascii40)
                    .required_token(SyntaxKind::Select)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Ascii41)
                    .any_tokens(Some(vec![
                        SyntaxKind::Union,
                        SyntaxKind::Except,
                        SyntaxKind::Intersect,
                        SyntaxKind::All,
                    ]))
                    .required_token(SyntaxKind::Ascii40)
                    .required_token(SyntaxKind::Select)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Ascii41)
                    .optional_repeated_group(
                        SyntaxBuilder::new()
                            .one_of(vec![
                                SyntaxKind::Union,
                                SyntaxKind::Except,
                                SyntaxKind::Intersect,
                            ])
                            .optional_token(SyntaxKind::All)
                            .required_token(SyntaxKind::Ascii40)
                            .required_token(SyntaxKind::Select)
                            .any_tokens(None)
                            .required_token(SyntaxKind::Ascii41),
                    ),
            )
            .with_ignore_if_prohibited(),
        );

        // // "TABLE t1;"
        // // is syntactic sugar for "SELECT * FROM t1"
        // m.push(
        //     StatementDefinition::new(
        //         SyntaxKind::SelectStmt,
        //         // we use "new_complete" here
        //         SyntaxBuilder::new_complete()
        //             .required_token(SyntaxKind::Table)
        //             .optional_schema_name_group()
        //             .ident_like(),
        //     )
        //     // this pollutes the "prohibited following statements" logic too much
        //     // so we need to ignore it as a prohibited statement
        //     .with_ignore_if_prohibited(),
        // );

        // VALUES is also legal as a standalone query
        // e.g. VALUES (1,2), (3,4+4), (7,77.7);
        // todo use repeated group
        m.push(StatementDefinition::new(
            SyntaxKind::SelectStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Values)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::InsertStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Insert)
                    .required_token(SyntaxKind::Into)
                    .optional_schema_name_group()
                    .ident_like(),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::SelectStmt,
                SyntaxKind::VariableSetStmt,
            ]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::DeleteStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::DeleteP)
                .required_token(SyntaxKind::From)
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::UpdateStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Update)
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .ident_like()
                .any_tokens(None)
                .required_token(SyntaxKind::Set)
                .any_token(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::MergeStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Merge)
                .required_token(SyntaxKind::Into)
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterTableStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .optional_token(SyntaxKind::Materialized)
                    .optional_token(SyntaxKind::Foreign)
                    .one_of(vec![SyntaxKind::Table, SyntaxKind::Index, SyntaxKind::View])
                    .optional_if_exists_group()
                    .optional_token(SyntaxKind::Only)
                    .optional_schema_name_group()
                    .ident_like()
                    .any_token(),
            )
            .with_prohibited_tokens(vec![SyntaxKind::Rename]),
        );

        // no idea why this is an AlterTableStmt
        m.push(StatementDefinition::new(
            SyntaxKind::AlterTableStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Sequence)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .ident_like()
                .required_token(SyntaxKind::Set)
                .one_of(vec![SyntaxKind::Logged, SyntaxKind::Unlogged]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::RenameStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .any_tokens(None)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::Rename)
                .required_token(SyntaxKind::To)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::RenameStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Table)
                .optional_if_exists_group()
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Rename),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterDomainStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::DomainP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CallStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Call)
                .optional_schema_name_group()
                .ident_like()
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterDefaultPrivilegesStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .required_token(SyntaxKind::Default)
                    .required_token(SyntaxKind::Privileges),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::GrantStmt]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::ClusterStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Cluster),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CopyStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Copy),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ExecuteStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Execute)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::CreateStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .any_tokens(Some(vec![
                        SyntaxKind::Global,
                        SyntaxKind::Local,
                        SyntaxKind::Temporary,
                        SyntaxKind::Temp,
                        SyntaxKind::Unlogged,
                    ]))
                    .required_token(SyntaxKind::Table)
                    .optional_if_not_exists_group()
                    .optional_schema_name_group()
                    .ident_like(),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::TransactionStmt])
            .with_prohibited_tokens(vec![SyntaxKind::As]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::DefineStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_token(SyntaxKind::Or)
                .optional_token(SyntaxKind::Replace)
                .required_token(SyntaxKind::Aggregate),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateOpClassStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Operator)
                .required_token(SyntaxKind::Class)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .optional_token(SyntaxKind::Default)
                .required_token(SyntaxKind::For)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::TextP])
                .required_token(SyntaxKind::Using),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .one_of(vec![
                    SyntaxKind::Server,
                    SyntaxKind::Collation,
                    SyntaxKind::ConversionP,
                    SyntaxKind::Extension,
                    SyntaxKind::Aggregate,
                    SyntaxKind::DomainP,
                    SyntaxKind::Sequence,
                    SyntaxKind::Table,
                    SyntaxKind::TypeP,
                    SyntaxKind::Routine,
                    SyntaxKind::Procedure,
                    SyntaxKind::Schema,
                    SyntaxKind::View,
                    SyntaxKind::Language,
                    SyntaxKind::Function,
                ])
                .optional_if_exists_group()
                .optional_schema_name_group()
                .ident_like(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::TextP)
                .required_token(SyntaxKind::Search)
                .one_of(vec![
                    SyntaxKind::Parser,
                    SyntaxKind::Dictionary,
                    SyntaxKind::Template,
                    SyntaxKind::Configuration,
                ])
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Materialized)
                .required_token(SyntaxKind::View)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Event)
                .required_token(SyntaxKind::Trigger)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .optional_token(SyntaxKind::Procedural)
                .required_token(SyntaxKind::Language)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Operator)
                .required_token(SyntaxKind::Class)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Using)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Access)
                .required_token(SyntaxKind::Method)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .one_of(vec![SyntaxKind::Rule, SyntaxKind::Trigger])
                .optional_if_exists_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::On)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::TextP)
                .required_token(SyntaxKind::Search)
                .one_of(vec![
                    SyntaxKind::Template,
                    SyntaxKind::Configuration,
                    SyntaxKind::Parser,
                    SyntaxKind::Dictionary,
                ])
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::Table)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Cast)
                .optional_if_exists_group()
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::As)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::DataP)
                .required_token(SyntaxKind::Wrapper)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Index)
                .optional_token(SyntaxKind::Concurrently)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Operator)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::Op])
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Function)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .ident_like()
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Operator)
                .required_token(SyntaxKind::Family)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Using)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DefineStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::TextP)
                .required_token(SyntaxKind::Search)
                .one_of(vec![
                    SyntaxKind::Dictionary,
                    SyntaxKind::Configuration,
                    SyntaxKind::Template,
                    SyntaxKind::Parser,
                ])
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DefineStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Operator),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DefineStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_or_replace_group()
                .required_token(SyntaxKind::Aggregate)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DefineStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CompositeTypeStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::As),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateEnumStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::As)
                .required_token(SyntaxKind::EnumP),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateRangeStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::As)
                .required_token(SyntaxKind::Range),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TruncateStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Truncate)
                .optional_token(SyntaxKind::Table)
                .optional_token(SyntaxKind::Only)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CommentStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Comment)
                .required_token(SyntaxKind::On)
                .any_tokens(None)
                .required_token(SyntaxKind::Is)
                .one_of(vec![
                    SyntaxKind::Ident,
                    SyntaxKind::Sconst,
                    SyntaxKind::NullP,
                ]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::FetchStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Fetch)
                .any_tokens(None)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::FetchStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Move)
                .any_tokens(None)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VacuumStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Analyze),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::IndexStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_token(SyntaxKind::Unique)
                .required_token(SyntaxKind::Index)
                .any_tokens(None)
                .required_token(SyntaxKind::On)
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::CreateFunctionStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .optional_token(SyntaxKind::Or)
                    .optional_token(SyntaxKind::Replace)
                    .one_of(vec![SyntaxKind::Function, SyntaxKind::Procedure])
                    .any_tokens(None)
                    .required_token(SyntaxKind::Ascii40)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Ascii41),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::TransactionStmt,
                SyntaxKind::VariableSetStmt,
            ]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterFunctionStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .one_of(vec![SyntaxKind::Function, SyntaxKind::Procedure])
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::VariableSetStmt]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::DoStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Do)
                .optional_token(SyntaxKind::Language)
                .optional_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Sconst),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::RuleStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .optional_token(SyntaxKind::Or)
                    .optional_token(SyntaxKind::Replace)
                    .required_token(SyntaxKind::Rule)
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident)
                    .required_token(SyntaxKind::As)
                    .required_token(SyntaxKind::On)
                    .one_of(vec![
                        SyntaxKind::Select,
                        SyntaxKind::Insert,
                        SyntaxKind::Update,
                        SyntaxKind::DeleteP,
                    ])
                    .required_token(SyntaxKind::To)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Do),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::SelectStmt,
                SyntaxKind::InsertStmt,
                SyntaxKind::UpdateStmt,
                SyntaxKind::DeleteStmt,
                SyntaxKind::VariableSetStmt,
            ]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::NotifyStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Notify)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ListenStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Listen)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::UnlistenStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Unlisten)
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::Ascii42]),
        ));

        // DECLARE c CURSOR FOR SELECT ctid,cmin,* FROM combocidtest
        m.push(
            StatementDefinition::new(
                SyntaxKind::DeclareCursorStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Declare)
                    .required_token(SyntaxKind::Ident)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Cursor)
                    .any_tokens(None)
                    .required_token(SyntaxKind::For),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::SelectStmt]),
        );

        // m.push(StatementDefinition::new(
        //     SyntaxKind::DeclareCursorStmt,
        //     SyntaxBuilder::new()
        //         .required_token(SyntaxKind::Declare)
        //         .required_token(SyntaxKind::Ident)
        //         .any_tokens(None)
        //         .required_token(SyntaxKind::Cursor)
        //         .any_tokens(None)
        //         .required_token(SyntaxKind::For)
        //         .one_of(vec![SyntaxKind::Select, SyntaxKind::With])
        //         .any_token(),
        // ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::TransactionStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Release)
                    .optional_token(SyntaxKind::Savepoint)
                    .required_token(SyntaxKind::Ident),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::TransactionStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::TransactionStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Savepoint)
                    .required_token(SyntaxKind::Ident),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::TransactionStmt]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::BeginP),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::EndP),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Prepare)
                .required_token(SyntaxKind::Transaction)
                .required_token(SyntaxKind::Sconst),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Start)
                .required_token(SyntaxKind::Transaction)
                .any_token(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::BeginP)
                .required_token(SyntaxKind::Transaction),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::TransactionStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Commit),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::TransactionStmt,
                SyntaxBuilder::new_complete()
                    .required_token(SyntaxKind::Rollback)
                    .any_tokens(None)
                    .required_token(SyntaxKind::To)
                    .optional_token(SyntaxKind::Savepoint)
                    .required_token(SyntaxKind::Ident),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::TransactionStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::TransactionStmt,
                SyntaxBuilder::new_complete().required_token(SyntaxKind::Rollback),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::TransactionStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::ViewStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .optional_or_replace_group()
                    .optional_token(SyntaxKind::Temporary)
                    .optional_token(SyntaxKind::Temp)
                    .optional_token(SyntaxKind::Recursive)
                    .required_token(SyntaxKind::View)
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident)
                    .any_tokens(None)
                    .required_token(SyntaxKind::As),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::SelectStmt]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::LoadStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Load),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateDomainStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::DomainP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreatedbStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Database)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropdbStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Database)
                .optional_if_exists_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::VacuumStmt,
                SyntaxBuilder::new().required_token(SyntaxKind::Vacuum),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::VacuumStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::CreateTableAsStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .required_token(SyntaxKind::Materialized)
                    .required_token(SyntaxKind::View)
                    .optional_if_not_exists_group()
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident)
                    .any_tokens(None)
                    .required_token(SyntaxKind::As),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::SelectStmt,
                SyntaxKind::ExecuteStmt,
            ]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::CreateTableAsStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .any_tokens(Some(vec![
                        SyntaxKind::Global,
                        SyntaxKind::Local,
                        SyntaxKind::Temporary,
                        SyntaxKind::Temp,
                    ]))
                    .required_token(SyntaxKind::Table)
                    .optional_if_not_exists_group()
                    .optional_schema_name_group()
                    .ident_like()
                    .any_tokens(None)
                    .required_token(SyntaxKind::As)
                    .one_of(vec![
                        SyntaxKind::With,
                        SyntaxKind::Select,
                        SyntaxKind::Values,
                        SyntaxKind::Table,
                        SyntaxKind::Execute,
                    ]),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::SelectStmt,
                SyntaxKind::ExecuteStmt,
            ]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::ViewStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .optional_token(SyntaxKind::Or)
                    .optional_token(SyntaxKind::Replace)
                    .optional_token(SyntaxKind::Temporary)
                    .optional_token(SyntaxKind::Temp)
                    .optional_token(SyntaxKind::Recursive)
                    .required_token(SyntaxKind::View)
                    .optional_if_not_exists_group()
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident)
                    .any_tokens(None)
                    .required_token(SyntaxKind::As),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::SelectStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::ExplainStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Explain)
                    .any_tokens(None)
                    .one_of(vec![
                        SyntaxKind::Select,
                        SyntaxKind::Insert,
                        SyntaxKind::Update,
                        SyntaxKind::DeleteP,
                        SyntaxKind::Merge,
                        SyntaxKind::Execute,
                        SyntaxKind::Create,
                        SyntaxKind::Declare,
                        SyntaxKind::Create,
                    ]),
            )
            .with_prohibited_following_statements(vec![
                // SyntaxKind::VacuumStmt,
                // SyntaxKind::SelectStmt,
                // SyntaxKind::CreateTableAsStmt,
                // SyntaxKind::InsertStmt,
                // SyntaxKind::DeleteStmt,
                // SyntaxKind::UpdateStmt,
                // SyntaxKind::MergeStmt,
                // SyntaxKind::ExecuteStmt,
                // SyntaxKind::CreateStmt,
                // SyntaxKind::DeclareCursorStmt,
                // todo remove this again when we include all deps
                SyntaxKind::VariableSetStmt,
            ]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::CreateSeqStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .any_tokens(Some(vec![
                    SyntaxKind::Temporary,
                    SyntaxKind::Temp,
                    SyntaxKind::Unlogged,
                ]))
                .required_token(SyntaxKind::Sequence)
                .optional_if_not_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterSeqStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Sequence)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Reset)
                .one_of(vec![SyntaxKind::All, SyntaxKind::Ident, SyntaxKind::Role]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Reset)
                .required_token(SyntaxKind::Session)
                .required_token(SyntaxKind::Authorization),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .required_token(SyntaxKind::Transaction),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .required_token(SyntaxKind::Role)
                .required_token(SyntaxKind::Ident),
        ));

        // ref: https://www.postgresql.org/docs/current/sql-set-session-authorization.html
        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .optional_token(SyntaxKind::Local)
                .required_token(SyntaxKind::Session)
                .required_token(SyntaxKind::Authorization)
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::Sconst]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .any_tokens(Some(vec![
                    SyntaxKind::Local,
                    SyntaxKind::Session,
                    SyntaxKind::Ident,
                    SyntaxKind::Ascii46,
                ]))
                .one_of(vec![SyntaxKind::To, SyntaxKind::Ascii61])
                .any_token(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .optional_token(SyntaxKind::Session)
                .optional_token(SyntaxKind::Local)
                .required_token(SyntaxKind::Time)
                .required_token(SyntaxKind::Zone)
                .any_token(),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::VariableShowStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Show)
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::All]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DiscardStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Discard)
                .one_of(vec![
                    SyntaxKind::All,
                    SyntaxKind::Plans,
                    SyntaxKind::Sequences,
                    SyntaxKind::Temp,
                    SyntaxKind::Temporary,
                ]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateRoleStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .one_of(vec![SyntaxKind::Role, SyntaxKind::GroupP, SyntaxKind::User])
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterRoleStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .one_of(vec![SyntaxKind::Role, SyntaxKind::User])
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterRoleSetStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .required_token(SyntaxKind::Role)
                    .required_token(SyntaxKind::Ident)
                    .required_token(SyntaxKind::Set),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::VariableSetStmt]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::DropRoleStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .one_of(vec![SyntaxKind::Role, SyntaxKind::User, SyntaxKind::GroupP])
                .optional_token(SyntaxKind::IfP)
                .optional_token(SyntaxKind::Exists)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::LockStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::LockP)
                .optional_token(SyntaxKind::Table)
                .optional_token(SyntaxKind::Only)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ConstraintsSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Set)
                .required_token(SyntaxKind::Constraints),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ReindexStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Reindex)
                .any_tokens(None)
                .one_of(vec![
                    SyntaxKind::Table,
                    SyntaxKind::Index,
                    SyntaxKind::Schema,
                ])
                .optional_token(SyntaxKind::Concurrently)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ReindexStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Reindex)
                .any_tokens(None)
                .one_of(vec![SyntaxKind::Database, SyntaxKind::SystemP]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CheckPointStmt,
            SyntaxBuilder::new().required_token(SyntaxKind::Checkpoint),
        ));

        // CREATE TABLE, CREATE VIEW, CREATE INDEX, CREATE SEQUENCE, CREATE TRIGGER and GRANT
        m.push(
            StatementDefinition::new(
                SyntaxKind::CreateSchemaStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Create)
                    .required_token(SyntaxKind::Schema),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::CreateTableAsStmt,
                SyntaxKind::CreateStmt,
                SyntaxKind::SelectStmt,
                SyntaxKind::IndexStmt,
                SyntaxKind::CreateSeqStmt,
                SyntaxKind::CreateTrigStmt,
                SyntaxKind::GrantStmt,
            ]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::AlterDatabaseStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Database)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterDatabaseRefreshCollStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Database)
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Refresh)
                .required_token(SyntaxKind::Collation)
                .required_token(SyntaxKind::VersionP),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterDatabaseSetStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Database)
                .required_token(SyntaxKind::Ident)
                .one_of(vec![SyntaxKind::Set, SyntaxKind::Reset]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateConversionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_token(SyntaxKind::Default)
                .required_token(SyntaxKind::ConversionP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::For)
                .required_token(SyntaxKind::Sconst)
                .required_token(SyntaxKind::To)
                .required_token(SyntaxKind::Sconst)
                .required_token(SyntaxKind::From)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateCastStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Cast)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::As)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateOpFamilyStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Operator)
                .required_token(SyntaxKind::Family)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterOpFamilyStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .required_token(SyntaxKind::Operator)
                    .required_token(SyntaxKind::Family)
                    .optional_schema_name_group()
                    .required_token(SyntaxKind::Ident)
                    .required_token(SyntaxKind::Using)
                    .required_token(SyntaxKind::Ident)
                    .one_of(vec![SyntaxKind::Drop, SyntaxKind::AddP, SyntaxKind::Rename]),
            )
            .with_prohibited_tokens(vec![SyntaxKind::Rename]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::PrepareStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Prepare)
                    .required_token(SyntaxKind::Ident)
                    .any_tokens(None)
                    .required_token(SyntaxKind::As)
                    .any_token(),
            )
            .with_prohibited_following_statements(vec![
                SyntaxKind::SelectStmt,
                SyntaxKind::InsertStmt,
                SyntaxKind::UpdateStmt,
                SyntaxKind::DeleteStmt,
            ]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::ClosePortalStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Close)
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::All]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DeallocateStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Deallocate)
                .optional_token(SyntaxKind::Prepare)
                .one_of(vec![SyntaxKind::Ident, SyntaxKind::All]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateTableSpaceStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Tablespace)
                .any_tokens(None)
                .required_token(SyntaxKind::Location),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropTableSpaceStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Tablespace)
                .optional_if_exists_group()
                .optional_token(SyntaxKind::IfP)
                .optional_token(SyntaxKind::Exists)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterOperatorStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Operator),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterTypeStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::TypeP)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropOwnedStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Owned)
                .required_token(SyntaxKind::By),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ReassignOwnedStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Reassign)
                .required_token(SyntaxKind::Owned)
                .required_token(SyntaxKind::By)
                .any_tokens(None)
                .required_token(SyntaxKind::To),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateFdwStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::DataP)
                .required_token(SyntaxKind::Wrapper)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterFdwStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .required_token(SyntaxKind::Foreign)
                    .required_token(SyntaxKind::DataP)
                    .required_token(SyntaxKind::Wrapper)
                    .required_token(SyntaxKind::Ident),
            )
            .with_prohibited_tokens(vec![SyntaxKind::Rename]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::CreateForeignServerStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Server)
                .optional_if_not_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::DataP)
                .required_token(SyntaxKind::Wrapper)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterForeignServerStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Server)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateUserMappingStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::User)
                .required_token(SyntaxKind::Mapping)
                .optional_if_not_exists_group()
                .required_token(SyntaxKind::For)
                .any_tokens(None)
                .required_token(SyntaxKind::Server)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterUserMappingStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::User)
                .required_token(SyntaxKind::Mapping)
                .optional_token(SyntaxKind::For)
                .any_tokens(None)
                .required_token(SyntaxKind::Server)
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Options),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropUserMappingStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::User)
                .required_token(SyntaxKind::Mapping)
                .optional_if_exists_group()
                .optional_token(SyntaxKind::For)
                .any_tokens(None)
                .required_token(SyntaxKind::Server)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::SecLabelStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Security)
                .required_token(SyntaxKind::Label)
                .optional_token(SyntaxKind::For)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::On),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateForeignTableStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::Table)
                .optional_if_not_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::Server)
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::ImportForeignSchemaStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::ImportP)
                .required_token(SyntaxKind::Foreign)
                .required_token(SyntaxKind::Schema)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::From)
                .required_token(SyntaxKind::Server)
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Into)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateExtensionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Extension)
                .optional_if_not_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterExtensionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Extension)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateEventTrigStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Event)
                .required_token(SyntaxKind::Trigger)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::On)
                .required_token(SyntaxKind::Ident)
                .any_tokens(None)
                .required_token(SyntaxKind::Execute)
                .one_of(vec![SyntaxKind::Function, SyntaxKind::Procedure])
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Ascii40)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterEventTrigStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Event)
                .required_token(SyntaxKind::Trigger)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::RefreshMatViewStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Refresh)
                .required_token(SyntaxKind::Materialized)
                .required_token(SyntaxKind::View)
                .optional_token(SyntaxKind::Concurrently)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterSystemStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::SystemP)
                .one_of(vec![SyntaxKind::Set, SyntaxKind::Reset]),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreatePolicyStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Policy)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::On)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterPolicyStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Policy)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::On)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateTransformStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_or_replace_group()
                .required_token(SyntaxKind::Transform)
                .required_token(SyntaxKind::For)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Language)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Ascii40)
                .any_tokens(None)
                .required_token(SyntaxKind::Ascii41),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateAmStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Access)
                .required_token(SyntaxKind::Method)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::TypeP),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreatePublicationStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Publication)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterPublicationStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Publication)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateSubscriptionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Subscription)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident)
                .required_token(SyntaxKind::Connection)
                .required_token(SyntaxKind::Sconst)
                .required_token(SyntaxKind::Publication)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::AlterSubscriptionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .required_token(SyntaxKind::Subscription)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::DropSubscriptionStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Drop)
                .required_token(SyntaxKind::Subscription)
                .optional_if_exists_group()
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::GrantStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Grant)
                .any_tokens(None)
                .required_token(SyntaxKind::On)
                .any_tokens(None)
                .required_token(SyntaxKind::To),
        ));

        m.push(
            StatementDefinition::new(
                SyntaxKind::GrantStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Revoke)
                    .any_tokens(None)
                    .required_token(SyntaxKind::On),
            )
            .with_prohibited_following_statements(vec![SyntaxKind::SelectStmt]),
        );

        m.push(
            StatementDefinition::new(
                SyntaxKind::AlterOwnerStmt,
                SyntaxBuilder::new()
                    .required_token(SyntaxKind::Alter)
                    .any_tokens(None)
                    .required_token(SyntaxKind::Owner)
                    .required_token(SyntaxKind::To)
                    .required_token(SyntaxKind::Ident),
            )
            // dont ask why, but it seems like tables are special
            // and altering their owner is an AlterTableStmt
            .with_prohibited_tokens(vec![SyntaxKind::Table]),
        );

        m.push(StatementDefinition::new(
            SyntaxKind::AlterObjectSchemaStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Alter)
                .any_tokens(None)
                .required_token(SyntaxKind::Set)
                .required_token(SyntaxKind::Schema)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreatePlangStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .optional_or_replace_group()
                .optional_token(SyntaxKind::Trusted)
                .optional_token(SyntaxKind::Procedural)
                .required_token(SyntaxKind::Language)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        m.push(StatementDefinition::new(
            SyntaxKind::CreateStatsStmt,
            SyntaxBuilder::new()
                .required_token(SyntaxKind::Create)
                .required_token(SyntaxKind::Statistics)
                .any_tokens(None)
                .required_token(SyntaxKind::On)
                .any_tokens(None)
                .required_token(SyntaxKind::From)
                .optional_schema_name_group()
                .required_token(SyntaxKind::Ident),
        ));

        let mut stmt_starts: HashMap<SyntaxKind, Vec<StatementDefinition>> = HashMap::new();

        for stmt in m {
            let first_token = stmt
                .tokens
                .first()
                .expect("Expected first token to be present");

            if let SyntaxDefinition::RequiredToken(token) = first_token {
                stmt_starts.entry(*token).or_insert(Vec::new()).push(stmt);
            } else {
                panic!("Expected first token to be a required token");
            }
        }

        stmt_starts
    });
