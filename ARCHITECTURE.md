## Architecture

This document describes the high-level architecture of postgres_lsp. If you want to familiarize yourself with the code base, you are just in the right place!

> Since the project still evolves rapidly, this document may not be up-to-date. If you find any inconsistency, please let us know by creating an issue.

### Bird's Eye View

On the highest level, the postgres language server is a thing which accepts input source code, cuts it into individual sql statements and parses and analyses each. In addition, it connects to a postgres database and stores an im-memory schema cache with all required type information such as tables, columns and functions. The result of the parsing is used alongside the schema cache to answer queries about a statement.

The client can submit a delta of input data (typically, a change to a single file), and the server will update the affected statements and their analysis accordingly. The underlying engine makes sure that we only re-parse and re-analyse what is necessary.

### Entry Points

The main entry point is as of now the `pg_lsp` crate, especially the `main.rs` function. It spawns the language server and starts listening for incoming messages. The server is implemented in the `server` module.

There might be an additional entry point for a CLI tool in the future.

### Code Map

This section talks briefly about various important directories and data structures.

#### `lib/`

Independent libraries that are used by the project but are not specific for postgres.

#### `crates/pg_lsp`

The main entry point of the language server. It contains the server implementation and the main loop.

#### `crates/pg_workspace`

> This crate will grow significantly in near future. The current implementation just contains the base data structures and stores the diagnostic results from various features.

The main API for consumers of the IDE. It stores the internal state of the workspace, such as the schema cache and the parsed statements and their analysis.

#### `crates/pg_lexer`

Simple lexer that tokenizes the input source code. Enhances the output of the `pg_query` tokenizer with the missing whitespace tokens.

#### `crates/pg_statement_splitter`

Implements the statement splitter, which cuts the input source code into individual statements.

#### `crates/pg_base_db`

Implements the base data structures and defines how documents and statements are stored and updated efficiently.

#### `crates/pg_schema_cache`

We store an in-memory representation of the database schema to efficiently resolve types.

#### `crates/pg_query_ext`

Simple wrapper crate for `pg_query` to expose types and a function to get the root node for an SQL statement. It also host any "extensions" to the `pg_query` crate that are not yet contributed upstream. Once all extensions are contributed upstream, this crate will be removed.

#### `crates/pg_query_proto_parser`

We use procedural macros a lot to generate repetitive code from the protobuf definition provided by `libg_query`. The `pg_query_proto_parser` crate is used to parse the proto file into a more usable data structure.

#### `crates/pg_syntax`

Implements the CST parser and AST enhancer. The CST parser is what is described in [this blog post](https://supabase.com/blog/postgres-language-server-implementing-parser). The AST enhancer takes in the CST and enriches the AST returned by `pg_query` with a range for each node.

#### `crates/pg_type_resolver`

Utility crate used by the feature crates listed below to resolve the source types to the actual types in the schema cache.

#### `crates/pg_commands`, `crates/pg_completions`, `crates/pg_hover`, `crates/pg_inlay_hints`, `crates/pg_lint`, `crates/pg_typecheck`

These crates implement the various features of the language server. They are all independent of each other and always operate on the schema cache and a single statement and its parse results. They are intentionally implemented in separate creates and without any language server flavour to make them reusable eg in a later cli.
