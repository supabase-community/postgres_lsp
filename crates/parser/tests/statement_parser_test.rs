use std::fs;
mod common;
mod get_node_properties_test;
use log::debug;
use parser::parse_source;

const VALID_STATEMENTS_PATH: &str = "tests/data/statements/valid/";

#[test]
fn valid_statements() {
    common::setup();

    let mut paths: Vec<_> = fs::read_dir(VALID_STATEMENTS_PATH)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    paths.iter().for_each(|f| {
        let path = f.path();
        let file_name = path.file_name().unwrap();
        let test_name = file_name.to_str().unwrap().replace(".sql", "");

        let contents = fs::read_to_string(&path).unwrap();

        debug!("Parsing statement {}\n{}", test_name, contents);

        let result = std::panic::catch_unwind(|| parse_source(&contents));
        if result.is_err() {
            assert!(
                false,
                "Failed to parse statement {}: {:#?}",
                test_name,
                result.unwrap_err()
            );
        }
    });
}
