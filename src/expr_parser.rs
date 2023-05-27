use cstree::build::GreenNodeBuilder;
use cstree::testing::SyntaxNode;
use logos::Lexer;
use logos::Logos;
use pg_query::NodeRef;

use crate::SyntaxKind;

pub fn parse_expr<'builder>(
    expression: &str,
    builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
) -> Result<(), String> {
    let parsed = pg_query::parse(expression);
    match parsed {
        Ok(result) => {
            let mut parser = ExprParser::new(expression, result.protobuf.nodes(), builder);
            parser.parse();
            return Ok(());
        }
        Err(error) => {
            // error parsing query, return early
            // TODO: extract position from error
            return Err(error.to_string());
        }
    }
}

// All non-matches characters will emit an error
// all of those characters can be derived from either the node(s) at their position
// or neighboring nodes
#[derive(Logos, Debug, PartialEq)]
pub enum ExprToken {
    #[token("'")]
    Apostrophe,
    #[token("*")]
    Star,
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

pub struct ExprParser<'input, 'builder> {
    lexer: Lexer<'input, ExprToken>,
    expression: &'input str,
    nodes: Vec<(NodeRef<'input>, i32, pg_query::Context)>,
    builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,

    // the list of syntax errors we've accumulated so far
    errors: Vec<String>,
}

// parser for individual sql expressions, e.g. a select statement
impl<'input, 'builder> ExprParser<'input, 'builder> {
    pub fn new(
        expression: &'input str,
        nodes: Vec<(NodeRef<'input>, i32, pg_query::Context)>,
        builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
    ) -> Self {
        Self {
            lexer: ExprToken::lexer(expression),
            expression,
            nodes,
            builder,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        // get root node
        self.nodes.iter().for_each(|(node, _, _)| {
            println!("####");
            println!("node: {:?}", node);
        });
        let root_node: SyntaxKind = match self.nodes[0].0 {
            NodeRef::SelectStmt(_) => SyntaxKind::SelectStmt,
            _ => return Err("root node is not a select statement".to_string()),
        };
        self.builder.start_node(root_node);

        // parse statement into cstree. why? because later we can simply deparse back into pg_query
        // result by passing the string of the node into pg_query::parse

        // iterate over tokens and check whether there is a pg query node at the position
        // todo: check lsp types and other lsps to check type of e.g. commas in arg lists

        // everything that is not a node is a keyword
        // 2. use simple sub-lexer to get position of whitespaces, newlines, comments, ...
        // 3. walk StatementToken iter, match with pg_query nodes
        // and build CST node

        self.builder.finish_node();
        return Ok(());
    }
}

#[test]
fn test_expr_parser() {
    let input = "select *,test from contact where id = '123';";

    println!("input: {}", input);

    let mut builder = GreenNodeBuilder::new();

    parse_expr(input, &mut builder);

    let (tree, cache) = builder.finish();
    let (tree, interner) = (tree, cache.unwrap().into_interner().unwrap());
    let root = SyntaxNode::<SyntaxKind>::new_root_with_resolver(tree, interner);
    dbg!(root);

    assert_eq!(1, 1);
}
