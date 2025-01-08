mod relations;

use std::ops::Range;

pub use relations::*;

pub enum QueryResult<'a> {
    Relation(RelationMatch<'a>),
}

impl<'a> QueryResult<'a> {
    pub fn within_range(&self, range: &Range<usize>) -> bool {
        match self {
            Self::Relation(rm) => {
                let tb_range = rm.table.byte_range();

                let start = match rm.schema {
                    Some(s) => s.byte_range().start,
                    None => tb_range.start,
                };

                let end = tb_range.end;

                range.contains(&start) && range.contains(&end)
            }
        }
    }
}

// This trait enforces that for any `Self` that implements `Query`,
// its &Self must implement TryFrom<&QueryResult>
pub(crate) trait QueryTryFrom<'a>: Sized {
    type Ref: for<'any> TryFrom<&'a QueryResult<'a>, Error = String>;
}

pub(crate) trait Query<'a>: QueryTryFrom<'a> {
    async fn execute(
        root_node: tree_sitter::Node<'a>,
        stmt: &'a str,
    ) -> Vec<crate::QueryResult<'a>>;
}
