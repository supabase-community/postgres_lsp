use std::fmt::Debug;
use text_size::TextRange;

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub description: Option<String>,
    pub severity: Severity,
    pub source: String,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The severity to associate to a diagnostic.
pub enum Severity {
    /// Reports a hint.
    Hint,
    /// Reports an information.
    #[default]
    Information,
    /// Reports a warning.
    Warning,
    /// Reports an error.
    Error,
    /// Reports a crash.
    Fatal,
}
