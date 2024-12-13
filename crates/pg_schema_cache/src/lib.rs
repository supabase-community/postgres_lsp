//! The schema cache

#![allow(dead_code)]

mod diagnostics;
mod functions;
mod schema_cache;
mod schemas;
mod tables;
mod types;
mod versions;

pub use diagnostics::SchemaCacheError;
pub use functions::{Behavior, Function, FunctionArg, FunctionArgs};
pub use schema_cache::SchemaCache;
pub use tables::{ReplicaIdentity, Table};
