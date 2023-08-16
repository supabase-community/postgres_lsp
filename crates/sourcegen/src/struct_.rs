use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct StructField {
    name: String,
    type_: String,
    public: bool,
}

/// A builder for a rust Struct
#[derive(Debug, Clone)]
pub struct Struct {
    name: String,
    fields: Vec<StructField>,
    public: bool,
    attributes: Vec<String>,
}

impl Builder for Struct {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        result.push_str(&self.attributes.join("\n"));
        if self.public {
            result.push_str("pub ");
        }
        result.push_str("struct ");
        result.push_str(&self.name);
        result.push_str(" {\n");
        result.push_str(
            &self
                .fields
                .iter()
                .map(|field| {
                    let mut result = String::new();
                    if field.public {
                        result.push_str("pub ");
                    } else {
                        result.push_str("    ");
                    }
                    result.push_str(&field.name);
                    result.push_str(": ");
                    result.push_str(&field.type_);
                    result.push_str(",\n");
                    result
                })
                .collect::<Vec<String>>()
                .join(""),
        );
        result.push_str("}\n");
        result
    }
}

impl Struct {
    pub fn new(name: String) -> Self {
        Struct {
            name,
            fields: Vec::new(),
            public: false,
            attributes: Vec::new(),
        }
    }

    pub fn public(&mut self) -> &mut Self {
        self.public = true;
        self
    }

    pub fn with_field(&mut self, name: String, type_: String, public: bool) -> &mut Self {
        self.fields.push(StructField {
            name,
            type_,
            public,
        });
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

    use crate::{attribute::Attribute, builder::Builder};

    #[test]
    fn test_struct() {
        assert_eq!(
            Attribute::new("derive".into())
                .with_argument("Debug".to_string(), None)
                .with_argument("Copy".to_string(), Some("value".to_string()))
                .finish(),
            "#[derive(Debug, Copy = value)]\n"
        )
    }
}
