//! Postgres Statement Splitter
//!
//! This crate provides a function to split a SQL source string into individual statements.
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
            assert_eq!(
                self.parse.ranges.len(),
                expected.len(),
                "Expected {} statements, got {}",
                expected.len(),
                self.parse.ranges.len()
            );

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

    #[test]
    fn insert_with_select() {
        Tester::from("\ninsert into tbl (id) select 1\n\nselect 3")
            .expect_statements(vec!["insert into tbl (id) select 1", "select 3"]);
    }

    #[test]
    fn case() {
        Tester::from("select case when select 2 then 1 else 0 end")
            .expect_statements(vec!["select case when select 2 then 1 else 0 end"]);
    }

    #[test]
    fn create_rule() {
        Tester::from(
            "create rule log_employee_insert as
on insert to employees
do also insert into employee_log (action, employee_id, log_time)
values ('insert', new.id, now());",
        )
        .expect_statements(vec![
            "create rule log_employee_insert as
on insert to employees
do also insert into employee_log (action, employee_id, log_time)
values ('insert', new.id, now());",
        ]);
    }

    #[test]
    fn insert_into() {
        Tester::from("randomness\ninsert into tbl (id) values (1)\nselect 3").expect_statements(
            vec!["randomness", "insert into tbl (id) values (1)", "select 3"],
        );
    }

    #[test]
    fn update() {
        Tester::from("more randomness\nupdate tbl set col = '1'\n\nselect 3").expect_statements(
            vec!["more randomness", "update tbl set col = '1'", "select 3"],
        );
    }

    #[test]
    fn delete_from() {
        Tester::from("more randomness\ndelete from test where id = 1\n\nselect 3")
            .expect_statements(vec![
                "more randomness",
                "delete from test where id = 1",
                "select 3",
            ]);
    }

    #[test]
    fn unknown() {
        Tester::from("random stuff\n\nmore randomness\n\nselect 3").expect_statements(vec![
            "random stuff",
            "more randomness",
            "select 3",
        ]);
    }

    #[test]
    fn unknown_2() {
        Tester::from("random stuff\nselect 1\n\nselect 3").expect_statements(vec![
            "random stuff",
            "select 1",
            "select 3",
        ]);
    }
}
