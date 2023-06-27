use crate::statement_parser::parse_statement;
use crate::syntax_error::SyntaxError;
use cstree::build::GreenNodeBuilder;
use cstree::green::GreenNode;
use cstree::interning::Interner;
use logos::Lexer;
use logos::Logos;

use crate::source_file_lexer::SourceFileToken;
use crate::syntax_kind::SyntaxKind;
use cstree::syntax::SyntaxNode;

pub struct SourceFileParser<'input> {
    lexer: Lexer<'input, SourceFileToken>,
    builder: GreenNodeBuilder<'static, 'static, SyntaxKind>,
    // the list of syntax errors we've accumulated so far
    errors: Vec<SyntaxError>,
}

impl<'input> SourceFileParser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: SourceFileToken::lexer(input),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.builder.start_node(SyntaxKind::SourceFile);
        self.consume_token();
        self.builder.finish_node();
    }

    fn consume_token(&mut self) {
        let token = self.lexer.next();
        println!("{:?}", token);
        match token {
            Some(Ok(token)) => {
                match token {
                    SourceFileToken::Comment => {
                        self.builder.token(SyntaxKind::Comment, self.lexer.slice());
                    }
                    SourceFileToken::Newline => {
                        self.builder.token(SyntaxKind::Newline, self.lexer.slice());
                    }
                    SourceFileToken::Expr => {
                        match parse_statement(&self.lexer.slice(), &mut self.builder) {
                            Ok(_) => {}
                            Err(err) => {
                                self.errors.push(err);
                            }
                        }
                    }
                };
                self.consume_token();
            }
            Some(Err(err)) => {
                panic!("lexer error: {:?}", err);
            }
            None => return,
        };
    }

    pub fn finish(self) -> (GreenNode, impl Interner) {
        // assert!(self.lexer.next().map(|t| t == Token::EoF).unwrap_or(true));
        let (tree, cache) = self.builder.finish();
        (tree, cache.unwrap().into_interner().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_file_parser() {
        let input = "select * from contact where id = '123';\n\n-- test comment\n\nselect wrong statement;\n\nselect id,username from contact\n\nselect id,name\nfrom contact -- test inline comment\nwhere id = '123';\n\n";

        let mut parser = SourceFileParser::new(&input);
        parser.parse();
        let (tree, interner) = parser.finish();

        let root = SyntaxNode::<SyntaxKind>::new_root_with_resolver(tree, interner);
        println!("{:#?}", input);
        // parser.errors.iter().for_each(|err| println!("{:#?}", err));
        dbg!(root);

        assert_eq!(1, 1);
    }
}
