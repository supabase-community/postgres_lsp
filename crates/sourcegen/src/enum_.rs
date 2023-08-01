use crate::{builder::Builder, Comment};

#[derive(Debug, Clone)]
pub struct EnumValue {
    name: String,
    value: Option<String>,
}

/// Generate a rust enum
#[derive(Debug, Clone)]
pub struct Enum {
    name: String,
    comments: Comment,
    public: bool,
    values: Vec<EnumValue>,
    attributes: Vec<String>,
}

impl Builder for Enum {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str(self.comments.finish().as_str());
        result.push_str("\n");
        for attribute in &self.attributes {
            result.push_str(&attribute);
        }
        if self.public {
            result.push_str("pub ");
        }
        result.push_str("enum ");
        result.push_str(&self.name);
        result.push_str(" {\n");
        for value in &self.values {
            result.push_str("    ");
            result.push_str(&value.name);
            if let Some(value) = &value.value {
                result.push_str(" = ");
                result.push_str(&value);
            }
            result.push_str(",\n");
        }
        result.push_str("}\n");
        result
    }
}

impl Enum {
    pub fn new(name: String) -> Self {
        Enum {
            name,
            comments: Comment::new("///".to_string()),
            public: false,
            values: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn public(&mut self) -> &mut Self {
        self.public = true;
        self
    }

    pub fn with_comment(&mut self, comment: String) -> &mut Self {
        self.comments.with_text(comment);
        self
    }

    pub fn with_value(&mut self, name: String, value: Option<String>) -> &mut Self {
        if self.values.iter().find(|v| v.name == name).is_some() {
            return self;
        }
        self.values.push(EnumValue { name, value });
        self
    }

    pub fn with_attribute(&mut self, attribute: String) -> &mut Self {
        self.attributes.push(attribute);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, enum_::Enum};

    #[test]
    fn test_enum() {
        assert_eq!(
            Enum::new("my_enum".into())
                .public()
                .with_value("A".into(), None)
                .with_attribute("#[derive(Copy)]\n".to_string())
                .with_value("".into(), Some("5".into()))
                .finish(),
            "#[derive(Copy)]\npub enum my_enum {\n    A,\n    B = 5,\n}\n"
        )
    }
}
