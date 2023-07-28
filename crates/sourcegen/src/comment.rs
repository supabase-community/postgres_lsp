use textwrap::wrap;

use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct Comment {
    prefix: String,
    parts: Vec<String>,
}

impl Builder for Comment {
    fn finish(&mut self) -> String {
        wrap(
            self.parts.join('\n'.to_string().as_str()).as_str(),
            80 - self.prefix.len(),
        )
        .iter()
        .map(|line| format!("{} {}", self.prefix, line))
        .collect::<Vec<String>>()
        .join("\n")
    }
}

/// A builder for a rust function
impl Comment {
    pub fn new(prefix: String) -> Self {
        Comment {
            prefix,
            parts: Vec::new(),
        }
    }

    pub fn with_text(&mut self, text: String) -> &mut Self {
        self.parts.push(text);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, comment::Comment};

    #[test]
    fn test_comment() {
        assert_eq!(
            Comment::new("//".into())
                .with_text(
                    "one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three one two three"
                        .to_string()
                )
                .with_text("three two one".to_string())
                .finish(),
            "// one two three one two three one two three one two three one two three one two\n// three one two three one two three one two three one two three one two three\n// one two three one two three one two three one two three\n// three two one"
        )
    }
}
