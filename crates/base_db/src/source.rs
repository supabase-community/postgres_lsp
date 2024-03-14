use text_size::TextRange;

use crate::statement::Statement;

// manages files and hold text
//
#[derive(Debug)]
pub struct SourceParams {
    pub text: String,
}

pub struct Source {
    pub text: String,
    pub version: i32,
    pub statements: Vec<(TextRange, Statement)>,
}

impl Source {
    pub fn parse(params: SourceParams) -> Source {
        Source {
            text: params.text,
            version: 0,
            statements: vec![],
        }
    }
}
