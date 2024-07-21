use insta::{assert_debug_snapshot, Settings};
use pg_query::split_with_parser;
use std::{
    fs::{self},
    panic,
};

const VALID_STATEMENTS_PATH: &str = "tests/data/";
const POSTGRES_REGRESS_PATH: &str = "../../libpg_query/test/sql/postgres_regress/";
const SKIPPED_REGRESS_TESTS: &str = include_str!("skipped.txt");

const REGRESSION_SNAPSHOTS_PATH: &str = "snapshots/postgres_regress";
const SNAPSHOTS_PATH: &str = "snapshots/data";

#[test]
fn valid_statements() {
    for path in [VALID_STATEMENTS_PATH, POSTGRES_REGRESS_PATH] {
        let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
        paths.sort_by_key(|dir| dir.path());

        for f in paths.iter() {
            let path = f.path();
            let test_name = path.file_stem().unwrap().to_str().unwrap();

            if SKIPPED_REGRESS_TESTS
                .lines()
                .collect::<Vec<_>>()
                .contains(&test_name)
            {
                continue;
            }

            let contents = fs::read_to_string(&path).unwrap();

            let cases = split_with_parser(&contents).unwrap();

            for (i, case) in cases.iter().enumerate() {
                let case = format!("{};", case.trim());

                let root = pg_query_ext::parse(&case).unwrap();

                let result = panic::catch_unwind(|| pg_syntax::parse_syntax(&case, &root));

                if result.is_err() {
                    assert!(false, "Failed to parse statement {}:\n{}", test_name, case);
                }

                let mut settings = Settings::clone_current();
                settings.set_input_file(&path);
                settings.set_prepend_module_to_snapshot(false);
                settings.set_description(case.to_string());
                settings.set_omit_expression(true);
                let snapshot_path = if path.starts_with(POSTGRES_REGRESS_PATH) {
                    REGRESSION_SNAPSHOTS_PATH
                } else if path.starts_with(VALID_STATEMENTS_PATH) {
                    SNAPSHOTS_PATH
                } else {
                    panic!("Unknown path: {:?}", path);
                };
                settings.set_snapshot_path(snapshot_path);
                settings.set_snapshot_suffix((i + 1).to_string());

                settings.bind(|| assert_debug_snapshot!(test_name, result.unwrap().cst));
            }
        }
    }
}
