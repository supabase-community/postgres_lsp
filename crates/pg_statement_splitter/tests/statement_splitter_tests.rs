use insta::{assert_debug_snapshot, Settings};
use std::fs::{self};

use pg_lexer::SyntaxKind;

const DATA_DIR_PATH: &str = "tests/data/";
const POSTGRES_REGRESS_PATH: &str = "../../libpg_query/test/sql/postgres_regress/";
const SKIPPED_REGRESS_TESTS: &str = include_str!("skipped.txt");

const SNAPSHOTS_PATH: &str = "snapshots/data";

// #[test]
// fn test_postgres_regress() {
//     // all postgres regress tests are valid and complete statements, so we can use `split_with_parser` and compare with our own splitter
//
//     let mut paths: Vec<_> = fs::read_dir(POSTGRES_REGRESS_PATH)
//         .unwrap()
//         .map(|r| r.unwrap())
//         .collect();
//     paths.sort_by_key(|dir| dir.path());
//
//     for f in paths.iter() {
//         let path = f.path();
//
//         let test_name = path.file_stem().unwrap().to_str().unwrap();
//
//         // these require fixes in the parser
//         if SKIPPED_REGRESS_TESTS
//             .lines()
//             .collect::<Vec<_>>()
//             .contains(&test_name)
//         {
//             continue;
//         }
//
//         println!("Running test: {}", test_name);
//
//         // remove \commands because pg_query doesn't support them
//         let contents = fs::read_to_string(&path)
//             .unwrap()
//             .lines()
//             .filter(|l| {
//                 !l.starts_with("\\")
//                     && !l.ends_with("\\gset")
//                     && !l.starts_with("--")
//                     && !l.contains(":'")
//                     && l.split("\t").count() <= 2
//             })
//             .collect::<Vec<_>>()
//             .join("\n");
//
//         let libpg_query_split = pg_query::split_with_parser(&contents).expect("Failed to split");
//
//         let split = pg_statement_splitter::statements(&contents);
//
//         // assert_eq!(
//         //     libpg_query_split.len(),
//         //     split.len(),
//         //     "[{}] Mismatch in statement count: Expected {} statements, got {}. Contents:\n{}",
//         //     test_name,
//         //     libpg_query_split.len(),
//         //     split.len(),
//         //     contents
//         // );
//
//         for (libpg_query_stmt, parser_result) in libpg_query_split.iter().zip(split.iter()) {
//             let parser_stmt = &contents[parser_result.range.clone()].trim();
//
//             let libpg_query_stmt = if libpg_query_stmt.ends_with(';') {
//                 libpg_query_stmt.to_string()
//             } else {
//                 format!("{};", libpg_query_stmt.trim())
//             };
//
//             let libpg_query_stmt_trimmed = libpg_query_stmt.trim();
//             let parser_stmt_trimmed = parser_stmt.trim();
//
//             assert_eq!(
//                 libpg_query_stmt_trimmed, parser_stmt_trimmed,
//                 "[{}] Mismatch in statement:\nlibg_query: '{}'\nsplitter:   '{}'",
//                 test_name, libpg_query_stmt_trimmed, parser_stmt_trimmed
//             );
//
//             let root = pg_query::parse(libpg_query_stmt_trimmed).map(|parsed| {
//                 parsed
//                     .protobuf
//                     .nodes()
//                     .iter()
//                     .find(|n| n.1 == 1)
//                     .unwrap()
//                     .0
//                     .to_enum()
//             });
//
//             let syntax_kind = SyntaxKind::from(&root.expect("Failed to parse statement"));
//
//             assert_eq!(
//                 syntax_kind, parser_result.kind,
//                 "[{}] Mismatch in statement type. Expected {:?}, got {:?}",
//                 test_name, parser_result.kind, syntax_kind
//             );
//
//             println!("[{}] Matched {}", test_name, parser_stmt_trimmed);
//         }
//     }
// }

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
