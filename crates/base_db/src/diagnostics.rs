use text_size::TextRange;

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSource {
    PgQuery,
    Lint,
    Typecheck,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<Kind> {
    pub source: DiagnosticSource,
    pub range: Option<TextRange>,
    pub kind: Kind,
    pub severity: Severity,
}

pub enum PgQueryDiagnostic {
    PgQueryError(parser::pg_query::Error),
}

impl From<parser::pg_query::Error> for Diagnostic<PgQueryDiagnostic> {
    fn from(err: parser::pg_query::Error) -> Self {
        Self {
            source: DiagnosticSource::PgQuery,
            range: None,
            kind: PgQueryDiagnostic::PgQueryError(err),
            severity: Severity::Error,
        }
    }
}
