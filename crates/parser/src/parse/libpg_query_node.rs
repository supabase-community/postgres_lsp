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
            } else if let Some((node_idx, prop_idx)) = self.search_children_properties() {
                // close all nodes until the target depth is reached
                self.finish_nodes_until_depth(self.node_graph[node_idx].depth + 1);

                if self.node_is_open(&node_idx) {
                    // if the node is already open, advance and continue with the next token
                    self.parser.advance();
                    continue;
                }

                // open all nodes from `self.current_node` to the node in whichs property the current token found (`node_idx`)
                let mut ancestors = self.ancestors(Some(node_idx));
                let mut nodes_to_open = Vec::<NodeIndex<DefaultIx>>::new();
                nodes_to_open.push(node_idx);
                while let Some(nx) = ancestors.next() {
                    if nx == self.current_node {
                        break;
                    }
                    nodes_to_open.push(nx);
                }
                nodes_to_open.iter().rev().for_each(|n| {
                    self.start_node(*n);
                });

                self.parser.advance();

                self.current_node = node_idx;

                self.finish_open_leaf_nodes();
            } else if let Some((node_idx, prop_idx)) = self.search_parent_properties() {
                self.remove_property(node_idx, prop_idx);

                self.finish_nodes_until_depth(self.node_graph[node_idx].depth + 1);

                // do not open any new nodes because the node is already open

                // set the current node to the deepest node (looking up from the current node) that has at least one children
                // has_children is true if there are outgoing neighbors
                for a in self.ancestors(Some(node_idx)) {
                    if self.has_children(&a) {
                        self.current_node = a;
                        break;
                    }
                }

                self.parser.advance();
            } else {
                panic!(
                    "could not find node for token {:?} at depth {}",
                    self.current_token(),
                    self.parser.depth
                );
            }
        }
        // close all remaining nodes
        for _ in 0..self.open_nodes.len() {
            self.finish_node();
        }
    }

    fn search_parent_properties(&self) -> Option<(NodeIndex<DefaultIx>, usize)> {
        self.ancestors(None).find_map(|n| {
            let prop_idx = self.node_graph[n]
                .properties
                .iter()
                .position(|p| cmp_tokens(p, self.current_token()));
            if prop_idx.is_some() {
                Some((n, prop_idx.unwrap()))
            } else {
                None
            }
        })
    }

    /// breadth-first search (`Bfs`) for the node that has the current token as its property
    ///
    /// Returns indices of both node and property if found
    ///
    /// Skips visited branches
    fn search_children_properties(&self) -> Option<(NodeIndex<DefaultIx>, usize)> {
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

    /// finish current node while it is an open leaf node with no properties
    fn finish_open_leaf_nodes(&mut self) {
        while self
            .node_graph
            .neighbors_directed(self.current_node, Direction::Outgoing)
            .count()
            == 0
            && self.node_graph[self.current_node].properties.len() == 0
        {
            self.finish_node();
            if self.open_nodes.len() == 0 {
                break;
            }
            self.current_node = self.open_nodes.last().unwrap().clone();
        }
    }

    fn has_children(&self, idx: &NodeIndex<DefaultIx>) -> bool {
        self.node_graph
            .neighbors_directed(*idx, Direction::Outgoing)
            .count()
            > 0
    }

    fn ancestors(&self, from: Option<NodeIndex<DefaultIx>>) -> Ancestors {
        Ancestors {
            graph: &self.node_graph,
            current_node: from.unwrap_or(self.current_node),
        }
    }

    fn node_is_open(&self, idx: &NodeIndex<DefaultIx>) -> bool {
        self.open_nodes.contains(idx)
    }

    fn finish_nodes_until_depth(&mut self, until: usize) {
        while self.parser.depth > until {
            self.finish_node();
        }
    }

    fn finish_node(&mut self) {
        self.node_graph.remove_node(self.open_nodes.pop().unwrap());
        self.parser.finish_node();
    }

    fn remove_property(&mut self, node_idx: NodeIndex<DefaultIx>, idx: usize) {
        self.node_graph[node_idx].properties.remove(idx);
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

struct Ancestors<'a> {
    graph: &'a StableGraph<Node, ()>,
    current_node: NodeIndex<DefaultIx>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = NodeIndex<DefaultIx>;

    fn next(&mut self) -> Option<Self::Item> {
        let parent = self
            .graph
            .neighbors_directed(self.current_node, petgraph::Direction::Incoming)
            .next();
        if let Some(parent_node) = parent {
            self.current_node = parent_node;
            Some(parent_node)
        } else {
            None
        }
    }
}
