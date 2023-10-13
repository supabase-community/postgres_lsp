use codegen::get_nodes;

get_nodes!();

#[cfg(test)]
mod tests {
    use crate::get_nodes_codegen::get_nodes;

    #[test]
    fn test_get_nodes() {
        let input = "with c as (insert into contact (id) values ('id')) select * from c;";

        let pg_query_root = match pg_query::parse(input) {
            Ok(parsed) => Some(
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .find(|n| n.1 == 1)
                    .unwrap()
                    .0
                    .to_enum(),
            ),
            Err(_) => None,
        };

        let nodes = get_nodes(&pg_query_root.unwrap(), input.to_string(), 1);
        assert_eq!(nodes.len(), 14);
    }
}
