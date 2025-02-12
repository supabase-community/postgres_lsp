use std::{hash::Hash, hash::Hasher, ops::RangeBounds};

use line_index::LineIndex;
use text_size::{TextRange, TextSize};

use pglt_fs::PgLspPath;

extern crate test;

/// Represents a sql source file, and contains a list of statements represented by their ranges
pub struct Document {
    /// The url of the document
    pub url: PgLspPath,
    /// The text of the document
    pub text: String,
    /// The version of the document
    pub version: i32,
    /// List of statements sorted by range.start()
    pub statement_ranges: Vec<TextRange>,
    /// Line index for the document
    pub line_index: LineIndex,
}

impl Hash for Document {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

/// Represents a reference to a sql statement. This is the primary data structure that is used by higher-level crates to save and retrieve information about a statement.
/// This needs to be optimised by removing the text from the struct and making it a reference to the text in the document.
///
/// Note that the ref must include all information needed to uniquely identify the statement, so that it can be used as a key in a hashmap.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct StatementRef {
    pub document_url: PgLspPath,
    // TODO use string interner for text
    pub text: String,
    pub idx: usize,
}

impl Document {
    /// Create a new document
    pub fn new(url: PgLspPath, text: Option<String>) -> Document {
        Document {
            version: 0,
            line_index: LineIndex::new(text.as_ref().unwrap_or(&"".to_string())),
            // TODO: use errors returned by split
            statement_ranges: text.as_ref().map_or_else(Vec::new, |f| {
                pglt_statement_splitter::split(f).ranges.to_vec()
            }),
            text: text.unwrap_or("".to_string()),
            url,
        }
    }

    /// Returns the statement at the given offset
    pub fn statement_at_offset(&self, offset: &TextSize) -> Option<StatementRef> {
        self.statement_ranges
            .iter()
            .position(|r| r.contains(offset))
            .map(|idx| self.statement_ref(idx))
    }

    /// Returns the statements at the given range
    pub fn statements_at_range(&self, range: &TextRange) -> Vec<StatementRef> {
        self.statement_ranges
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                range.contains_range(r.to_owned().to_owned()) || r.contains_range(range.to_owned())
            })
            .map(|(idx, _)| self.statement_ref(idx))
            .collect()
    }

    /// Returns the statement at the given offset with its range in the document
    pub fn statement_at_offset_with_range(
        &self,
        offset: &TextSize,
    ) -> Option<(TextRange, StatementRef)> {
        self.statement_ranges
            .iter()
            .position(|r| r.contains(offset))
            .map(|idx| self.statement_ref_with_range(idx))
    }

    /// Drains the statements from the document
    pub(crate) fn drain_statements(&mut self) -> Vec<StatementRef> {
        self.statement_ranges
            .drain(..)
            .enumerate()
            .map(|(idx, range)| StatementRef {
                document_url: self.url.clone(),
                text: self.text[range].to_string(),
                idx,
            })
            .collect()
    }

    /// Returns all statements with their ranges in the document
    pub fn statement_refs_with_range(&self) -> Vec<(TextRange, StatementRef)> {
        self.statement_ranges
            .iter()
            .enumerate()
            .map(|(idx, range)| {
                (
                    *range,
                    StatementRef {
                        document_url: self.url.clone(),
                        text: self.text[*range].to_string(),
                        idx,
                    },
                )
            })
            .collect()
    }

    /// Returns all statements in the document
    pub fn statement_refs(&self) -> Vec<StatementRef> {
        self.statement_ranges
            .iter()
            .enumerate()
            .map(|(idx, range)| StatementRef {
                document_url: self.url.clone(),
                text: self.text[*range].to_string(),
                idx,
            })
            .collect()
    }

    /// Returns the statement with the given index, throws an error if the index is out of bounds
    pub fn statement_ref(&self, pos: usize) -> StatementRef {
        self.statement_ranges
            .get(pos)
            .map(|range| StatementRef {
                document_url: self.url.clone(),
                text: self.text[*range].to_string(),
                idx: pos,
            })
            .unwrap()
    }

    /// Returns the statement with the given index and its range in the document
    pub fn statement_ref_with_range(&self, pos: usize) -> (TextRange, StatementRef) {
        self.statement_ranges
            .get(pos)
            .map(|range| {
                (
                    *range,
                    StatementRef {
                        document_url: self.url.clone(),
                        text: self.text[*range].to_string(),
                        idx: pos,
                    },
                )
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use pglt_fs::PgLspPath;
    use text_size::{TextRange, TextSize};

    use crate::Document;

    #[test]
    fn test_statements_at_range() {
        let url = PgLspPath::new("test.sql");

        let doc = Document::new(
            url,
            Some("select unknown from contact;\n\nselect 12345;\n\nalter table test drop column id;\n"
                .to_string()),
        );

        let x = doc.statements_at_range(&TextRange::new(TextSize::from(2), TextSize::from(5)));

        assert_eq!(x.len(), 1);

        assert_eq!(x[0].text, "select unknown from contact;");
    }
}
