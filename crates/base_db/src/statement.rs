use crate::{
    source_file::FileChange,
    utils::{apply_text_change, edit_from_change},
};

pub struct Statement {
    pub version: i32,
    pub text: String,

    parser: tree_sitter::Parser,

    pub tree: tree_sitter::Tree,
}

impl Statement {
    pub fn new(text: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        let tree = parser.parse(&text, None).unwrap();

        Self {
            text,
            version: 0,
            parser,
            tree,
        }
    }

    pub fn apply_change(&mut self, change: FileChange) {
        assert!(change.range.is_some());

        let range = change.range.unwrap();

        let edit = edit_from_change(
            &self.text.as_str(),
            usize::from(range.start()),
            usize::from(range.end()),
            change.text.as_str(),
        );

        self.tree.edit(&edit);

        self.text = apply_text_change(&self.text, &change);

        self.tree = self.parser.parse(&self.text, Some(&self.tree)).unwrap();

        self.version += 1;
    }
}
