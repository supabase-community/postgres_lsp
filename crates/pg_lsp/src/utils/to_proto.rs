use pg_diagnostics::Diagnostic;
use tower_lsp::lsp_types;

pub fn diagnostic(diagnostic: Diagnostic, range: lsp_types::Range) -> lsp_types::Diagnostic {
    let severity = match diagnostic.severity {
        pg_diagnostics::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        pg_diagnostics::Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
        pg_diagnostics::Severity::Information => lsp_types::DiagnosticSeverity::INFORMATION,
        pg_diagnostics::Severity::Hint => lsp_types::DiagnosticSeverity::HINT,
        pg_diagnostics::Severity::Fatal => lsp_types::DiagnosticSeverity::ERROR,
    };

    lsp_types::Diagnostic {
        severity: Some(severity),
        source: Some(diagnostic.source),
        ..lsp_types::Diagnostic::new_simple(range, diagnostic.message)
    }
}
