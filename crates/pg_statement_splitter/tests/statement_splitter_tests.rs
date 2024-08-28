use insta::{assert_debug_snapshot, Settings};
use std::{
    fs::{self},
    panic,
};

use pg_lexer::SyntaxKind;

const DATA_DIR_PATH: &str = "tests/data/";
const POSTGRES_REGRESS_PATH: &str = "../../libpg_query/test/sql/postgres_regress/";
const SKIPPED_REGRESS_TESTS: &str = include_str!("skipped.txt");

const SNAPSHOTS_PATH: &str = "snapshots/data";

#[test]
fn test_postgres_regress() {
    // all postgres regress tests are valid and complete statements, so we can use `split_with_parser` and compare with our own splitter

    let mut paths: Vec<_> = fs::read_dir(POSTGRES_REGRESS_PATH)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    for f in paths.iter() {
        let path = f.path();

        let test_name = path.file_stem().unwrap().to_str().unwrap();

        // these require fixes in the parser
        if SKIPPED_REGRESS_TESTS
            .lines()
            .collect::<Vec<_>>()
            .contains(&test_name)
        {
            continue;
        }

        println!("Running test: {}", test_name);

        // remove \commands because pg_query doesn't support them
        let contents = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter_map(|l| {
                if !l.starts_with("\\")
                    && !l.ends_with("\\gset")
                    && !l.starts_with("--")
                    && !l.contains(":'")
                    && l.split("\t").count() <= 1
                    && l != "ALTER INDEX attmp_idx ALTER COLUMN 0 SET STATISTICS 1000;"
                {
                    if let Some(index) = l.find("--") {
                        Some(l[..index].to_string())
                    } else {
                        Some(l.to_string())
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let libpg_query_split_result = pg_query::split_with_parser(&contents);

        if libpg_query_split_result.is_err() {
            eprintln!(
                "Failed to split statements for test '{}': {:?}",
                test_name, libpg_query_split_result
            );
            continue;
        }

        let libpg_query_split = libpg_query_split_result.unwrap();

        let result = panic::catch_unwind(|| pg_statement_splitter::statements(&contents));

        if result.is_err() {
            panic!(
                "Failed to split statements for test '{}': {:?}",
                test_name,
                result.unwrap_err()
            );
        }

        let split = result.unwrap();

        // assert_eq!(
        //     libpg_query_split.len(),
        //     split.len(),
        //     "[{}] Mismatch in statement count: Expected {} statements, got {}. Contents:\n{}",
        //     test_name,
        //     libpg_query_split.len(),
        //     split.len(),
        //     contents
        // );

        for (libpg_query_stmt, parser_result) in libpg_query_split.iter().zip(split.iter()) {
            let mut parser_stmt = contents[parser_result.range.clone()].trim().to_string();

            if parser_stmt.ends_with(';') {
                let mut s = parser_stmt.chars().rev().skip(1).collect::<String>();
                s = s.chars().rev().collect();
                parser_stmt = format!("{}{}", s.trim(), ";");
            }

            let libpg_query_stmt = if libpg_query_stmt.ends_with(';') {
                libpg_query_stmt.to_string()
            } else {
                format!("{};", libpg_query_stmt.trim())
            };

            let libpg_query_stmt_trimmed = libpg_query_stmt.trim();
            let parser_stmt_trimmed = parser_stmt.trim();

            let root = pg_query::parse(libpg_query_stmt_trimmed)
                .map(|parsed| {
                    parsed
                        .protobuf
                        .nodes()
                        .iter()
                        .find(|n| n.1 == 1)
                        .unwrap()
                        .0
                        .to_enum()
                })
                .expect("Failed to parse statement");

            assert_eq!(
                libpg_query_stmt_trimmed, parser_stmt_trimmed,
                "[{}] Mismatch in statement:\nlibg_query: '{}'\nsplitter:   '{}'\n Root Node: {:?}",
                test_name, libpg_query_stmt_trimmed, parser_stmt_trimmed, root
            );

            let syntax_kind = SyntaxKind::from(&root);

            assert_eq!(
                syntax_kind, parser_result.kind,
                "[{}] Mismatch in statement type. Expected {:?}, got {:?} for statement '{}'. Root Node: {:?}",
                test_name, syntax_kind, parser_result.kind, parser_stmt_trimmed, root
            );

            println!("[{}] Matched {}", test_name, parser_stmt_trimmed);
        }
    }
}

#[test]
fn test_statement_splitter() {
    let mut paths: Vec<_> = fs::read_dir(DATA_DIR_PATH)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    for f in paths.iter() {
        let path = f.path();
        let test_name = path.file_stem().unwrap().to_str().unwrap();

        let contents = fs::read_to_string(&path).unwrap();

        let statements = pg_statement_splitter::statements(&contents);

        let result = statements
            .iter()
            .map(|x| (x.kind, x.range, &contents[x.range.clone()]))
            .collect::<Vec<_>>();

        let mut settings = Settings::clone_current();
        settings.set_input_file(&path);
        settings.set_prepend_module_to_snapshot(false);
        settings.set_description(contents.to_string());
        settings.set_omit_expression(true);
        settings.set_snapshot_path(SNAPSHOTS_PATH);

        settings.bind(|| assert_debug_snapshot!(test_name, result));
    }
}
