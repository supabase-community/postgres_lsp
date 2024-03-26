#![feature(extract_if, lazy_cell, test)]

mod diagnostics;
mod document;
mod document_change;
mod path;
mod statement;
mod utils;

pub use diagnostics::{Diagnostic, DiagnosticSource, Severity};
pub use document::{Document, DocumentParams};
pub use document_change::{DocumentChange, DocumentChangesParams};
pub use path::PgLspPath;
pub use statement::{Statement, StatementParams};
