![Postgres Language Server](/docs/images/pls-github.png)

# Postgres Language Server

A Language Server for Postgres. Not SQL with flavors, just Postgres.

> [!WARNING]
> This is in active development and is only ready for collaborators. But we are getting there! You can find the current roadmap and opportunities to contribute in https://github.com/supabase-community/postgres_lsp/issues/136.

## Features

The [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) is an open protocol between code editors and servers to provide code intelligence tools such as code completion and syntax highlighting. This project implements such a language server for Postgres, significantly enhancing the developer experience within your favorite editor by adding:

- Lint
- Hover
- Typechecking
- Syntax Error Diagnostics
- Inlay Hints
- Auto-Completion
- Code actions such as `Execute the statement under the cursor`, or `Execute the current file`
- Formatter
- ... and many more

We plan to support all of the above for SQL and PL/pgSQL function bodies too!

## Motivation

Despite the rising popularity of Postgres, support for the PL/pgSQL in IDEs and editors is limited. While there are some _generic_ SQL Language Servers[^1] offering the Postgres syntax as a "flavor" within the parser, they usually fall short due to the ever-evolving and complex syntax of PostgreSQL. There are a few proprietary IDEs[^2] that work well, but the features are only available within the respective IDE.

This Language Server is designed to support Postgres, and only Postgres. The server uses [libpg_query](https://github.com/pganalyze/libpg_query), both as a git submodule for access to its protobuf file and as the [pg_query](https://crates.io/crates/pg_query/5.0.0) rust crate, therefore leveraging the PostgreSQL source to parse the SQL code reliably. Using Postgres within a Language Server might seem unconventional, but it's the only reliable way of parsing all valid PostgreSQL queries. You can find a longer rationale on why This is the Way™ [here](https://pganalyze.com/blog/parse-postgresql-queries-in-ruby). While libpg_query was built to execute SQL, and not to build a language server, any shortcomings have been successfully mitigated in the `parser` crate. You can read the [commented source code](./crates/parser/src/lib.rs) for more details on the inner workings of the parser.

Once the parser is stable, and a robust and scalable data model is implemented, the language server will not only provide basic features such as semantic highlighting, code completion and syntax error diagnostics, but also serve as the user interface for all the great tooling of the Postgres ecosystem.

## Installation

> [!WARNING]
> This is not ready for production use. Only install this if you want to help with development.

> [!NOTE]
> Interested in setting up a release process and client extensions for Neovim and VS Code? Please check out https://github.com/supabase-community/postgres_lsp/issues/136!

### Neovim

Add the postgres_lsp executable to your path, and add the following to your config to use it.

```lua
local util = require 'lspconfig.util'
local lspconfig = require 'lspconfig'

require('lspconfig.configs').postgres_lsp = {
  default_config = {
    name = 'postgres_lsp',
    cmd = { 'postgres_lsp' },
    filetypes = { 'sql' },
    single_file_support = true,
    root_dir = util.root_pattern 'root-file.txt',
  },
}

lspconfig.postgres_lsp.setup { force_setup = true }
```

### Building from source

You'll need _nightly_ Cargo, Node, and npm installed.

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

You can setup your development environment on [CodeSpaces](https://github.com/features/codespaces).

After your codespace boots up, run the following command in the shell to install Rust:

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Proceed with the rest of the installation as usual.

## Contributors

- [psteinroe](https://github.com/psteinroe) (Maintainer)

## Footnotes

[^1]: Generic SQL Solutions: [sql-language-server](https://github.com/joe-re/sql-language-server), [pgFormatter](https://github.com/darold/pgFormatter/tree/master), [sql-parser-cst](https://github.com/nene/sql-parser-cst)
[^2]: Proprietary IDEs: [DataGrip](https://www.jetbrains.com/datagrip/)
