pub fn parse_sql_statement(sql: &str) -> pg_query::Result<pg_query::NodeEnum> {
    pg_query::parse(sql).map(|parsed| {
        parsed
            .protobuf
            .nodes()
            .iter()
            .find(|n| n.1 == 1)
            .unwrap()
            .0
            .to_enum()
    })
}
