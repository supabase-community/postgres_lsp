use pglt_diagnostics::{Diagnostic, MessageAndDescription};
use text_size::TextRange;

/// A specialized diagnostic for the statement splitter parser.
///
/// Parser diagnostics are always **errors**.
#[derive(Clone, Debug, Diagnostic, PartialEq)]
#[diagnostic(category = "syntax", severity = Error)]
pub struct ParseDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    pub message: MessageAndDescription,
    // if true, the error is fatal and the parsing should stop
    pub is_fatal: bool,
}

impl ParseDiagnostic {
    pub fn new(message: impl Into<String>, range: TextRange) -> Self {
        Self {
            span: Some(range),
            message: MessageAndDescription::from(message.into()),
            is_fatal: false,
        }
    }

    pub fn from_pg_query_err(err: pglt_query_ext::Error, input: &str) -> Vec<Self> {
        let err_msg = err.to_string();
        let re = regex::Regex::new(r#"at or near "(.*?)""#).unwrap();
        let mut diagnostics = Vec::new();

        for captures in re.captures_iter(&err_msg) {
            if let Some(matched) = captures.get(1) {
                let search_term = matched.as_str();
                for (idx, _) in input.match_indices(search_term) {
                    let from = idx;
                    let to = from + search_term.len();
                    diagnostics.push(ParseDiagnostic {
                        span: Some(TextRange::new(
                            from.try_into().unwrap(),
                            to.try_into().unwrap(),
                        )),
                        message: MessageAndDescription::from(err_msg.clone()),
                        is_fatal: true,
                    });
                }
            }
        }

        if diagnostics.is_empty() {
            diagnostics.push(ParseDiagnostic {
                span: None,
                message: MessageAndDescription::from(err_msg),
                is_fatal: true,
            });
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use pglt_lexer::lex;

    use super::*;

    #[test]
    fn failing_lexer() {
        let input =
            "select 1443ddwwd33djwdkjw13331333333333; select 1443ddwwd33djwdkjw13331333333333;";
        let err = lex(input).unwrap_err();

        let diagnostics = ParseDiagnostic::from_pg_query_err(err, input);
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|d| d.is_fatal));
    }
}
