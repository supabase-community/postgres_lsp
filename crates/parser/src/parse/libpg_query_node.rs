use std::{assert_eq, ops::Range};

use crate::{
    codegen::{get_nodes, Node, SyntaxKind},
    lexer::TokenType,
};
use log::debug;
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    visit::Bfs,
    Direction,
};
use pg_query::NodeEnum;

use crate::Parser;

pub fn libpg_query_node(parser: &mut Parser, node: NodeEnum, token_range: &Range<usize>) {
    LibpgQueryNodeParser::new(parser, node, token_range).parse();
}

pub static SKIPPABLE_TOKENS: &[SyntaxKind] = &[
    // "("
    SyntaxKind::Ascii40,
    // ")"
    SyntaxKind::Ascii41,
    // ","
    SyntaxKind::Ascii44,
    // "."
    SyntaxKind::Ascii46,
    // ";"
    SyntaxKind::Ascii59,
];

struct LibpgQueryNodeParser<'p> {
    parser: &'p mut Parser,
    token_range: &'p Range<usize>,
    node_graph: StableGraph<Node, ()>,
    current_node: NodeIndex<DefaultIx>,
    open_nodes: Vec<NodeIndex<DefaultIx>>,
}

impl<'p> LibpgQueryNodeParser<'p> {
    pub fn new(
        parser: &'p mut Parser,
        node: NodeEnum,
        token_range: &'p Range<usize>,
    ) -> LibpgQueryNodeParser<'p> {
        let current_depth = parser.depth.clone();
        debug!("Parsing node {:#?}", node);
        Self {
            parser,
            token_range,
            node_graph: get_nodes(&node, current_depth),
            current_node: NodeIndex::<DefaultIx>::new(0),
            open_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        // IDEA: do not use String tokens and add their properties to their parents
        //
        // enhance using location:
        // - problem: handling of tokens that are not at the start of the node with a location
        // property
        // - if we assume that all nodes have either all token properties or a location, we can
        // apply all tokens until either we are at the location of a new node or a token is found
        // in properties. stop searching if current location < node location
        // - compare depth of node with current depth and panic if wrong
        dbg!(&self.node_graph);
        while self.parser.pos < self.token_range.end {
            if !self.at_whitespace() && !self.at_skippable() {
                debug!("current node: {:#?}", self.current_node);
                debug!("current token: {:#?}", self.current_token());
            }
            if self.at_whitespace() || self.at_skippable() {
                self.parser.advance();
            } else if let Some(idx) = self.node_properties_position(self.current_node) {
                println!("found property at current node {:?}", self.current_node);
                // token is in current node. remove and advance.
                // open if not opened yet.
                if !self.node_is_open(&self.current_node) {
                    self.start_node(self.current_node);
                }
                self.remove_property(self.current_node, idx);
                self.parser.advance();
                self.finish_open_leaf_nodes();
            } else if let Some((node_idx, prop_idx)) = self.search_children_properties() {
                println!("found property within children node {:?}", node_idx);
                self.remove_property(node_idx, prop_idx);

                // close all nodes until the target depth is reached
                self.finish_nodes_until_depth(self.node_graph[node_idx].depth + 1);

                if !self.node_is_open(&node_idx) {
                    // open all nodes from `self.current_node` to the node in whichs property the current token found (`node_idx`)
                    let mut ancestors = self.ancestors(Some(node_idx));
                    let mut nodes_to_open = Vec::<NodeIndex<DefaultIx>>::new();
                    // including the target node
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
                }

                self.parser.advance();

                self.current_node = node_idx;

                self.finish_open_leaf_nodes();
            } else if let Some((node_idx, prop_idx)) = self.search_parent_properties() {
                println!("found property within parent node {:?}", node_idx);
                self.remove_property(node_idx, prop_idx);

                self.finish_nodes_until_depth(self.node_graph[node_idx].depth + 1);

                // do not open any new nodes because the node is already open

                self.current_node = node_idx;

                // set the current node to the deepest node (looking up from the current node) that has at least one children
                // has_children is true if there are outgoing neighbors
                if self.has_children(&node_idx) {
                    self.current_node = node_idx;
                } else {
                    for a in self.ancestors(Some(node_idx)) {
                        if self.has_children(&a) {
                            self.current_node = a;
                            break;
                        }
                    }
                }

                self.parser.advance();
            } else {
                panic!(
                    "could not find node for token {:?} at depth {} in {:#?}",
                    self.current_token(),
                    self.parser.depth,
                    self.node_graph
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
        // let mut skipped_nodes = Vec::<NodeIndex<DefaultIx>>::new();

        // (node index, property index)
        // always check all nodes on the same depth of the first node that is found
        let mut possible_nodes: Vec<(NodeIndex<DefaultIx>, usize)> = Vec::new();
        let mut target_depth: Option<usize> = None;
        while let Some(nx) = bfs.next(&self.node_graph) {
            if target_depth.is_some() && self.node_graph[nx].depth != target_depth.unwrap() {
                break;
            }

            // if all direct children of the current node are being skipped, break
            // if current_node_children
            //     .iter()
            //     .all(|n| skipped_nodes.contains(&n))
            // {
            //     break;
            // }

            // if the current node has an edge to any node that is being skipped, skip the current
            // this will ensure that we skip invalid branches entirely
            // if skipped_nodes
            //     .iter()
            //     .any(|n| self.node_graph.contains_edge(nx, *n))
            // {
            //     skipped_nodes.push(nx);
            //     continue;
            // }

            let prop_idx = self.node_properties_position(nx);

            // if prop_idx.is_none() && self.node_graph[nx].properties.len() > 0 {
            // if the current node has properties and the token does not match any of them, add it
            // to the list of skipped nodes and continue
            // skipped_nodes.push(nx);
            //     continue;
            // }

            if prop_idx.is_some() {
                possible_nodes.push((nx, prop_idx.unwrap()));
                target_depth = Some(self.node_graph[nx].depth);
            }
        }

        if possible_nodes.len() == 1 {
            Some((possible_nodes[0].0, possible_nodes[0].1))
        } else if possible_nodes.len() > 1 {
            // FIXME: I dont think that just using the one with the smallest index will always work
            //       because the order of the nodes in the graph is not deterministic
            //       we should instead figure out which one is the correct node based on future
            //       tokens
            possible_nodes.into_iter().min_by_key(|x| x.0)
        } else {
            None
        }
    }

    /// check if the current node has children that have properties that are in the part of the token stream that is not yet consumed
    fn has_children_with_relevant_properties(&self) -> bool {
        let tokens = &self.parser.tokens[self.parser.pos..self.token_range.end];
        let mut b = Bfs::new(&self.node_graph, self.current_node);
        while let Some(nx) = b.next(&self.node_graph) {
            if self.node_graph[nx]
                .properties
                .iter()
                .any(|p| tokens.iter().any(|t| cmp_tokens(p, t)))
            {
                return true;
            }
        }
        false
    }

    /// finish current node while it is an open leaf node with no properties
    fn finish_open_leaf_nodes(&mut self) {
        while self.open_nodes.len() > 1
            && (self
                .node_graph
                .neighbors_directed(self.current_node, Direction::Outgoing)
                .count()
                == 0
                || !self.has_children_with_relevant_properties())
        {
            // check if the node contains properties that are not at all in the part of the token stream that is not yet consumed and remove them
            if self.node_graph[self.current_node].properties.len() > 0 {
                let tokens = &self.parser.tokens[self.parser.pos..self.token_range.end];
                self.node_graph[self.current_node]
                    .properties
                    .retain(|p| tokens.iter().any(|t| cmp_tokens(p, t)));
            }

            if self.node_graph[self.current_node].properties.len() > 0 {
                break;
            }

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
        let node_to_remove = self.open_nodes.pop().unwrap();
        assert_eq!(
            self.node_graph[node_to_remove].depth,
            self.parser.depth - 1,
            "Tried to finish node with depth {} but parser depth is {}",
            self.node_graph[node_to_remove].depth,
            self.parser.depth
        );
        self.node_graph.remove_node(node_to_remove);
        self.parser.finish_node();
    }

    fn remove_property(&mut self, node_idx: NodeIndex<DefaultIx>, idx: usize) {
        self.node_graph[node_idx].properties.remove(idx);
    }

    fn start_node(&mut self, idx: NodeIndex<DefaultIx>) {
        assert_eq!(
            self.node_graph[idx].depth, self.parser.depth,
            "Tried to start node with depth {} but parser depth is {}",
            self.node_graph[idx].depth, self.parser.depth
        );
        self.parser.start_node(self.node_graph[idx].kind);
        self.open_nodes.push(idx);
    }

    fn current_location(&self) -> usize {
        usize::from(self.current_token().span.start())
    }

    fn current_token(&self) -> &crate::lexer::Token {
        self.parser.tokens.get(self.parser.pos).unwrap()
    }

    fn at_skippable(&self) -> bool {
        SKIPPABLE_TOKENS.contains(&self.current_token().kind)
    }

    fn at_whitespace(&self) -> bool {
        // TODO: merge whitespace token def with whitespace def in parser
        self.current_token().token_type == TokenType::Whitespace
    }

    fn node_properties_position(&self, idx: NodeIndex<DefaultIx>) -> Option<usize> {
        self.node_graph[idx]
            .properties
            .iter()
            .position(|p| cmp_tokens(p, self.current_token()))
    }
}

/// list of aliases from https://www.postgresql.org/docs/current/datatype.html
const ALIASES: [&[&str]; 2] = [&["integer", "int", "int4"], &["real", "float4"]];

fn cmp_tokens(p: &crate::codegen::TokenProperty, token: &crate::lexer::Token) -> bool {
    // TokenProperty has always either value or kind set
    assert!(p.value.is_some() || p.kind.is_some());

    // TODO: move this to lexer

    // remove enclosing ' quotes from token text
    let string_delimiter: &[char; 2] = &['\'', '$'];
    let token_text = token
        .text
        .trim_start_matches(string_delimiter)
        .trim_end_matches(string_delimiter)
        .to_string()
        .to_lowercase();
    let token_text_values = aliases(&token_text);

    (p.value.is_none() || token_text_values.contains(&p.value.as_ref().unwrap().as_str()))
        && (p.kind.is_none() || p.kind.unwrap() == token.kind)
}

/// returns a list of aliases for a string. primarily used for data types.
fn aliases(text: &str) -> Vec<&str> {
    for alias in ALIASES {
        if alias.contains(&text) {
            return alias.to_vec();
        }
    }
    return vec![text];
}

/// Custom iterator for walking ancestors of a node until the root of the tree is reached
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
