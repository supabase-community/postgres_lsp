use crate::{
    codegen::{get_nodes, Node},
    lexer::TokenType,
};
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    visit::Bfs,
    Direction,
};
use pg_query::NodeEnum;

use crate::Parser;

pub fn libpg_query_node(parser: &mut Parser, node: NodeEnum, until: usize) {
    LibpgQueryNodeParser::new(parser, node, until).parse();
}

struct LibpgQueryNodeParser<'p> {
    parser: &'p mut Parser,
    until: usize,
    node_graph: StableGraph<Node, ()>,
    current_node: NodeIndex<DefaultIx>,
    open_nodes: Vec<NodeIndex<DefaultIx>>,
}

impl<'p> LibpgQueryNodeParser<'p> {
    pub fn new(parser: &mut Parser, node: NodeEnum, until: usize) -> LibpgQueryNodeParser {
        Self {
            parser,
            until,
            node_graph: get_nodes(&node, parser.depth),
            current_node: NodeIndex::<DefaultIx>::new(0),
            open_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.parser.pos < self.until {
            if self.at_whitespace() {
                self.parser.advance();
                continue;
            }
            if let Some(idx) = self.node_properties_position(self.current_node) {
                // token is in current node. remove and advance.
                // open if not opened yet.
                if !self.node_is_open(&self.current_node) {
                    self.start_node(self.current_node);
                }
                self.remove_property(self.current_node, idx);
                self.parser.advance();
                continue;
            }

            let in_properties = self.breadth_first_search();
        }
    }

    /// breadth-first search (`Bfs`) for the node that has the current token as its property
    ///
    /// Returns index of node and property if found
    ///
    /// Skips visited branches
    fn breadth_first_search(&self) -> Option<(NodeIndex<DefaultIx>, usize)> {
        let mut bfs = Bfs::new(&self.node_graph, self.current_node);
        let current_node_children = self
            .node_graph
            .neighbors_directed(self.current_node, Direction::Outgoing)
            .collect::<Vec<NodeIndex<DefaultIx>>>();
        let mut skipped_nodes = Vec::<NodeIndex<DefaultIx>>::new();

        while let Some(nx) = bfs.next(&self.node_graph) {
            // if all direct children of the current node are being skipped, break
            if current_node_children
                .iter()
                .all(|n| skipped_nodes.contains(&n))
            {
                break;
            }

            // if the current node has an edge to any node that is being skipped, skip the current
            // this will ensure that we skip invalid branches entirely
            if skipped_nodes
                .iter()
                .any(|n| self.node_graph.contains_edge(nx, *n))
            {
                skipped_nodes.push(nx);
                continue;
            }

            let prop_idx = self.node_properties_position(nx);

            if prop_idx.is_none() && self.node_graph[nx].properties.len() > 0 {
                // if the current node has properties and the token does not match any of them, add it
                // to the list of skipped nodes and continue
                skipped_nodes.push(nx);
                continue;
            }

            if prop_idx.is_some() {
                return Some((nx, prop_idx.unwrap()));
            }
        }

        None
    }

    fn remove_property(&mut self, node_idx: NodeIndex<DefaultIx>, idx: usize) {
        self.node_graph[node_idx].properties.remove(idx);
    }

    fn node_is_open(&self, idx: &NodeIndex<DefaultIx>) -> bool {
        self.open_nodes.contains(idx)
    }

    fn start_node(&mut self, idx: NodeIndex<DefaultIx>) {
        self.parser.start_node(self.node_graph[idx].kind);
        self.open_nodes.push(idx);
    }

    fn current_token(&self) -> &crate::lexer::Token {
        self.parser.tokens.get(self.parser.pos).unwrap()
    }

    fn at_whitespace(&self) -> bool {
        // TODO: merge whitespace token def with whitespace def in parser
        self.parser.tokens.get(self.parser.pos).unwrap().token_type == TokenType::Whitespace
    }

    fn node_properties_position(&self, idx: NodeIndex<DefaultIx>) -> Option<usize> {
        self.node_graph[idx]
            .properties
            .iter()
            .position(|p| cmp_tokens(p, self.current_token()))
    }
}

fn cmp_tokens(p: &crate::codegen::TokenProperty, token: &crate::lexer::Token) -> bool {
    (!p.value.is_some() || p.value.as_ref().unwrap() == &token.text)
        && (!p.kind.is_some() || p.kind.unwrap() == token.kind)
}
