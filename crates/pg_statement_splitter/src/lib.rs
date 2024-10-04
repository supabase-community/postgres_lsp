///! Postgres Statement Splitter
///!
///! This crate provides a function to split a SQL source string into individual statements.

mod data;
mod split;
mod parser;
mod syntax_error;

use parser::{Parse, Parser};

use pg_lexer::{lex};
use split::parse_source;

pub fn split(sql: &str) -> Parse {
    let mut parser = Parser::new(lex(sql));

    parse_source(&mut parser);

    parser.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitter() {
        let input = "select 1 from contact;\nselect 1;";

        let res = split(input);
        assert_eq!(res.ranges.len(), 2);
        assert_eq!("select 1 from contact;", input[res.ranges[0]].to_string());
        assert_eq!("select 1;", input[res.ranges[1]].to_string());
    }

    #[test]
    fn test_splitter_no_semicolons() {
        let input = "select 1 from contact\nselect 1";

        let res = split(input);
        assert_eq!(res.ranges.len(), 2);
        assert_eq!("select 1 from contact", input[res.ranges[0]].to_string());
        assert_eq!("select 1", input[res.ranges[1]].to_string());
    }

    #[test]
    fn test_splitter_double_newlines() {
        let input = "select 1 from contact\nselect 1\n\nalter table t add column c int";

        let res = split(input);
        assert_eq!(res.ranges.len(), 3);
        assert_eq!("select 1 from contact", input[res.ranges[0]].to_string());
        assert_eq!("select 1", input[res.ranges[1]].to_string());
        assert_eq!("alter table t add column c int", input[res.ranges[2]].to_string());
    }
}
