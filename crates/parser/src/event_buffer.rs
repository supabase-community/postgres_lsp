use crate::{
    event_sink::EventSink,
    syntax_kind::{SyntaxKind, SyntaxKindType},
};

pub struct EventBuffer<'sink, S: EventSink> {
    sink: &'sink mut S,
    token_buffer: Vec<(SyntaxKind, String)>,
    curr_depth: i32,
}

/// Wrapper around EventSink to simplify integration with SyntaxKind
impl<'sink, S: EventSink> EventBuffer<'sink, S> {
    pub fn new(sink: &'sink mut S) -> Self {
        return Self {
            sink,
            token_buffer: Vec::new(),
            curr_depth: 0,
        };
    }

    pub fn close_until_depth(&mut self, depth: i32) {
        while self.curr_depth >= depth {
            self.sink.finish_node();
            self.curr_depth -= 1;
        }
    }

    /// start a new node of `SyntaxKind` at `depth`
    /// handles closing previous nodes if necessary
    /// and consumes token buffer before starting new node
    pub fn start_node(&mut self, kind: SyntaxKind, depth: &i32) {
        // close until target depth
        self.close_until_depth(*depth);

        self.consume_token_buffer();

        self.curr_depth = *depth;
        self.sink.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.sink.finish_node();
    }

    /// Drains the token buffer and applies all tokens
    pub fn consume_token_buffer(&mut self) {
        for (kind, text) in self.token_buffer.drain(..) {
            self.sink.token(kind, &text);
        }
    }

    /// applies token based on its `SyntaxKindType`
    /// if `SyntaxKindType::Close`, closes all nodes until depth 1
    /// if `SyntaxKindType::Follow`, add token to buffer and wait until next node to apply token at
    /// same depth
    /// otherwise, applies token immediately
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        match kind.get_type() {
            Some(SyntaxKindType::Close) => {
                // move up to depth 2 and consume buffered tokens before applying closing token
                self.close_until_depth(2);
                self.consume_token_buffer();
                self.sink.token(kind, text);
            }
            Some(SyntaxKindType::Follow) => {
                // wait until next node, and apply token at same depth
                self.token_buffer.push((kind, text.to_string()));
            }
            _ => {
                self.sink.token(kind, text);
            }
        }
    }

    pub fn error(&mut self, error: String, range: std::ops::Range<usize>) {
        self.sink.error(error, range);
    }
}
