use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct MatchArm {
    pattern: String,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Match {
    variable: String,
    arms: Vec<MatchArm>,
}

impl Builder for Match {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str("match ");
        result.push_str(&self.variable);
        result.push_str(" {\n");
        for arm in &self.arms {
            result.push_str("    ");
            result.push_str(&arm.pattern);
            result.push_str(" => ");
            result.push_str(&arm.body);
            result.push_str(",\n");
        }
        result.push_str("}\n");
        result
    }
}

/// A builder for a Rust match expression.
impl Match {
    pub fn new(variable: String) -> Self {
        Match {
            variable,
            arms: Vec::new(),
        }
    }

    pub fn with_arm(&mut self, pattern: String, body: String) -> &mut Self {
        self.arms.push(MatchArm { pattern, body });
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, match_::Match};

    #[test]
    fn test_match() {
        assert_eq!(
            Match::new("variable".into())
                .with_arm("one".into(), "result one".into())
                .with_arm("two".into(), "result two".into())
                .with_arm("_".into(), "result catch all".into())
                .finish(),
            "match variable {\n    one => result one,\n    two => result two,\n    _ => result catch all,\n}\n"
        )
    }
}
