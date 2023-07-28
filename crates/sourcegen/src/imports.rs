use crate::builder::Builder;

#[derive(Debug, Clone)]
pub struct Import {
    path: String,
    items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Imports {
    imports: Vec<Import>,
}

impl Builder for Imports {
    fn finish(&mut self) -> String {
        let mut result = String::new();
        for import in &self.imports {
            result.push_str("use ");
            result.push_str(&import.path);
            if !import.items.is_empty() {
                result.push_str("::{");
                result.push_str(&import.items.join(", "));
                result.push_str("}");
            }
            result.push_str(";\n");
        }
        result
    }
}

/// A builder for a rust function
impl Imports {
    pub fn new() -> Self {
        Imports {
            imports: Vec::new(),
        }
    }

    pub fn with_import(&mut self, path: String, items: Vec<String>) -> &mut Self {
        self.imports.push(Import { path, items });
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{builder::Builder, imports::Imports};

    #[test]
    fn test_imports() {
        assert_eq!(
            Imports::new()
                .with_import("deeply::nested".to_string(), vec!["function".to_string()])
                .with_import(
                    "another::deeply::nested".to_string(),
                    vec!["function".to_string(), "another_item".to_string()]
                )
                .finish(),
            "use deeply::nested::{function};\nuse another::deeply::nested::{function, another_item};\n"
        )
    }
}
