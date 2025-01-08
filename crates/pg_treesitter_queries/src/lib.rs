pub mod queries;

use std::{ops::Range, slice::Iter};

use queries::{Query, QueryResult};

pub struct TreeSitterQueriesExecutor<'a> {
    root_node: tree_sitter::Node<'a>,
    stmt: &'a str,
    results: Vec<QueryResult<'a>>,
}

impl<'a> TreeSitterQueriesExecutor<'a> {
    pub fn new(root_node: tree_sitter::Node<'a>, stmt: &'a str) -> Self {
        Self {
            root_node,
            stmt,
            results: vec![],
        }
    }

    #[allow(private_bounds)]
    pub fn add_query_results<Q: Query<'a>>(&mut self) {
        let mut results = Q::execute(self.root_node, &self.stmt);
        self.results.append(&mut results);
    }

    pub fn get_iter(&self, range: Option<&'a Range<usize>>) -> QueryResultIter {
        match range {
            Some(r) => QueryResultIter::new(&self.results).within_range(r),
            None => QueryResultIter::new(&self.results),
        }
    }
}

pub struct QueryResultIter<'a> {
    inner: Iter<'a, QueryResult<'a>>,
    range: Option<&'a Range<usize>>,
}

impl<'a> QueryResultIter<'a> {
    pub(crate) fn new(results: &'a Vec<QueryResult<'a>>) -> Self {
        Self {
            inner: results.iter(),
            range: None,
        }
    }

    fn within_range(mut self, r: &'a Range<usize>) -> Self {
        self.range = Some(r);
        self
    }
}

impl<'a> Iterator for QueryResultIter<'a> {
    type Item = &'a QueryResult<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;

        if self.range.as_ref().is_some_and(|r| !next.within_range(r)) {
            return self.next();
        }

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use crate::{queries::RelationMatch, TreeSitterQueriesExecutor};

    #[test]
    fn finds_all_relations_and_ignores_functions() {
        let sql = r#"
select
  *
from
  (
    select
      something
    from
      public.cool_table pu
      join private.cool_tableau pr on pu.id = pr.id
    where
      x = '123'
    union
    select
      something_else
    from
      another_table puat
      inner join private.another_tableau prat on puat.id = prat.id
    union
    select
      x,
      y
    from
      public.get_something_cool ()
  )
where
  col = 17;
"#;

        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_sql::language()).unwrap();

        let tree = parser.parse(&sql, None).unwrap();

        let mut executor = TreeSitterQueriesExecutor::new(tree.root_node(), &sql);

        executor.add_query_results::<RelationMatch>();

        let results: Vec<&RelationMatch> = executor
            .get_iter(None)
            .filter_map(|q| q.try_into().ok())
            .collect();

        assert_eq!(
            results[0]
                .schema
                .map(|s| s.utf8_text(&sql.as_bytes()).unwrap()),
            Some("public")
        );
        assert_eq!(
            results[0].table.utf8_text(&sql.as_bytes()).unwrap(),
            "cool_table"
        );

        assert_eq!(
            results[1]
                .schema
                .map(|s| s.utf8_text(&sql.as_bytes()).unwrap()),
            Some("private")
        );
        assert_eq!(
            results[1].table.utf8_text(&sql.as_bytes()).unwrap(),
            "cool_tableau"
        );

        assert_eq!(results[2].schema, None);
        assert_eq!(
            results[2].table.utf8_text(&sql.as_bytes()).unwrap(),
            "another_table"
        );

        assert_eq!(
            results[3]
                .schema
                .map(|s| s.utf8_text(&sql.as_bytes()).unwrap()),
            Some("private")
        );
        assert_eq!(
            results[3].table.utf8_text(&sql.as_bytes()).unwrap(),
            "another_tableau"
        );

        // we have exhausted the matches: function invocations are ignored.
        assert!(results.len() == 4);
    }
}
