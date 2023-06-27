use crate::{event_sink::EventSink, source_file_lexer::SourceFileToken, syntax_kind::SyntaxKind};
use logos::Logos;

pub fn parse_source_file<T: EventSink>(input: &str, sink: &mut T) {
    let mut lexer = SourceFileToken::lexer(&input);

    sink.start_node(SyntaxKind::SourceFile);
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                match token {
                    SourceFileToken::Comment => {
                        sink.token(SyntaxKind::Comment, lexer.slice());
                    }
                    SourceFileToken::Newline => {
                        sink.token(SyntaxKind::Newline, lexer.slice());
                    }
                    SourceFileToken::Statement => {
                        todo!();
                    }
                };
            }
            Err(_) => panic!("Unknown SourceFileToken: {:?}", lexer.span()),
        }
    }
    sink.finish_node();
}
