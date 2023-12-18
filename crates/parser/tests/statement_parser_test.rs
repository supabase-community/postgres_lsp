mod common;

use insta::{assert_debug_snapshot, Settings};
use log::{debug, info};
use parser::parse_source;
use pg_query::split_with_parser;
use std::{fs, panic};

const VALID_STATEMENTS_PATH: &str = "tests/data/statements/valid/";

#[test]
fn valid_statements() {
    common::setup();

    let mut paths: Vec<_> = fs::read_dir(VALID_STATEMENTS_PATH)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    for f in paths.iter() {
        let path = f.path();
        let test_name = path.file_stem().unwrap().to_str().unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        let cases = split_with_parser(&contents).unwrap();

        for (i, case) in cases.iter().enumerate() {
            debug!("Parsing statement {}\n{}", test_name, case);

            let result = panic::catch_unwind(|| parse_source(&case));

            if result.is_err() {
                assert!(false, "Failed to parse statement {}:\n{}", test_name, case);
            } else {
                info!(
                    "Successfully parsed statement {}\n'{}'\n{:#?}",
                    test_name,
                    case,
                    result.as_ref().unwrap().cst
                );
            }

            let mut settings = Settings::clone_current();
            settings.set_input_file(&path);
            settings.set_prepend_module_to_snapshot(false);
            settings.set_description(case.to_string());
            settings.set_omit_expression(true);
            settings.set_snapshot_path("snapshots/statements/valid");
            settings.set_snapshot_suffix((i + 1).to_string());

            settings.bind(|| assert_debug_snapshot!(test_name, result.unwrap()));
        }
    }
}
