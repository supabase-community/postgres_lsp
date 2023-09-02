use codegen::get_children;

get_children!();

#[cfg(test)]
mod tests {
    use crate::get_children_codegen::get_children;

    #[test]
    fn test_get_children() {
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

        let children = get_children(&pg_query_root.unwrap(), input.to_string(), 1);
        assert_eq!(children.len(), 13);
    }
}
