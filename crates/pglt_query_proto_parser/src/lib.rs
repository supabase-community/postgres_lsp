//! A parser for the libg_query proto file
//!
//! This crate provides a parser for the libg_query proto file, and a struct to represent and interact with the parsed file.

mod proto_file;
mod proto_parser;

pub use crate::proto_file::{Field, FieldType, Node, ProtoFile, Token};
pub use crate::proto_parser::ProtoParser;
