#[cfg(test)]
mod tests {
    use std::assert_eq;
    use std::fs;
    use std::path::Path;

    use crate::pg_query_utils_generated::get_children;

    const VALID_STATEMENTS_PATH: &str = "test_data/statements/valid/";

    #[test]
    fn test_get_children() {
        let input = "select id;";

        let pg_query_root = match pg_query::parse(input) {
            Ok(parsed) => {
                parsed
                    .protobuf
                    .nodes()
                    .iter()
                    .for_each(|n| println!("{:?}", n));
                Some(
                    parsed
                        .protobuf
                        .nodes()
                        .iter()
                        .find(|n| n.1 == 1)
                        .unwrap()
                        .0
                        .to_enum(),
                )
            }
            Err(_) => None,
        };

        println!("{:?}", pg_query_root);

        let result = get_children(&pg_query_root.unwrap(), 1);

        println!("NUMBER OF CHILDREN: {:?}", result.len());

        // TODO: parse AConst correctly

        result.iter().for_each(|n| {
            println!("##");
            println!("{:?}", n)
        });
    }
}
