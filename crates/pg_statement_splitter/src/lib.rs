///! Postgres Statement Splitter
///!
///! This crate provides a function to split a SQL source string into individual statements.
///!
///! TODO:
///! Instead of relying on statement start tokens, we need to include as many tokens as
///! possible. For example, a `CREATE TRIGGER` statement includes an `EXECUTE [ PROCEDURE |
///! FUNCTION ]` clause, but `EXECUTE` is also a statement start token for an `EXECUTE` statement.
/// We should expand the definition map to include an `Any*`, which must be followed by at least
/// one required token and allows the parser to search for the end tokens of the statement. This
/// will hopefully be enough to reduce collisions to zero.
mod data;
mod parser;
mod statement_splitter;
mod syntax_error;
mod tracker;

use statement_splitter::{StatementPosition, StatementSplitter};
use text_size::TextRange;

pub fn split(sql: &str) -> Vec<TextRange> {
    StatementSplitter::new(sql)
        .run()
        .iter()
        .map(|x| x.range)
        .collect()
}

/// mostly used for testing
pub fn statements(sql: &str) -> Vec<StatementPosition> {
    StatementSplitter::new(sql).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitter() {
        let input = "select 1 from contact;\nselect 1;\nalter table test drop column id;";

        let res = split(input);

        assert_eq!(res.len(), 3);
        assert_eq!("select 1 from contact;", input[res[0]].to_string());
        assert_eq!("select 1;", input[res[1]].to_string());
        assert_eq!(
            "alter table test drop column id;",
            input[res[2]].to_string()
        );
    }
}
