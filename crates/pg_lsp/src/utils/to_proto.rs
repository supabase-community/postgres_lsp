use pg_base_db::Document;
use pg_diagnostics::Diagnostic;

use super::line_index_ext::LineIndexExt;

pub fn diagnostic(document: &Document, diagnostic: &Diagnostic) -> lsp_types::Diagnostic {
    let severity = match diagnostic.severity {
        pg_diagnostics::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        pg_diagnostics::Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
        pg_diagnostics::Severity::Information => lsp_types::DiagnosticSeverity::INFORMATION,
        pg_diagnostics::Severity::Hint => lsp_types::DiagnosticSeverity::HINT,
        pg_diagnostics::Severity::Fatal => lsp_types::DiagnosticSeverity::ERROR,
    };

    let range = document
        .line_index
        .line_col_lsp_range(diagnostic.range)
        .unwrap();

    lsp_types::Diagnostic {
        severity: Some(severity),
        source: Some(diagnostic.source.clone()),
        ..lsp_types::Diagnostic::new_simple(range, diagnostic.message.clone())
    }
}
