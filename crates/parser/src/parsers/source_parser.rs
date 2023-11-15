use crate::Parser;

struct SourceParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> SourceParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(&mut self) {}
}

fn test() {
    let mut p_1 = Parser::new(vec![]);

    let mut parser_1 = SourceParser::new(&mut p_1);

    parser_1.parse();

    let parser_2 = SourceParser::new(&mut p_1);
}
