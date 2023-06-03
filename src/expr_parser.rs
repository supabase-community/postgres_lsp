use std::iter::Peekable;
use std::ops::Range;
use std::slice::Iter;
use std::vec::IntoIter;

use cstree::build::GreenNodeBuilder;
use cstree::testing::SyntaxNode;
use logos::Lexer;
use logos::Logos;
use pg_query::parse;
use pg_query::protobuf::ParseResult;
use pg_query::protobuf::ScanResult;
use pg_query::protobuf::ScanToken;
use pg_query::protobuf::Token;
use pg_query::NodeEnum;
use pg_query::NodeMut;
use pg_query::NodeRef;

use crate::syntax::convert_expr_token_to_syntax_kind;
use crate::syntax::convert_pg_query_node_to_syntax_kind;
use crate::syntax::convert_pg_query_token_to_syntax_kind;
use crate::syntax::get_position_for_pg_query_node;
use crate::SyntaxKind;

// All non-matches characters will emit an error
// all of those characters can be derived from either the node(s) at their position
// or neighboring nodes
#[derive(Logos, Debug, PartialEq)]
pub enum ExprToken {
    // copied from protobuf::Token. can be generated later
    #[token("%")]
    Ascii37,
    #[token("(")]
    Ascii40,
    #[token(")")]
    Ascii41,
    #[token("*")]
    Ascii42,
    #[token("+")]
    Ascii43,
    #[token(",")]
    Ascii44,
    #[token("-")]
    Ascii45,
    #[token(".")]
    Ascii46,
    #[token("/")]
    Ascii47,
    #[token(":")]
    Ascii58,
    #[token(";")]
    Ascii59,
    #[token("<")]
    Ascii60,
    #[token("=")]
    Ascii61,
    #[token(">")]
    Ascii62,
    #[token("?")]
    Ascii63,
    #[token("[")]
    Ascii91,
    #[token("\\")]
    Ascii92,
    #[token("]")]
    Ascii93,
    #[token("^")]
    Ascii94,
    // comments, whitespaces and keywords
    #[regex("'([^']+)'")]
    Sconst,
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
    curr_depth: i32,
    nodes: Peekable<Iter<'input, (NodeRef<'input>, i32, pg_query::Context)>>,
    tokens: Peekable<Iter<'input, ScanToken>>,
    builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,

    // the list of syntax errors we've accumulated so far
    errors: Vec<String>,
}

pub fn parse_expression<'input, 'builder>(
    expression: &'input str,
    builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
) {
    let parsed = pg_query::parse(expression).unwrap();
    let scan_res = pg_query::scan(expression).unwrap();
    let mut sort_nodes = parsed.protobuf.nodes();
    sort_nodes.sort_by(|a, b| {
        get_position_for_pg_query_node(&a.0)
            .unwrap()
            .cmp(&get_position_for_pg_query_node(&b.0).unwrap())
    });

    sort_nodes.iter().for_each(|node| {
        println!("####");
        println!("{:?}", node);
    });

    ExprParser::new(
        ExprToken::lexer(expression),
        expression,
        builder,
        sort_nodes.iter().peekable(),
        scan_res.tokens.iter().peekable(),
    )
    .unwrap()
    .parse();
}

// parser for individual sql expressions, e.g. a select statement
impl<'input, 'builder> ExprParser<'input, 'builder> {
    pub fn new(
        lexer: Lexer<'input, ExprToken>,
        expression: &'input str,
        builder: &'builder mut GreenNodeBuilder<'static, 'static, SyntaxKind>,
        nodes: Peekable<Iter<'input, (NodeRef<'input>, i32, pg_query::Context)>>,
        tokens: Peekable<Iter<'input, ScanToken>>,
    ) -> Result<Self, String> {
        return Ok(Self {
            lexer,
            expression,
            nodes,
            curr_depth: 1,
            tokens,
            builder,
            errors: Vec::new(),
        });
        // match parsed {
        //     Ok(result) => {
        //         let scan_result = pg_query::scan(expression);
        //         match scan_result {
        //             Ok(scan_res) => {
        //                 return Ok(Self {
        //                     lexer: ExprToken::lexer(expression),
        //                     expression,
        //                     nodes: result.protobuf.nodes().iter().peekable(),
        //                     tokens: scan_res.tokens.iter().peekable(),
        //                     builder,
        //                     errors: Vec::new(),
        //                 });
        //             }
        //             Err(error) => {
        //                 // error parsing query, return early
        //                 // TODO: extract position from error
        //                 return Err(error.to_string());
        //             }
        //         }
        //     }
        //     Err(error) => {
        //         // error parsing query, return early
        //         // TODO: extract position from error
        //         return Err(error.to_string());
        //     }
        // }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        // get root node
        let root_node = self.nodes.next();
        let root_node: SyntaxKind = match root_node.unwrap().0 {
            NodeRef::SelectStmt(_) => SyntaxKind::SelectStmt,
            _ => return Err("root node is not a select statement".to_string()),
        };
        self.builder.start_node(root_node);
        self.parse_token();

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

    fn consume_pg_query_nodes(&mut self) {
        let span = self.lexer.span();
        let node = self.nodes.peek();
        println!("move_node - node: {:?}", node);
        println!("move_node - span: {:?}", span);
        match node {
            Some((node, _, _)) => {
                let pos = get_position_for_pg_query_node(node);
                println!("move_node - pos: {:?}", pos);
                match pos {
                    Some(pos) => {
                        // todo handle error
                        if span.contains(&usize::try_from(pos).unwrap()) {
                            // node is within span
                            let (node, depth, _) = self.nodes.next().unwrap();
                            let s = convert_pg_query_node_to_syntax_kind(&node);
                            println!("move_node - node is within span: {:?}, {:?}", s, depth);
                            println!("move_node - depth {:?} -> {:?}", self.curr_depth, depth);
                            if depth <= &self.curr_depth {
                                // we are going up or stay the same, finish prev node
                                println!("## move_node - finish_node",);
                                self.builder.finish_node();
                            }
                            if depth < &self.curr_depth {
                                // we are going up, finish prev node
                                println!("## move_node - finish_node",);
                                self.builder.finish_node();
                            }

                            self.curr_depth = *depth;
                            println!("## move_node - start node: {:?}", s);
                            self.builder.start_node(s.unwrap());
                            self.consume_pg_query_nodes();
                            return;
                        } else {
                            println!("move_node - node not within span");
                            // node is not within span
                            return;
                        }
                    }
                    None => {
                        println!("move_node - node has no position");
                        // node has no position
                        return;
                    }
                }
            }
            None => {
                // no more nodes
                println!("move_node - no more nodes, depth: {:?}", self.curr_depth);
                return;
            }
        }
    }

    fn consume_pg_query_token(&mut self) -> Option<SyntaxKind> {
        let span = self.lexer.span();
        let token = self.tokens.peek();
        println!("get_token_syntax_in_span - span: {:?}", span);
        println!("get_token_syntax_in_span - token: {:?}", token);
        match token {
            Some(token) => {
                // todo handle error
                if span.contains(&usize::try_from(token.start).unwrap())
                    || span.contains(&usize::try_from(token.end).unwrap())
                {
                    println!("token is within span");
                    // token is within span
                    let token = self.tokens.next().unwrap();
                    return convert_pg_query_token_to_syntax_kind(token);
                } else {
                    println!("token is not within span");
                    // token is not within span
                    return None;
                }
            }
            None => {
                println!("no more tokens");
                // no more tokens
                return None;
            }
        }
    }

    fn parse_token(&mut self) {
        let token = self.lexer.next();
        println!(
            "## parse_token: lexer token: {:?} - {:?}",
            token,
            self.lexer.slice()
        );

        match token {
            Some(token) => {
                // check if current pg_query node is within span
                // this does not work yet :(
                // the pg query node location is not a reliable way of determining when a node
                // starts, and when it ends. As of now, it seems like we would have to manually
                // parse the nodes to properly return the cstree. I dont think this is worth it,
                // since we want to work with the raw nodes anyways.
                // self.consume_pg_query_nodes();

                // check if current pg_query token is within span
                let pg_query_token = self.consume_pg_query_token();
                println!("pg_query_token: {:?}", pg_query_token);
                match pg_query_token {
                    Some(t) => {
                        self.builder.token(t, self.lexer.slice());
                    }
                    None => {
                        // no token found, fallback to exprtoken (e.g. whitespace)
                        // todo: handle error
                        let expr_token =
                            convert_expr_token_to_syntax_kind(&token.unwrap()).unwrap();
                        println!("no token found, fallback to expr_token: {:?}", expr_token);
                        self.builder.token(expr_token, self.lexer.slice());
                    }
                }

                // only if no pg_query token, convert expr token to syntax kind

                // todo: how to know when the current branch is finished?
                // --> USE DEPTH!! (NodeRef, depth, context)
                // todo: move conversions into own file

                self.parse_token();
            }
            None => {
                // end of input
                while self.curr_depth > 1 {
                    println!("end of input - finish_node at {:?}", self.curr_depth);
                    self.builder.finish_node();
                    self.curr_depth -= 1;
                }
                return;
            }
        }
    }
}

#[test]
fn test_expr_parser() {
    let input = "select *,test from contact where (id = '123');";
    // let input = "select test from contact where id = 123;";

    println!("input: {}", input);

    let mut builder = GreenNodeBuilder::new();

    parse_expression(input, &mut builder);

    let (tree, cache) = builder.finish();
    let (tree, interner) = (tree, cache.unwrap().into_interner().unwrap());
    let root = SyntaxNode::<SyntaxKind>::new_root_with_resolver(tree, interner);
    dbg!(root);

    assert_eq!(1, 1);
}
