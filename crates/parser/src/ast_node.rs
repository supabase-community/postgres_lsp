use cstree::text::TextRange;
use pg_query::NodeEnum;

// TODO: implement serde for node: https://serde.rs/remote-derive.html

#[derive(Debug)]
pub struct RawStmt {
    pub stmt: NodeEnum,
    pub range: TextRange,
}
