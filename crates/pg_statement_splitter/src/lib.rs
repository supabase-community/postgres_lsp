///! Postgres Statement Splitter
///!
///! This crate provides a function to split a SQL source string into individual statements.
mod parser;
mod syntax_error;

use parser::{source, Parse, Parser};

pub fn split(sql: &str) -> Parse {
    let mut parser = Parser::new(sql);

    source(&mut parser);

    parser.finish()
}

#[cfg(test)]
mod tests {
    use ntest::timeout;

    use super::*;

    struct Tester {
        input: String,
        parse: Parse,
    }

    impl From<&str> for Tester {
        fn from(input: &str) -> Self {
            Tester {
                parse: split(input),
                input: input.to_string(),
            }
        }
    }

    impl Tester {
        fn expect_statements(&self, expected: Vec<&str>) {
            assert_eq!(self.parse.ranges.len(), expected.len());

            for (range, expected) in self.parse.ranges.iter().zip(expected.iter()) {
                assert_eq!(*expected, self.input[*range].to_string());
            }
        }
    }

    #[test]
    #[timeout(1000)]
    fn basic() {
        Tester::from("select 1 from contact; select 1;")
            .expect_statements(vec!["select 1 from contact;", "select 1;"]);
    }

    #[test]
    fn no_semicolons() {
        Tester::from("select 1 from contact\nselect 1")
            .expect_statements(vec!["select 1 from contact", "select 1"]);
    }

    #[test]
    fn double_newlines() {
        Tester::from("select 1 from contact\n\nselect 1\n\nselect 3").expect_statements(vec![
            "select 1 from contact",
            "select 1",
            "select 3",
        ]);
    }
}
