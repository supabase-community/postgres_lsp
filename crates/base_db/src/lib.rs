#![feature(extract_if, lazy_cell, test)]

mod diagnostics;
mod document;
mod document_change;
mod path;
mod utils;

pub use diagnostics::{Diagnostic, DiagnosticSource, Severity};
pub use document::{Document, DocumentParams, StatementRef};
pub use document_change::{Change, DocumentChange, StatementChange};
pub use path::PgLspPath;
