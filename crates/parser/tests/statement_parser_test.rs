use std::fs;
mod common;
use insta;
use log::debug;
use parser::Parser;

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

        debug!("Parsing statement: {}", test_name);

        let mut parser = Parser::new();
        parser.parse_statement_at(&contents, None);
        let parsed = parser.finish();

        let mut settings = insta::Settings::clone_current();
        settings.set_input_file(path);
        settings.set_prepend_module_to_snapshot(false);
        settings.set_description(contents);
        settings.set_omit_expression(true);
        settings.set_snapshot_path("snapshots/statements/valid");
        settings.bind(|| {
            insta::assert_debug_snapshot!(test_name, &parsed.cst);
        });
    });
}
