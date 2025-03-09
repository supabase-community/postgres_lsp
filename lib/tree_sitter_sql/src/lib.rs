use tree_sitter::Language;

unsafe extern "C" {
    fn tree_sitter_sql() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_sql() }
}

#[cfg(test)]
mod tests {
    use tree_sitter::{Query, QueryCursor};

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading sql language");
        let source_code = "SELECT 1 FROM public.table where id = 4";

        let query = Query::new(
            parser.language().unwrap(),
            "(
  relation (
    (
      object_reference
      schema: (identifier)
      name: (identifier)
    ) @reference
  )
)
",
        )
        .unwrap();

        let tree = parser.parse(source_code, None).unwrap();

        let mut cursor = QueryCursor::new();

        let mut captures = cursor.captures(&query, tree.root_node(), source_code.as_bytes());
        let (match_, idx) = captures.next().unwrap();
        let capture = match_.captures[idx];
        assert_eq!(capture.node.kind(), "object_reference");
    }
}
