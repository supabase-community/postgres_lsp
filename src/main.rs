use cstree::build::GreenNodeBuilder;
use cstree::green::GreenNode;
use cstree::interning::Interner;
use cstree::syntax::SyntaxNode;
use cstree::RawSyntaxKind;
use cstree::Syntax;
use logos::Lexer;
use logos::Logos;
use pg_query::Context;
use pg_query::NodeEnum;
use pg_query::NodeRef;
use std::fs;
use std::iter::Peekable;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex("[a-zA-Z0-9_]+[^;]*;"gm)]
    Expr,
    #[regex("\n+"gm)]
    Newline,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

// All non-matches characters will emit an error
// all of those characters can be derived from either the node(s) at their position
// or neighboring nodes
#[derive(Logos, Debug, PartialEq)]
pub enum StatementToken {
    #[token(")")]
    RParen,
    #[token("(")]
    LParen,
    #[regex("(\\w+)"gm)]
    Keyword,
    #[regex(" +"gm)]
    Whitespace,
    #[regex("\n+"gm)]
    Newline,
    #[regex("\t+"gm)]
    Tab,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

// this can be generated later
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[derive(Syntax)]
pub enum SyntaxKind {
    // custom nodes
    Root,
    Comment,
    Whitespace,
    Newline,
    Keyword, // common syntax for all keywords (select, from, ...)
    // from here copyied from NodeEnum
    Alias,
    RangeVar,
    TableFunc,
    Expr,
    Var,
    Param,
    Aggref,
    GroupingFunc,
    WindowFunc,
    SubscriptingRef,
    FuncExpr,
    NamedArgExpr,
    OpExpr,
    DistinctExpr,
    NullIfExpr,
    ScalarArrayOpExpr,
    BoolExpr,
    SubLink,
    SubPlan,
    AlternativeSubPlan,
    FieldSelect,
    FieldStore,
    RelabelType,
    CoerceViaIo,
    ArrayCoerceExpr,
    ConvertRowtypeExpr,
    CollateExpr,
    CaseExpr,
    CaseWhen,
    CaseTestExpr,
    ArrayExpr,
    RowExpr,
    RowCompareExpr,
    CoalesceExpr,
    MinMaxExpr,
    SqlvalueFunction,
    XmlExpr,
    NullTest,
    BooleanTest,
    CoerceToDomain,
    CoerceToDomainValue,
    SetToDefault,
    CurrentOfExpr,
    NextValueExpr,
    InferenceElem,
    TargetEntry,
    RangeTblRef,
    JoinExpr,
    FromExpr,
    OnConflictExpr,
    IntoClause,
    RawStmt,
    Query,
    InsertStmt,
    DeleteStmt,
    UpdateStmt,
    SelectStmt,
    AlterTableStmt,
    AlterTableCmd,
    AlterDomainStmt,
    SetOperationStmt,
    GrantStmt,
    GrantRoleStmt,
    AlterDefaultPrivilegesStmt,
    ClosePortalStmt,
    ClusterStmt,
    CopyStmt,
    CreateStmt,
    DefineStmt,
    DropStmt,
    TruncateStmt,
    CommentStmt,
    FetchStmt,
    IndexStmt,
    CreateFunctionStmt,
    AlterFunctionStmt,
    DoStmt,
    RenameStmt,
    RuleStmt,
    NotifyStmt,
    ListenStmt,
    UnlistenStmt,
    TransactionStmt,
    ViewStmt,
    LoadStmt,
    CreateDomainStmt,
    CreatedbStmt,
    DropdbStmt,
    VacuumStmt,
    ExplainStmt,
    CreateTableAsStmt,
    CreateSeqStmt,
    AlterSeqStmt,
    VariableSetStmt,
    VariableShowStmt,
    DiscardStmt,
    CreateTrigStmt,
    CreatePlangStmt,
    CreateRoleStmt,
    AlterRoleStmt,
    DropRoleStmt,
    LockStmt,
    ConstraintsSetStmt,
    ReindexStmt,
    CheckPointStmt,
    CreateSchemaStmt,
    AlterDatabaseStmt,
    AlterDatabaseSetStmt,
    AlterRoleSetStmt,
    CreateConversionStmt,
    CreateCastStmt,
    CreateOpClassStmt,
    CreateOpFamilyStmt,
    AlterOpFamilyStmt,
    PrepareStmt,
    ExecuteStmt,
    DeallocateStmt,
    DeclareCursorStmt,
    CreateTableSpaceStmt,
    DropTableSpaceStmt,
    AlterObjectDependsStmt,
    AlterObjectSchemaStmt,
    AlterOwnerStmt,
    AlterOperatorStmt,
    AlterTypeStmt,
    DropOwnedStmt,
    ReassignOwnedStmt,
    CompositeTypeStmt,
    CreateEnumStmt,
    CreateRangeStmt,
    AlterEnumStmt,
    AlterTsdictionaryStmt,
    AlterTsconfigurationStmt,
    CreateFdwStmt,
    AlterFdwStmt,
    CreateForeignServerStmt,
    AlterForeignServerStmt,
    CreateUserMappingStmt,
    AlterUserMappingStmt,
    DropUserMappingStmt,
    AlterTableSpaceOptionsStmt,
    AlterTableMoveAllStmt,
    SecLabelStmt,
    CreateForeignTableStmt,
    ImportForeignSchemaStmt,
    CreateExtensionStmt,
    AlterExtensionStmt,
    AlterExtensionContentsStmt,
    CreateEventTrigStmt,
    AlterEventTrigStmt,
    RefreshMatViewStmt,
    ReplicaIdentityStmt,
    AlterSystemStmt,
    CreatePolicyStmt,
    AlterPolicyStmt,
    CreateTransformStmt,
    CreateAmStmt,
    CreatePublicationStmt,
    AlterPublicationStmt,
    CreateSubscriptionStmt,
    AlterSubscriptionStmt,
    DropSubscriptionStmt,
    CreateStatsStmt,
    AlterCollationStmt,
    CallStmt,
    AlterStatsStmt,
    AExpr,
    ColumnRef,
    ParamRef,
    AConst,
    FuncCall,
    AStar,
    AIndices,
    AIndirection,
    AArrayExpr,
    ResTarget,
    MultiAssignRef,
    TypeCast,
    CollateClause,
    SortBy,
    WindowDef,
    RangeSubselect,
    RangeFunction,
    RangeTableSample,
    RangeTableFunc,
    RangeTableFuncCol,
    TypeName,
    ColumnDef,
    IndexElem,
    Constraint,
    DefElem,
    RangeTblEntry,
    RangeTblFunction,
    TableSampleClause,
    WithCheckOption,
    SortGroupClause,
    GroupingSet,
    WindowClause,
    ObjectWithArgs,
    AccessPriv,
    CreateOpClassItem,
    TableLikeClause,
    FunctionParameter,
    LockingClause,
    RowMarkClause,
    XmlSerialize,
    WithClause,
    InferClause,
    OnConflictClause,
    CommonTableExpr,
    RoleSpec,
    TriggerTransition,
    PartitionElem,
    PartitionSpec,
    PartitionBoundSpec,
    PartitionRangeDatum,
    PartitionCmd,
    VacuumRelation,
    InlineCodeBlock,
    CallContext,
    Integer,
    Float,
    String,
    BitString,
    Null,
    List,
    IntList,
    OidList,
}

pub struct Parser<'input> {
    lexer: Lexer<'input, Token>,
    builder: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    // the list of syntax errors we've accumulated so far
    errors: Vec<String>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: Token::lexer(input),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.builder.start_node(SyntaxKind::Root);
        self.parse_next_token();
        self.builder.finish_node();
        return Ok(());
    }

    fn parse_next_token(&mut self) {
        let token = self.lexer.next();
        println!("token: {:?}", token);
        match token {
            Some(Ok(token)) => {
                match token {
                    Token::Comment => {
                        self.builder.token(SyntaxKind::Comment, self.lexer.slice());
                    }
                    Token::Newline => {
                        self.builder.token(SyntaxKind::Newline, self.lexer.slice());
                    }
                    Token::Expr => {
                        self.parse_expr();
                    }
                };
                self.parse_next_token();
            }
            Some(Err(_)) => {
                self.errors
                    .push(format!("Error parsing token: '{:?}'", token));
                self.parse_next_token();
            }
            None => return,
        };
    }

    fn parse_expr(&mut self) {
        let expr_str = self.lexer.slice();
        println!("expr_str: {:?}", expr_str);

        // 1. parse expr using pg_query
        let parsed = pg_query::parse(expr_str);
        let result = match parsed {
            Ok(result) => {
                result.protobuf.nodes().iter().for_each(|n| {
                    println!("##");
                    println!("node: {:?}", n);
                });
                // everything that is not a node is a keyword
                Some(result)
            }
            Err(e) => {
                // TODO: extract line and column from error
                self.errors.push(e.to_string());
                None
            }
        };

        // 2. use simple sub-lexer to get position of whitespaces, newlines, comments, ...
        let statement_lexer = StatementToken::lexer(expr_str);
        statement_lexer.into_iter().for_each(|t| {
            println!("statement_lexer: {:?}", t);
        });

        // 3. walk StatementToken iter, match with pg_query nodes
        // and build CST node
    }

    pub fn finish(mut self) -> (GreenNode, impl Interner) {
        // assert!(self.lexer.next().map(|t| t == Token::EoF).unwrap_or(true));
        let (tree, cache) = self.builder.finish();
        (tree, cache.unwrap().into_interner().unwrap())
    }
}

fn main() {
    let source = fs::read_to_string("./src/example.sql").unwrap();
    let mut lex = Token::lexer(&source);

    println!("{:?}", source);

    let mut parser = Parser::new(&source);
    parser.parse().unwrap();
    let (tree, interner) = parser.finish();
    let root = SyntaxNode::<SyntaxKind>::new_root_with_resolver(tree, interner);
    dbg!(root);

    // https://github.com/domenicquirl/cstree
    // https://ericlippert.com/2012/06/08/red-green-trees/
    //
    // So, for example, to parse a struct definition the parser first "enters" the struct definition node, then parses the struct keyword and type name, then parses each field, and finally "finishes" parsing the struct node.
    //
    // 1. lexer: parse string into tokens. cstree will allow us to just move forward until next
    //    statement. also, for comments, we should be able to store them separately since we are
    //    just walking over the source code. tokens should be expr, newlines, comments.
    //    does not work because lexer is "dumb". Token != SyntaxKind, so maybe we do not
    //    need a real lexer.
    // 2. parser: parse tokens into cst with cstree. nodes are not typed, and we should be able to
    //    use pg_query to parse string, and turn that into SyntaxKind tokens.
    //
    //
    //    Notes:
    //    - maybe we do not real a lexer to parse into statements. we can just use simple string
    //    operations? or maybe lexer but with metadata on tokens because normally a token
    //    translates into a constant which is not what we want. instead, we want a token Expr to
    //    hold the expression string.

    // problem: comments
    // general problem: declarative parsing by token will, based on initial research, not work well because we have tokens
    // within tokens (comment can be within a sql query)
    // let parser = any::<_, extra::Err<Simple<char>>>()
    //     .and_is(just(';').not())
    //     .repeated()
    //     .collect::<String>()
    //     .padded()
    //     .separated_by(just(';'))
    //     .collect::<Vec<String>>();
    //
    // let comment = just("--")
    //     .then(
    //         any::<_, extra::Err<Simple<char>>>()
    //             .and_is(just('\n').not())
    //             .repeated(),
    //     )
    //     .padded();
    //
    // let comments = comment.parse(source.as_str());
    // let result = parser.parse(source.as_str());
    //
    // println!("{:?}", source);
    // println!("{:?}", result);
    // println!("{:?}", comments);
    //
    // let pg_query_result = pg_query::parse("SELECT * FROM contacts").unwrap();
    //
    // println!("{:?}", pg_query_result.protobuf.nodes());
}

#[test]
fn test_lexer() {
    let input = "select * from contact where id = '123';\n\n-- test comment\n\nselect wrong statement;\n\nselect id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';\n\n";

    let mut lex = Token::lexer(&input);

    assert_eq!(lex.next(), Some(Ok(Token::Expr)));
    assert_eq!(lex.slice(), "select * from contact where id = '123';");

    assert_eq!(lex.next(), Some(Ok(Token::Newline)));

    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.slice(), "-- test comment");

    assert_eq!(lex.next(), Some(Ok(Token::Newline)));

    assert_eq!(lex.next(), Some(Ok(Token::Expr)));
    assert_eq!(lex.slice(), "select wrong statement;");

    assert_eq!(lex.next(), Some(Ok(Token::Newline)));

    assert_eq!(lex.next(), Some(Ok(Token::Expr)));
    assert_eq!(lex.slice(), "select id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';");
}
