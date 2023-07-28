use crate::builder::Builder;

/// Generate a rust implementation
#[derive(Debug, Clone)]
pub struct Implementation {
    for_: String,
    blocks: Vec<String>,
}

impl Builder for Implementation {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str("impl ");
        result.push_str(&self.for_);
        result.push_str(" {\n");
        for block in &self.blocks {
            result.push_str(&block);
            result.push_str("\n");
        }
        result.push_str("}\n");
        result
    }
}

impl Implementation {
    pub fn new(for_: String) -> Self {
        Implementation {
            for_,
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: String) -> &mut Self {
        self.blocks.push(block);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, implementation::Implementation};

    #[test]
    fn test_implementation() {
        assert_eq!(
            Implementation::new("my_enum".into())
                .add_block("test".to_string())
                .finish(),
            "impl my_enum {\ntest}\n"
        )
    }
}
