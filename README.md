![Postgres Language Server](/docs/images/pls-github.png)

# Postgres Language Server

A Language Server for Postgres. Not SQL with flavors, just Postgres.

## Status

🚧 This is in active development and is only ready for collaborators. The majority of work is still ahead, but we've verified that the approach works. We're making this public so that we can develop it in the open with input from the community.

## Features

The [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) is an open protocol between code editors and servers to provide code intelligence tools such as code completion and syntax highlighting. This project implements such a language server for Postgres, significantly enhancing the developer experience within your favorite editor by adding:

- Semantic Highlighting
- Syntax Error Diagnostics
- Show SQL comments on hover
- Auto-Completion
- Code actions such as `Execute the statement under the cursor`, or `Execute the current file`
- Configurable Code Formatting
- ... and many more

## Motivation

Despite the rising popularity of Postgres, support for the PL/pgSQL in IDEs and editors is limited. While there are some _generic_ SQL Language Servers[^1] offering the Postgres syntax as a "flavor" within the parser, they usually fall short due to the ever-evolving and complex syntax of PostgreSQL. There are a few proprietary IDEs[^2] that work well, but the features are only available within the respective IDE.

This Language Server is designed to support Postgres, and only Postgres. The server uses [libpg_query](https://github.com/pganalyze/libpg_query), therefore leveraging the PostgreSQL source to parse the SQL code reliably. Using Postgres within a Language Server might seem unconventional, but it's the only reliable way of parsing all valid PostgreSQL queries. You can find a longer rationale on why This is the Way™ [here](https://pganalyze.com/blog/parse-postgresql-queries-in-ruby). While libpg_query was built to execute SQL, and not to build a language server, any shortcomings have been successfully mitigated in the `parser` crate. You can read the [commented source code](./crates/parser/src/lib.rs) for more details on the inner workings of the parser.

Once the parser is stable, and a robust and scalable data model is implemented, the language server will not only provide basic features such as semantic highlighting, code completion and syntax error diagnostics, but also serve as the user interface for all the great tooling of the Postgres ecosystem.

## Roadmap

This is a proof of concept for building both a concrete syntax tree and an abstract syntax tree from a potentially malformed PostgreSQL source code. The `postgres_lsp` crate was created to prove that it works end-to-end, and is just a very basic language server with semantic highlighting and error diagnostics. Before further feature development, we have to complete a bit of groundwork:

1. _Finish the parser_
   - ✅ The core parser algorithm is implemented. Check out [this blog post](https://supabase.com/blog/postgres-language-server-implementing-parser) for details.
   - 🚧 But we need help from the community to finalize it: https://github.com/supabase/postgres_lsp/issues/51.
2. _Implement a robust and scalable data model_
   - RFC posted here https://github.com/supabase/postgres_lsp/discussions/107
3. _Setup the language server properly_
   - This is still in a research phase
   - Once again `rust-analyzer` will serve as a role model, and we will most likely implement the same queueing and cancellation approach
4. _Implement basic language server features_
   - Semantic Highlighting
   - Syntax Error Diagnostics
   - Show SQL comments on hover
   - Auto-Completion
   - Code Actions, such as `Execute the statement under the cursor`, or `Execute the current file`
   - ... anything you can think of really
5. _Integrate all the existing open source tooling_
   - Show migration file lint errors from [squawk](https://github.com/sbdchd/squawk)
   - Show plpsql lint errors from [plpgsql_check](https://github.com/okbob/plpgsql_check)
6. _Build missing pieces_
   - An optionated code formatter (think prettier for PostgreSQL)
7. _(Maybe) Support advanced features with declarative schema management_
   - Jump to definition
   - ... anything you can think of really

## Installation

> [!WARNING]
> This is not ready for production use. Only install this if you want to help with development.

### Neovim

Add the postgres_lsp executable to your path, and add the following to your config to use it.

```lua
require('lspconfig.configs').postgres_lsp = {
  default_config = {
    name = 'postgres_lsp',
    cmd = {'postgres_lsp'},
    filetypes = {'sql'},
    single_file_support = true,
    root_dir = util.root_pattern 'root-file.txt'
  }
}

lsp.configure("postgres_lsp", {force_setup = true})
```

### Building from source

You'll need *nightly* Cargo, Node, and npm installed.

Install the `libpg_query` submodule by running:

```sh
git submodule update --init --recursive
```

If you are using VS Code, you can install both the server and the client extension by running:

```sh
cargo xtask install
```

If you're not using VS Code, you can install the server by running:

```sh
cargo xtask install --server
```


The server binary will be installed in `.cargo/bin`. Make sure that `.cargo/bin` is in `$PATH`.

### Github CodeSpaces
Currently, Windows does not support `libpg_query`. You can setup your development environment
on [CodeSpaces](https://github.com/features/codespaces).

After your codespace boots up, run the following command in the shell to install Rust:
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
Proceed with the rest of the installation as usual.

## Contributors

- [psteinroe](https://github.com/psteinroe) (Maintainer)

## Acknowledgments

- [rust-analyzer](https://github.com/rust-lang/rust-analyzer) for implementing such a robust, well documented, and feature-rich language server. Great place to learn from.
- [squawk](https://github.com/sbdchd/squawk) and [pganalyze](https://pganalyze.com) for inspiring the use of libpg_query.

## Footnotes

[^1]: Generic SQL Solutions: [sql-language-server](https://github.com/joe-re/sql-language-server), [pgFormatter](https://github.com/darold/pgFormatter/tree/master), [sql-parser-cst](https://github.com/nene/sql-parser-cst)
[^2]: Proprietary IDEs: [DataGrip](https://www.jetbrains.com/datagrip/)
