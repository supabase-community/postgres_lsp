use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct Function {
    body: String,
    name: String,
    public: bool,
    return_type: Option<String>,
    parameters: Vec<String>,
    comments: Vec<String>,
}

impl Builder for Function {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str("/// ");
            result.push_str(&comment);
            result.push_str("\n");
        }
        if self.public {
            result.push_str("pub ");
        }
        result.push_str("fn ");
        result.push_str(&self.name);
        result.push_str("(");
        result.push_str(&self.parameters.join(", "));
        result.push_str(")");
        if let Some(return_type) = &self.return_type {
            result.push_str(" -> ");
            result.push_str(&return_type);
        }
        result.push_str("{\n\t");
        result.push_str(&self.body);
        result.push_str("\n}\n");
        result
    }
}

/// A builder for a rust function
impl Function {
    pub fn new(name: String) -> Self {
        Function {
            body: "".into(),
            public: false,
            name,
            return_type: None,
            parameters: Vec::new(),
            comments: Vec::new(),
        }
    }

    pub fn public(&mut self) -> &mut Self {
        self.public = true;
        self
    }

    pub fn with_return_type(&mut self, return_type: String) -> &mut Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn with_parameter(&mut self, name: String, type_: Option<String>) -> &mut Self {
        let mut parameter = name;
        if let Some(type_) = type_ {
            parameter.push_str(": ");
            parameter.push_str(&type_);
        }
        self.parameters.push(parameter);
        self
    }

    pub fn with_comment(&mut self, comment: String) -> &mut Self {
        self.comments.push(comment);
        self
    }

    pub fn with_body(&mut self, body: String) -> &mut Self {
        self.body = body;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, function::Function};

    #[test]
    fn test_function() {
        assert_eq!(
            Function::new("my_function".into())
                .public()
                .with_return_type("String".to_string())
                .with_body("println!(\"Hello, world!\");".into())
                .finish(),
            "pub fn my_function() -> String{\nprintln!(\"Hello, world!\");\n}\n"
        )
    }
}
