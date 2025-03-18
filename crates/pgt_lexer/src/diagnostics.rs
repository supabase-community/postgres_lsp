use pgt_diagnostics::{Diagnostic, MessageAndDescription};
use pgt_text_size::TextRange;

/// A specialized diagnostic for scan errors.
///
/// Scan diagnostics are always **fatal errors**.
#[derive(Clone, Debug, Diagnostic, PartialEq)]
#[diagnostic(category = "syntax", severity = Fatal)]
pub struct ScanError {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

impl ScanError {
    pub fn from_pg_query_err(err: pg_query::Error, input: &str) -> Vec<Self> {
        let err_msg = err.to_string();
        let re = regex::Regex::new(r#"at or near "(.*?)""#).unwrap();
        let mut diagnostics = Vec::new();

        for captures in re.captures_iter(&err_msg) {
            if let Some(matched) = captures.get(1) {
                let search_term = matched.as_str();
                for (idx, _) in input.match_indices(search_term) {
                    let from = idx;
                    let to = from + search_term.len();
                    diagnostics.push(ScanError {
                        span: Some(TextRange::new(
                            from.try_into().unwrap(),
                            to.try_into().unwrap(),
                        )),
                        message: MessageAndDescription::from(err_msg.clone()),
                    });
                }
            }
        }

        if diagnostics.is_empty() {
            diagnostics.push(ScanError {
                span: None,
                message: MessageAndDescription::from(err_msg),
            });
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use crate::lex;

    #[test]
    fn finds_all_occurrences() {
        let input =
            "select 1443ddwwd33djwdkjw13331333333333; select 1443ddwwd33djwdkjw13331333333333;";
        let diagnostics = lex(input).unwrap_err();
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].span.unwrap().start(), 7.into());
        assert_eq!(diagnostics[0].span.unwrap().end(), 39.into());
        assert_eq!(diagnostics[1].span.unwrap().start(), 48.into());
        assert_eq!(diagnostics[1].span.unwrap().end(), 80.into());
    }
}
