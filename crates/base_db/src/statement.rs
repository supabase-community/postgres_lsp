pub struct Statement {
    pub version: i32,

    parser: tree_sitter::Parser,

    pub tree: tree_sitter::Tree,
}

impl Statement {
    pub fn parse(content: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(&content, None).unwrap();

        Self {
            version: 0,
            parser,
            tree,
        }
    }
}
