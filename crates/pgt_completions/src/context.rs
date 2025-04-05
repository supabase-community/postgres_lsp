use std::collections::{HashMap, HashSet};

use pgt_schema_cache::SchemaCache;
use pgt_treesitter_queries::{
    TreeSitterQueriesExecutor,
    queries::{self, QueryResult},
};

use crate::CompletionParams;

#[derive(Debug, PartialEq, Eq)]
pub enum ClauseType {
    Select,
    Where,
    From,
    Update,
    Delete,
}

impl TryFrom<&str> for ClauseType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "select" => Ok(Self::Select),
            "where" => Ok(Self::Where),
            "from" | "keyword_from" => Ok(Self::From),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            _ => {
                let message = format!("Unimplemented ClauseType: {}", value);

                // Err on tests, so we notice that we're lacking an implementation immediately.
                if cfg!(test) {
                    panic!("{}", message);
                }

                Err(message)
            }
        }
    }
}

impl TryFrom<String> for ClauseType {
    type Error = String;
    fn try_from(value: String) -> Result<ClauseType, Self::Error> {
        ClauseType::try_from(value.as_str())
    }
}

pub(crate) struct CompletionContext<'a> {
    pub node_under_cursor: Option<tree_sitter::Node<'a>>,
    pub previous_node: Option<tree_sitter::Node<'a>>,

    pub tree: Option<&'a tree_sitter::Tree>,
    pub text: &'a str,
    pub schema_cache: &'a SchemaCache,
    pub position: usize,

    /// If the cursor of the user is offset to the right of the statement,
    /// we'll have to move it back to the last node, otherwise, tree-sitter will break.
    /// However, knowing that the user is typing on the "next" node lets us prioritize different completion results.
    /// We consider an offset of up to two characters as valid.
    ///
    /// Example:
    ///
    /// ```
    /// select * from {}
    /// ```
    ///
    /// We'll adjust the cursor position so it lies on the "from" token – but we're looking
    /// for table completions.
    pub cursor_offset_from_end: bool,

    pub schema_name: Option<String>,
    pub wrapping_clause_type: Option<ClauseType>,
    pub is_invocation: bool,
    pub wrapping_statement_range: Option<tree_sitter::Range>,

    pub mentioned_relations: HashMap<Option<String>, HashSet<String>>,
}

impl<'a> CompletionContext<'a> {
    pub fn new(params: &'a CompletionParams) -> Self {
        let mut ctx = Self {
            tree: params.tree,
            text: &params.text,
            schema_cache: params.schema,
            position: usize::from(params.position),
            cursor_offset_from_end: false,
            previous_node: None,
            node_under_cursor: None,
            schema_name: None,
            wrapping_clause_type: None,
            wrapping_statement_range: None,
            is_invocation: false,
            mentioned_relations: HashMap::new(),
        };

        ctx.gather_tree_context();
        ctx.gather_info_from_ts_queries();

        ctx
    }

    fn gather_info_from_ts_queries(&mut self) {
        let tree = match self.tree.as_ref() {
            None => return,
            Some(t) => t,
        };

        let stmt_range = self.wrapping_statement_range.as_ref();
        let sql = self.text;

        let mut executor = TreeSitterQueriesExecutor::new(tree.root_node(), sql);

        executor.add_query_results::<queries::RelationMatch>();

        for relation_match in executor.get_iter(stmt_range) {
            match relation_match {
                QueryResult::Relation(r) => {
                    let schema_name = r.get_schema(sql);
                    let table_name = r.get_table(sql);

                    let current = self.mentioned_relations.get_mut(&schema_name);

                    match current {
                        Some(c) => {
                            c.insert(table_name);
                        }
                        None => {
                            let mut new = HashSet::new();
                            new.insert(table_name);
                            self.mentioned_relations.insert(schema_name, new);
                        }
                    };
                }
            };
        }
    }

    pub fn get_ts_node_content(&self, ts_node: tree_sitter::Node<'a>) -> Option<&'a str> {
        let source = self.text;
        ts_node.utf8_text(source.as_bytes()).ok()
    }

    fn gather_tree_context(&mut self) {
        if self.tree.is_none() {
            return;
        }

        let mut cursor = self.tree.as_ref().unwrap().root_node().walk();

        /*
         * The head node of any treesitter tree is always the "PROGRAM" node.
         *
         * We want to enter the next layer and focus on the child node that matches the user's cursor position.
         * If there is no node under the users position, however, the cursor won't enter the next level – it
         * will stay on the Program node.
         *
         * This might lead to an unexpected context or infinite recursion.
         *
         * We'll therefore adjust the cursor position such that it meets the last node of the AST.
         * `select * from use           {}` becomes `select * from use{}`.
         */
        let current_node = cursor.node();
        let position_cache = self.position.clone();
        while cursor.goto_first_child_for_byte(self.position).is_none() && self.position > 0 {
            self.position -= 1;
        }

        let cursor_offset = position_cache - self.position;
        self.cursor_offset_from_end = cursor_offset > 0 && cursor_offset <= 2;

        self.gather_context_from_node(cursor, current_node);
    }

    fn gather_context_from_node(
        &mut self,
        mut cursor: tree_sitter::TreeCursor<'a>,
        parent_node: tree_sitter::Node<'a>,
    ) {
        let current_node = cursor.node();

        // prevent infinite recursion – this can happen if we only have a PROGRAM node
        if current_node.kind() == parent_node.kind() {
            self.node_under_cursor = Some(current_node);
            return;
        }

        match parent_node.kind() {
            "statement" | "subquery" => {
                self.wrapping_clause_type = current_node.kind().try_into().ok();
                self.wrapping_statement_range = Some(parent_node.range());
            }
            "invocation" => self.is_invocation = true,

            _ => {}
        }

        match current_node.kind() {
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
                self.wrapping_clause_type = "where".try_into().ok();
            }

            "keyword_from" => {
                self.wrapping_clause_type = "keyword_from".try_into().ok();
            }

            _ => {}
        }

        // We have arrived at the leaf node
        if current_node.child_count() == 0 {
            if self.cursor_offset_from_end {
                self.node_under_cursor = None;
                self.previous_node = Some(current_node);
            } else {
                // for the previous node, either select the previous sibling,
                // or collect the parent's previous sibling's last child.
                let previous = match current_node.prev_sibling() {
                    Some(n) => Some(n),
                    None => {
                        let sib_of_parent = parent_node.prev_sibling();
                        sib_of_parent.and_then(|p| p.children(&mut cursor).last())
                    }
                };
                self.node_under_cursor = Some(current_node);
                self.previous_node = previous;
            }

            return;
        }

        cursor.goto_first_child_for_byte(self.position);
        self.gather_context_from_node(cursor, current_node);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::{ClauseType, CompletionContext},
        test_helper::{CURSOR_POS, get_text_and_position},
    };

    fn get_tree(input: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Couldn't set language");

        parser.parse(input, None).expect("Unable to parse tree")
    }

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

        for (query, expected_clause) in test_cases {
            let (position, text) = get_text_and_position(query.as_str().into());

            let tree = get_tree(text.as_str());

            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text,
                tree: Some(&tree),
                schema: &pgt_schema_cache::SchemaCache::default(),
            };

            let ctx = CompletionContext::new(&params);

            assert_eq!(ctx.wrapping_clause_type, expected_clause.try_into().ok());
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

        for (query, expected_schema) in test_cases {
            let (position, text) = get_text_and_position(query.as_str().into());

            let tree = get_tree(text.as_str());
            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text,
                tree: Some(&tree),
                schema: &pgt_schema_cache::SchemaCache::default(),
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

        for (query, is_invocation) in test_cases {
            let (position, text) = get_text_and_position(query.as_str().into());

            let tree = get_tree(text.as_str());
            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text,
                tree: Some(&tree),
                schema: &pgt_schema_cache::SchemaCache::default(),
            };

            let ctx = CompletionContext::new(&params);

            assert_eq!(ctx.is_invocation, is_invocation);
        }
    }

    #[test]
    fn does_not_fail_on_leading_whitespace() {
        let cases = vec![
            format!("{}      select * from", CURSOR_POS),
            format!(" {}      select * from", CURSOR_POS),
        ];

        for query in cases {
            let (position, text) = get_text_and_position(query.as_str().into());

            let tree = get_tree(text.as_str());

            let params = crate::CompletionParams {
                position: (position as u32).into(),
                text,
                tree: Some(&tree),
                schema: &pgt_schema_cache::SchemaCache::default(),
            };

            let ctx = CompletionContext::new(&params);

            let node = ctx.node_under_cursor.unwrap();

            assert_eq!(ctx.get_ts_node_content(node), Some("select"));

            assert_eq!(
                ctx.wrapping_clause_type,
                Some(crate::context::ClauseType::Select)
            );
        }
    }

    #[test]
    fn does_not_fail_on_trailing_whitespace() {
        let query = format!("select * from   {}", CURSOR_POS);

        let (position, text) = get_text_and_position(query.as_str().into());

        let tree = get_tree(text.as_str());

        let params = crate::CompletionParams {
            position: (position as u32).into(),
            text,
            tree: Some(&tree),
            schema: &pgt_schema_cache::SchemaCache::default(),
        };

        let ctx = CompletionContext::new(&params);

        let node = ctx.node_under_cursor.unwrap();

        assert_eq!(ctx.get_ts_node_content(node), Some("from"));
        assert_eq!(
            ctx.wrapping_clause_type,
            Some(crate::context::ClauseType::From)
        );
    }

    #[test]
    fn does_not_fail_with_empty_statements() {
        let query = format!("{}", CURSOR_POS);

        let (position, text) = get_text_and_position(query.as_str().into());

        let tree = get_tree(text.as_str());

        let params = crate::CompletionParams {
            position: (position as u32).into(),
            text,
            tree: Some(&tree),
            schema: &pgt_schema_cache::SchemaCache::default(),
        };

        let ctx = CompletionContext::new(&params);

        let node = ctx.node_under_cursor.unwrap();

        assert_eq!(ctx.get_ts_node_content(node), Some(""));
        assert_eq!(ctx.wrapping_clause_type, None);
    }

    #[test]
    fn does_not_fail_on_incomplete_keywords() {
        //  Instead of autocompleting "FROM", we'll assume that the user
        // is selecting a certain column name, such as `frozen_account`.
        let query = format!("select * fro{}", CURSOR_POS);

        let (position, text) = get_text_and_position(query.as_str().into());

        let tree = get_tree(text.as_str());

        let params = crate::CompletionParams {
            position: (position as u32).into(),
            text,
            tree: Some(&tree),
            schema: &pgt_schema_cache::SchemaCache::default(),
        };

        let ctx = CompletionContext::new(&params);

        let node = ctx.node_under_cursor.unwrap();

        assert_eq!(ctx.get_ts_node_content(node), Some("fro"));
        assert_eq!(ctx.wrapping_clause_type, Some(ClauseType::Select));
    }
}
