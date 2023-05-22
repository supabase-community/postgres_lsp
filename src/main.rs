use logos::Logos;
use std::fs;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
enum Token {
    #[regex("[a-zA-Z0-9_]+[^;]*;"gm)]
    Expr,
    #[regex("\n+"gm)]
    Newline,
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/|--[^\n]*"g)]
    Comment,
}

fn main() {
    let source = fs::read_to_string("./src/example.sql").unwrap();
    let mut lex = Token::lexer(&source);

    println!("{:?}", source);

    // https://github.com/domenicquirl/cstree
    // https://ericlippert.com/2012/06/08/red-green-trees/
    //
    // So, for example, to parse a struct definition the parser first "enters" the struct definition node, then parses the struct keyword and type name, then parses each field, and finally "finishes" parsing the struct node.
    //
    // 1. lexer: parse string into tokens. cstree will allow us to just move forward until next
    //    statement. also, for comments, we should be able to store them separately since we are
    //    just walking over the source code. tokens should be expr, whitespace, newlines, comments
    //    and eof. does not work because lexer is "dumb". Token != SyntaxKind, so maybe we do not
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
