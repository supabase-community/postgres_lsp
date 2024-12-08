use pg_schema_cache::SchemaCache;

use crate::CompletionParams;

pub(crate) struct CompletionContext<'a> {
    pub ts_node: Option<tree_sitter::Node<'a>>,
    pub tree: Option<&'a tree_sitter::Tree>,
    pub text: &'a str,
    pub schema_cache: &'a SchemaCache,
    pub position: usize,

    pub schema_name: Option<String>,
    pub wrapping_clause_type: Option<String>,
    pub is_invocation: bool,
}

impl<'a> CompletionContext<'a> {
    pub fn new(params: &'a CompletionParams) -> Self {
        let mut tree = Self {
            tree: params.tree,
            text: &params.text,
            schema_cache: params.schema,
            position: usize::from(params.position),

            ts_node: None,
            schema_name: None,
            wrapping_clause_type: None,
            is_invocation: false,
        };

        tree.gather_tree_context();

        tree
    }

    pub fn get_ts_node_content(&self, ts_node: tree_sitter::Node<'a>) -> Option<&'a str> {
        let source = self.text;
        match ts_node.utf8_text(source.as_bytes()) {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }

    fn gather_tree_context(&mut self) {
        if self.tree.is_none() {
            return;
        }

        let mut cursor = self.tree.as_ref().unwrap().root_node().walk();

        // go to the statement node that matches the position
        let current_node_kind = cursor.node().kind();

        cursor.goto_first_child_for_byte(self.position);

        self.gather_context_from_node(cursor, current_node_kind);
    }

    fn gather_context_from_node(
        &mut self,
        mut cursor: tree_sitter::TreeCursor<'a>,
        previous_node_kind: &str,
    ) {
        let current_node = cursor.node();
        let current_node_kind = current_node.kind();

        match previous_node_kind {
            "statement" => self.wrapping_clause_type = Some(current_node_kind.to_string()),
            "invocation" => self.is_invocation = true,

            _ => {}
        }

        match current_node_kind {
            "object_reference" => {
                let txt = self.get_ts_node_content(current_node);
                if let Some(txt) = txt {
                    let parts: Vec<&str> = txt.split('.').collect();
                    if parts.len() == 2 {
                        self.schema_name = Some(parts[0].to_string());
                    }
                }
            }

            // in Treesitter, the Where clause is nested inside other clauses
            "where" => {
                self.wrapping_clause_type = Some("where".to_string());
            }

            _ => {}
        }

        if current_node.child_count() == 0 {
            self.ts_node = Some(current_node);
            return;
        }

        cursor.goto_first_child_for_byte(self.position);
        self.gather_context_from_node(cursor, current_node_kind);
    }
}

#[cfg(test)]
mod tests {
    use crate::context::CompletionContext;

    fn get_tree(input: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Couldn't set language");

        parser.parse(input, None).expect("Unable to parse tree")
    }

    static CURSOR_POS: &str = "XXX";

    #[test]
    fn identifies_clauses() {
        let test_cases = vec![
            (format!("Select {}* from users;", CURSOR_POS), "select"),
            (format!("Select * from u{};", CURSOR_POS), "from"),
            (
                format!("Select {}* from users where n = 1;", CURSOR_POS),
                "select",
            ),
            (
                format!("Select * from users where {}n = 1;", CURSOR_POS),
                "where",
            ),
            (
                format!("update users set u{} = 1 where n = 2;", CURSOR_POS),
                "update",
            ),
            (
                format!("update users set u = 1 where n{} = 2;", CURSOR_POS),
                "where",
            ),
            (format!("delete{} from users;", CURSOR_POS), "delete"),
            (format!("delete from {}users;", CURSOR_POS), "from"),
            (
                format!("select name, age, location from public.u{}sers", CURSOR_POS),
                "from",
            ),
        ];

        for (text, expected_clause) in test_cases {
            let position = text.find(CURSOR_POS).unwrap();
            let text = text.replace(CURSOR_POS, "");

            let tree = get_tree(text.as_str());
            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text: text.as_str(),
                tree: Some(&tree),
                schema: &pg_schema_cache::SchemaCache::new(),
            };

            let ctx = CompletionContext::new(&params);

            assert_eq!(ctx.wrapping_clause_type, Some(expected_clause.to_string()));
        }
    }

    #[test]
    fn identifies_schema() {
        let test_cases = vec![
            (
                format!("Select * from private.u{}", CURSOR_POS),
                Some("private"),
            ),
            (
                format!("Select * from private.u{}sers()", CURSOR_POS),
                Some("private"),
            ),
            (format!("Select * from u{}sers", CURSOR_POS), None),
            (format!("Select * from u{}sers()", CURSOR_POS), None),
        ];

        for (text, expected_schema) in test_cases {
            let position = text.find(CURSOR_POS).unwrap();
            let text = text.replace(CURSOR_POS, "");

            let tree = get_tree(text.as_str());
            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text: text.as_str(),
                tree: Some(&tree),
                schema: &pg_schema_cache::SchemaCache::new(),
            };

            let ctx = CompletionContext::new(&params);

            assert_eq!(ctx.schema_name, expected_schema.map(|f| f.to_string()));
        }
    }

    #[test]
    fn identifies_invocation() {
        let test_cases = vec![
            (format!("Select * from u{}sers", CURSOR_POS), false),
            (format!("Select * from u{}sers()", CURSOR_POS), true),
            (format!("Select cool{};", CURSOR_POS), false),
            (format!("Select cool{}();", CURSOR_POS), true),
            (
                format!("Select upp{}ercase as title from users;", CURSOR_POS),
                false,
            ),
            (
                format!("Select upp{}ercase(name) as title from users;", CURSOR_POS),
                true,
            ),
        ];

        for (text, is_invocation) in test_cases {
            let position = text.find(CURSOR_POS).unwrap();
            let text = text.replace(CURSOR_POS, "");

            let tree = get_tree(text.as_str());
            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text: text.as_str(),
                tree: Some(&tree),
                schema: &pg_schema_cache::SchemaCache::new(),
            };

            let ctx = CompletionContext::new(&params);

            assert_eq!(ctx.is_invocation, is_invocation);
        }
    }
}
