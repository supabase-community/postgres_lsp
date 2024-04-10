#![feature(extract_if, lazy_cell, test)]

mod change;
mod diagnostics;
mod document;
mod path;

pub use change::{Change, ChangedStatement, DocumentChange, StatementChange};
pub use diagnostics::{Diagnostic, DiagnosticSource, Severity};
pub use document::{Document, DocumentParams, StatementRef};
pub use path::PgLspPath;
