pub mod builder;

use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    visit::IntoNodeReferences,
    Direction,
};
use text_size::{TextRange, TextSize};

#[derive(Debug, Clone)]
pub struct RangedNode {
    pub node: pg_query_ext::NodeEnum,
    pub start: TextSize,
    pub end: Option<TextSize>,
}

impl RangedNode {
    pub fn range(&self) -> TextRange {
        TextRange::new(self.start, self.end.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct AST {
    inner: StableGraph<RangedNode, ()>,
}

impl AST {
    pub fn new(g: StableGraph<RangedNode, ()>) -> Self {
        Self { inner: g }
    }

    pub fn root_node(&self) -> &RangedNode {
        &self.inner[NodeIndex::<DefaultIx>::new(0)]
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &RangedNode> {
        self.inner.node_references().map(|(_, node)| node)
    }

    pub fn covering_node(&self, range: TextRange) -> Option<RangedNode> {
        let mut res: NodeIndex = NodeIndex::<DefaultIx>::new(0);

        // check if any children contains the range. if not return, else continue
        while let Some(idx) = self
            .inner
            .neighbors_directed(res, Direction::Outgoing)
            .find(|&idx| {
                let node = &self.inner[idx];
                node.range().contains_range(range)
            })
        {
            res = idx;
        }

        Some(self.inner[res].clone())
    }
}
