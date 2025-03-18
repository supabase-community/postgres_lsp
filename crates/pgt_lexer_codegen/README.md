# pgt_lexer_codegen

This crate is responsible for reading `libpg_query`'s protobuf file and turning it into the Rust enum `SyntaxKind`.

It does so by reading the file from the installed git submodule, parsing it with a protobuf parser, and using a procedural macro to generate the enum.

Rust requires procedural macros to be defined in a different crate than where they're used, hence this \_codegen crate.
