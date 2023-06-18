// TODO
//
// the ast nodes are what is returned by pg_query.rs on expression level
//
// right now, we already kind of build the ast on our way to the cst (see `statement_parser.rs`)
//
// we need to find a way to efficiently build the ast from a cst, and get the cst from an ast
//
// from ast to cst can be done by wrapping the pg_query node in a struct that stores the SyntaxNode
//
// rust analyzer stores the SyntaxNode in the AST node struct for a simple conversion
// open: how to get the ast node from the syntax node?
// rust analyzer uses a `cast` function to simply convert to another enum type,
// but it works differently for us because there is not a 1:1 mapping
//
// ideas:
// - wrap pg_query.rs api to cache conversions using fingerprints
// - build ast separately --> but then we need to implement parsing and re-parsing multiple times:
// bad :(
//
