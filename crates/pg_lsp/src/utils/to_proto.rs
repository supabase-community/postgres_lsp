use pg_workspace::diagnostics::{Diagnostic, Severity};
use tower_lsp::lsp_types;

pub fn diagnostic(diagnostic: Diagnostic, range: lsp_types::Range) -> lsp_types::Diagnostic {
    let severity = match diagnostic.severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
        Severity::Information => lsp_types::DiagnosticSeverity::INFORMATION,
        Severity::Hint => lsp_types::DiagnosticSeverity::HINT,
        Severity::Fatal => lsp_types::DiagnosticSeverity::ERROR,
    };

    lsp_types::Diagnostic {
        severity: Some(severity),
        source: Some(diagnostic.source),
        ..lsp_types::Diagnostic::new_simple(range, diagnostic.message)
    }
}
