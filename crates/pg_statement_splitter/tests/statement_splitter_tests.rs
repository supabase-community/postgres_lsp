use std::fs::{self};

const DATA_DIR_PATH: &str = "tests/data/";
const POSTGRES_REGRESS_PATH: &str = "../../libpg_query/test/sql/postgres_regress/";
const SKIPPED_REGRESS_TESTS: &str = include_str!("skipped.txt");

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

        // remove \commands because pg_query doesn't support them
        let contents = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with("\\") && !l.ends_with("\\gset"))
            .collect::<Vec<_>>()
            .join(" ");

        let libpg_query_split = pg_query::split_with_parser(&contents).unwrap();

        let parser_split = pg_statement_splitter::split(&contents);

        assert_eq!(
            parser_split.errors.len(),
            0,
            "Unexpected errors when parsing file {}:\n{:#?}",
            test_name,
            parser_split.errors
        );

        assert_eq!(
            libpg_query_split.len(),
            parser_split.ranges.len(),
            "Mismatch in statement count for file {}: Expected {} statements, got {}",
            test_name,
            libpg_query_split.len(),
            parser_split.ranges.len()
        );

        for (libpg_query_stmt, parser_range) in
            libpg_query_split.iter().zip(parser_split.ranges.iter())
        {
            let parser_stmt = &contents[parser_range.clone()].trim();

            let libpg_query_stmt = if libpg_query_stmt.ends_with(';') {
                libpg_query_stmt.to_string()
            } else {
                format!("{};", libpg_query_stmt.trim())
            };

            let libpg_query_stmt_trimmed = libpg_query_stmt.trim();
            let parser_stmt_trimmed = parser_stmt.trim();

            assert_eq!(
                libpg_query_stmt_trimmed, parser_stmt_trimmed,
                "Mismatch in statement {}:\nlibg_query: '{}'\nsplitter:   '{}'",
                test_name, libpg_query_stmt_trimmed, parser_stmt_trimmed
            );
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
        let expected_count = test_name
            .split("__")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let contents = fs::read_to_string(&path).unwrap();

        let split = pg_statement_splitter::split(&contents);

        assert_eq!(
            split.ranges.len(),
            expected_count,
            "Mismatch in statement count for file {}: Expected {} statements, got {}",
            test_name,
            expected_count,
            split.ranges.len()
        );
    }
}
