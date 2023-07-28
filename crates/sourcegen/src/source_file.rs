use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct SourceFile {
    content: String,
    comments: Vec<String>,
}

impl Builder for SourceFile {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str("//! ");
            result.push_str(&comment);
            result.push_str("\n");
        }
        result.push_str("\n");
        result.push_str(&self.content);
        result
    }
}

/// Generate a rust source file
impl SourceFile {
    pub fn new() -> Self {
        SourceFile {
            content: "".to_string(),
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, comment: String) -> &mut SourceFile {
        self.comments.push(comment);
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
