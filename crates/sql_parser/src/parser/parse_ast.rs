use cstree::syntax::ResolvedNode;
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    visit::{Bfs, Dfs},
    Direction,
};
use pg_query::{Error, NodeEnum};
use text_size::TextRange;

use crate::{
    codegen::{get_nodes, Node, SyntaxKind},
    lexer::{lex, TokenType},
    syntax_error::SyntaxError,
};

use super::{parse_sql_statement, Parser};

pub struct AstNode {
    pub node: NodeEnum,
    pub range: TextRange,
}

struct Ast {
    root: NodeEnum,
    /// A list of all nodes in the tree with resolved ranges.
    /// This is not optimal, but i dont know how to get a ref into the root nodes children without
    // cloning them
    nodes: Vec<AstNode>,
}

type Cst = ResolvedNode<SyntaxKind>;

struct ParsedAst {
    /// The abstract syntax tree with resolved ranges for each node
    pub ast: Ast,
    /// The concrete syntax tree
    pub cst: Cst,
    /// The syntax errors accumulated during parsing
    pub errors: Vec<SyntaxError>,
}

pub fn parse_ast(sql: &str) -> ParsedAst {
    // AFTER THE PARSING IS COMPLETE, WE CAN USE SET_DATA TO SET DATA ON EVERY NODE
    // How to get the damn ref from the cst to the ast node?
    // --> i think the only valid option is to build a second petgraph with refs to the ast node
    // and their ranges and then query the enriched ast directly for the ranges

    // -> refactor parser to store event stream instead of cst
    // -> use event stream to build cst and "enriched" ast in one go
    // -> or use composite patterns to wrap parser within parser since all other usages dont need
    // start and stop node

    let root = parse_sql_statement::parse_sql_statement(sql);
    let mut parser = Parser::new(lex(sql));
    SqlStatementParser::new(&mut parser, &root.unwrap()).parse();
    let r = parser.finish();
}

// TODO: implement sibling token handling
pub static SKIPPABLE_TOKENS: &[SyntaxKind] = &[
    // "["
    SyntaxKind::Ascii91,
    // "]"
    SyntaxKind::Ascii93,
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

struct SqlStatementParser<'p> {
    parser: &'p mut Parser,
    node_graph: StableGraph<Node, ()>,
    current_node: NodeIndex<DefaultIx>,
    open_nodes: Vec<NodeIndex<DefaultIx>>,
}

impl<'p> SqlStatementParser<'p> {
    pub fn new(parser: &'p mut Parser, node: &pg_query::NodeEnum) -> SqlStatementParser<'p> {
        Self {
            parser,
            node_graph: get_nodes(&node),
            current_node: NodeIndex::<DefaultIx>::new(0),
            open_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.parser.pos < self.parser.token_range().end {
            if self.at_whitespace() {
                self.parser.advance();
            } else if let Some(idx) = self.node_properties_position(self.current_node) {
                // token is in current node. remove and advance.
                // open if not opened yet.
                if !self.node_is_open(&self.current_node) {
                    self.start_node(self.current_node);
                }
                self.remove_property(self.current_node, idx);
                self.parser.advance();
                self.finish_open_leaf_nodes();
            } else if let Some((node_idx, prop_idx)) = self.search_children() {
                if prop_idx.is_some() {
                    self.remove_property(node_idx, prop_idx.unwrap());
                }

                // close all nodes until the target depth is reached
                self.finish_nodes_until_depth(self.node_graph[node_idx].depth + 1);

                if !self.node_is_open(&node_idx) {
                    // open all nodes from `self.current_node` to the target node `node_idx`
                    let mut ancestors = self.ancestors(Some(node_idx));
                    let mut nodes_to_open = Vec::<NodeIndex<DefaultIx>>::new();
                    // including the target node itself
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
            } else if self.at_skippable() {
                self.parser.advance();
            } else if let Some((node_idx, prop_idx)) = self.search_parent_properties() {
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
                    "could not find node for token {:?} at depth {}",
                    self.current_token(),
                    self.parser.depth,
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

    /// breadth-first search (`Bfs`) for the node that is at the current location or has the current token as its property
    ///
    /// Returns indices of both node and property if found
    ///
    /// Skips visited branches
    fn search_children(&self) -> Option<(NodeIndex<DefaultIx>, Option<usize>)> {
        let mut bfs = Bfs::new(&self.node_graph, self.current_node);
        let current_node_children = self
            .node_graph
            .neighbors_directed(self.current_node, Direction::Outgoing)
            .collect::<Vec<NodeIndex<DefaultIx>>>();
        let mut skipped_nodes = Vec::<NodeIndex<DefaultIx>>::new();

        // (node index, property index)
        // always check all nodes on the same depth of the first node that is found
        let mut possible_nodes: Vec<(NodeIndex<DefaultIx>, Option<usize>)> = Vec::new();
        let mut target_depth: Option<usize> = None;
        while let Some(nx) = bfs.next(&self.node_graph) {
            if target_depth.is_some() && self.node_graph[nx].depth != target_depth.unwrap() {
                break;
            }

            // if all direct children of the current node are being skipped, break
            if current_node_children
                .iter()
                .all(|n| skipped_nodes.contains(&n))
            {
                break;
            }

            // if the current node has an edge to any node that is being skipped, skip the current
            // this will ensure that we skip invalid branches entirely
            // note: order of nodes in contains_edge is important since we are using a directed
            // graph
            if skipped_nodes
                .iter()
                .any(|n| self.node_graph.contains_edge(*n, nx))
            {
                skipped_nodes.push(nx);
                continue;
            }

            if self.node_graph[nx].location.is_some()
                && self.node_graph[nx].location.unwrap() > self.current_location()
            {
                // if the node has a location and it is after the current location, add it to the list of skipped nodes and continue
                skipped_nodes.push(nx);
                continue;
            }

            // check if the node has a property that is the current token
            let prop_idx = self.node_properties_position(nx);

            if prop_idx.is_some() {
                possible_nodes.push((nx, prop_idx));
                if target_depth.is_none() {
                    target_depth = Some(self.node_graph[nx].depth);
                }
            } else if self.node_graph[nx].location.is_some()
                && self.node_graph[nx].location.unwrap() == self.current_location()
            {
                // check if the location of the node is the current location
                // do a depth-first search to find the first node that either has a location that
                // is not the current one, or has the current token as a property
                let mut dfs = Dfs::new(&self.node_graph, nx);
                let mut target_nx = nx;
                while let Some(node_idx) = dfs.next(&self.node_graph) {
                    if self.node_graph[node_idx].location.is_some()
                        && self.node_graph[node_idx].location.unwrap() != self.current_location()
                    {
                        break;
                    }

                    target_nx = node_idx;

                    if self.node_properties_position(node_idx).is_some() {
                        break;
                    }
                }
                return Some((target_nx, self.node_properties_position(target_nx)));
            }
        }

        if possible_nodes.len() == 1 {
            Some(possible_nodes[0])
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

    /// finish current node while it is an open leaf node with no properties and either no location
    /// or a location that is before the current location
    fn finish_open_leaf_nodes(&mut self) {
        while self.open_nodes.len() > 1
            && self
                .node_graph
                .neighbors_directed(self.current_node, Direction::Outgoing)
                .count()
                == 0
        {
            // check if the node contains properties that are not at all in the part of the token stream that is not yet consumed and remove them
            if self.node_graph[self.current_node].properties.len() > 0 {
                // if there is any property left it must be next in the token stream because we are at a
                // leaf node. We can thereby reduce the search space to the next n non-whitespace token
                // where n is the number of remaining properties of the current node
                let num_of_properties = self.node_graph[self.current_node].properties.len();
                self.node_graph[self.current_node].properties.retain(|p| {
                    let mut idx = 0;
                    let mut left_pull = 0;
                    while idx < num_of_properties + left_pull {
                        let token = self.parser.nth(idx, true);
                        if token.kind == SyntaxKind::Eof {
                            break;
                        }
                        if cmp_tokens(&p, token) {
                            return true;
                        }
                        // FIXME: we also need to skip non-whitespace tokens such as "(" or ")", but
                        // not all (e.g. Ident is also a non-whitespace token with type NoKeyword)
                        // for now, we just do one more iteration if the token has a length == 1
                        // can be improved by comparing against a list
                        if token.text.len() == 1 {
                            left_pull += 1;
                        }
                        idx += 1;
                    }
                    false
                });
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
        if self.node_graph[idx].location.is_some() {
            assert_eq!(
                self.node_graph[idx].location.unwrap(),
                self.current_location(),
                "Tried to start node {:#?} with location {} but current location is {}",
                self.node_graph[idx],
                self.node_graph[idx].location.unwrap(),
                self.current_location()
            );
        }
        self.parser
            .start_node(SyntaxKind::from(&self.node_graph[idx].inner));
        self.open_nodes.push(idx);
    }

    fn current_location(&self) -> usize {
        usize::from(
            self.current_token().span.start()
                - self.parser.tokens[self.parser.token_range().start]
                    .span
                    .start(),
        )
    }

    fn current_token(&self) -> &crate::lexer::Token {
        self.parser.tokens.get(self.parser.pos).unwrap()
    }

    fn at_skippable(&self) -> bool {
        SKIPPABLE_TOKENS.contains(&self.current_token().kind)
    }

    fn at_whitespace(&self) -> bool {
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
/// NOTE: support for multi-word alias (e.g. time with time zone) requires parser change
const ALIASES: [&[&str]; 10] = [
    &["bigint", "int8"],
    &["bigserial", "serial8"],
    &["boolean", "bool"],
    &["character", "char"],
    &["integer", "int", "int4"],
    &["numeric", "decimal"],
    &["real", "float4"],
    &["smallint", "int2"],
    &["smallserial", "serial2"],
    &["serial", "serial4"],
];

fn cmp_tokens(p: &crate::codegen::TokenProperty, token: &crate::lexer::Token) -> bool {
    // TokenProperty has always either value or kind set
    assert!(p.value.is_some() || p.kind.is_some());

    // TODO: move this to lexer
    // we should also move alias handling to the lexer

    // remove enclosing ' quotes from token text
    let string_delimiter: &[char; 3] = &['\'', '$', '\"'];
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
