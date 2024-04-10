use petgraph::stable_graph::{DefaultIx, NodeIndex, StableGraph};
use pg_query::NodeEnum;
use text_size::{TextRange, TextSize};

pub struct RangedNode {
    pub node: NodeEnum,
    pub start: TextSize,
    pub end: Option<TextSize>,
}

impl RangedNode {
    pub fn range(&self) -> TextRange {
        TextRange::new(self.start, self.end.unwrap())
    }
}

pub type EnrichedAst = StableGraph<RangedNode, ()>;

pub(super) struct AstBuilder {
    inner: EnrichedAst,
    open_nodes: Vec<NodeIndex<DefaultIx>>,
    current_pos: usize,
    current_idx: NodeIndex<DefaultIx>,
}

impl AstBuilder {
    pub fn new() -> Self {
        Self {
            inner: StableGraph::new(),
            open_nodes: Vec::new(),
            current_pos: 0,
            current_idx: NodeIndex::new(0),
        }
    }

    pub fn start_node(&mut self, node: NodeEnum) {
        let idx = self.inner.add_node(RangedNode {
            node,
            start: self.current_pos.try_into().unwrap(),
            end: None,
        });
        if self.open_nodes.len() > 0 {
            let parent = self.open_nodes.last().unwrap();
            self.inner.add_edge(parent.to_owned(), idx, ());
        }
        self.open_nodes.push(idx);
        self.current_idx = idx;
    }

    pub fn finish_node(&mut self) {
        let idx = self.open_nodes.pop().unwrap();
        let end = self.current_pos;
        self.inner[idx].end = Some(end.try_into().unwrap());
        self.current_idx = idx;
    }

    pub fn token(&mut self, text: &str) {
        self.current_pos += text.len();
    }

    pub fn finish(self) -> EnrichedAst {
        self.inner
    }
}
