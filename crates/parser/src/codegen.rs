use codegen::parser_codegen;

parser_codegen!();

#[cfg(test)]
mod tests {
    use crate::codegen::{get_nodes, Path};

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

        let node_graph = get_nodes(&pg_query_root.unwrap(), 0);
        assert_eq!(node_graph.node_count(), 13);
    }

    #[test]
    fn test_path() {
        assert!(Path::new(vec![0, 1, 2]) > Path::new(vec![0, 1]));
        assert!(Path::new(vec![0, 2, 2]) > Path::new(vec![0, 1]));
        assert!(Path::new(vec![1, 2, 0]) > Path::new(vec![1, 1]));
        assert!(Path::new(vec![1]) == Path::new(vec![1]));
        assert!(Path::from_parent(Path::new(vec![0, 1]), 2) == Path::new(vec![0, 1, 2]));
    }
}
