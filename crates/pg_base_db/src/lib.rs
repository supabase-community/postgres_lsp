//! # pg_base_db
//!
//! This crate implements the basic data structures and implements efficient change management. The idea is to make sure we only recompute what is necessary when a change occurs, and this is done by cutting a sql source into statements, and then applying the changes to individual statements. This way, we can avoid re-parsing the entire sql source when a change occurs.
//!
//! The main data structures are:
//! - `Document`: Represents a sql source file, and contains a list of statements represented by their ranges
//! - `StatementRef`: Represents a reference to a sql statement. This is the primary data structure that is used by higher-level crates to save and retrieve information about a statement.
//! - `DocumentChange`: Contains a list of individual `Change`s, and represents a change to a sql source file. This is used to update a `Document` with a new version of the sql source.
//! - `StatementChange` and `ChangedStatement` are results of applying the change and represent references to the changed statements, including information about the changes that were applied. This is used to invalidate caches and recompute information about the changed statements in higher-level crates.
//!
//! I am not yet 100% happy with this, because when we create a `StatementRef`, the text is cloned from `Document` and included in the Hash. This must be improved by leaving the text in the `Document`, and making the `StatementRef` and actual reference to the text in the `Document`. This will make the `StatementRef` smaller and faster to compare.
//! Additionally, the errors returned by the `pg_statement_splitter::split` are not exposed yet. This must be done to show syntax errors to the user.

#![feature(extract_if, test)]

mod change;
mod document;

pub use change::{Change, ChangedStatement, DocumentChange, StatementChange};
pub use document::{Document, StatementRef};
