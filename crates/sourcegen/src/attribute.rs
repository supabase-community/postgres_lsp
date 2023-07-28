use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct AttributeArgument {
    name: String,
    value: Option<String>,
}

/// A builder for a rust Attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    name: String,
    arguments: Vec<AttributeArgument>,
}

impl Builder for Attribute {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str("#[");
        result.push_str(&self.name);
        if !self.arguments.is_empty() {
            result.push_str("(");
            result.push_str(
                &self
                    .arguments
                    .iter()
                    .map(|arg| {
                        let mut result = String::new();
                        result.push_str(&arg.name);
                        if let Some(value) = &arg.value {
                            result.push_str(" = ");
                            result.push_str(&value);
                        }
                        result
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            result.push_str(")");
        }
        result.push_str("]\n");
        result
    }
}

impl Attribute {
    pub fn new(name: String) -> Self {
        Attribute {
            name,
            arguments: Vec::new(),
        }
    }

    pub fn with_argument(&mut self, name: String, value: Option<String>) -> &mut Self {
        self.arguments.push(AttributeArgument { name, value });
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{attribute::Attribute, builder::Builder};

    #[test]
    fn test_attribute() {
        assert_eq!(
            Attribute::new("derive".into())
                .with_argument("Debug".to_string(), None)
                .with_argument("Copy".to_string(), Some("value".to_string()))
                .finish(),
            "#[derive(Debug, Copy = value)]\n"
        )
    }
}
