use pglt_diagnostics::{Diagnostic, MessageAndDescription};
use text_size::TextRange;

/// A specialized diagnostic for the statement splitter parser.
///
/// Parser diagnostics are always **errors**.
#[derive(Clone, Debug, Diagnostic, PartialEq)]
#[diagnostic(category = "syntax", severity = Error)]
pub struct SplitDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

impl SplitDiagnostic {
    pub fn new(message: impl Into<String>, range: TextRange) -> Self {
        Self {
            span: Some(range),
            message: MessageAndDescription::from(message.into()),
        }
    }
}
