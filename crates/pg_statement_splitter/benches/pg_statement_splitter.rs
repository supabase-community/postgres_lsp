use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::{self};

const POSTGRES_REGRESS_PATH: &str = "../../libpg_query/test/sql/postgres_regress/";
const SKIPPED_REGRESS_TESTS: &str = include_str!("../tests/skipped.txt");

fn from_elem(c: &mut Criterion) {
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

        let contents_str = contents.as_str();

        c.bench_with_input(
            BenchmarkId::new(test_name, contents_str),
            &contents_str,
            |b, &s| {
                b.iter(|| pg_statement_splitter::split(&s));
            },
        );
    }
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
