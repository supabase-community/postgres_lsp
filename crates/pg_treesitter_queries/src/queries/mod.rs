mod relations;

pub use relations::*;

#[derive(Debug)]
pub enum QueryResult<'a> {
    Relation(RelationMatch<'a>),
}

impl<'a> QueryResult<'a> {
    pub fn within_range(&self, range: &tree_sitter::Range) -> bool {
        match self {
            Self::Relation(rm) => {
                let start = match rm.schema {
                    Some(s) => s.start_position(),
                    None => rm.table.start_position(),
                };

                let end = rm.table.end_position();

                start >= range.start_point && end <= range.end_point
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
    fn execute(root_node: tree_sitter::Node<'a>, stmt: &'a str) -> Vec<crate::QueryResult<'a>>;
}
