# pg_lexer

The `pg_lexer` crate exposes the `lex` method, which turns an SQL query text into a `Vec<Token>>`: the base for the `pg_parser` and most of pgtools's operations.

A token is always of a certain `SyntaxKind` kind. That `SyntaxKind` enum is derived from `libpg_query`'s protobuf file.

The SQL query text is mostly lexed using the `pg_query::scan` method (`pg_query` is just a Rust wrapper around `libpg_query`).
However, that method does not parse required whitespace tokens, so the `lex` method takes care of parsing those and merging them into the result.
