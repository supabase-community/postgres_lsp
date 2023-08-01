use crate::{builder::Builder, Comment};
use rustfmt_wrapper::rustfmt;

#[derive(Debug, Clone)]
pub struct SourceFile {
    content: String,
    comments: Comment,
}

impl Builder for SourceFile {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str(self.comments.finish().as_str());
        result.push_str("\n");
        result.push_str(&self.content);
        match rustfmt(&result) {
            Ok(formatted) => formatted,
            Err(e) => {
                println!("rustfmt error: {:?}", e);
                result
            }
        }
    }
}

/// Generate a rust source file
impl SourceFile {
    pub fn new() -> Self {
        SourceFile {
            content: "".to_string(),
            comments: Comment::new("//!".to_string()),
        }
    }

    pub fn add_comment(&mut self, comment: String) -> &mut SourceFile {
        self.comments.with_text(comment);
        self
    }

    pub fn add_block(&mut self, block: String) -> &mut SourceFile {
        self.content.push_str(block.as_str());
        self.content.push_str("\n");
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, source_file::SourceFile};

    #[test]
    fn test_source_file() {
        assert_eq!(
            SourceFile::new()
                .add_block("block 1".into())
                .add_block("block 2".into())
                .finish(),
            "block 1\nblock 2\n"
        )
    }
}
