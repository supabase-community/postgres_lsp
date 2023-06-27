use crate::syntax_kind::SyntaxKind;

pub trait EventSink {
    fn start_node(&mut self, kind: SyntaxKind);
    fn finish_node(&mut self);
    fn token(&mut self, kind: SyntaxKind, text: &str);
    fn error(&mut self, error: String, range: std::ops::Range<usize>);
}
