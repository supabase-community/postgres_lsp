use std::{hash::Hash, hash::Hasher, ops::RangeBounds, usize};

use line_index::LineIndex;
use text_size::{TextRange, TextSize};

use crate::PgLspPath;

extern crate test;

#[derive(Debug)]
pub struct DocumentParams {
    pub url: PgLspPath,
    pub text: String,
}

pub struct Document {
    pub url: PgLspPath,
    pub text: String,
    pub version: i32,
    // vector of statements sorted by range.start()
    pub statement_ranges: Vec<TextRange>,
    pub line_index: LineIndex,
}

impl Hash for Document {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct StatementRef {
    pub document_url: PgLspPath,
    // TODO use string interner for text
    pub text: String,
    pub idx: usize,
}

impl Document {
    pub fn new_empty(url: PgLspPath) -> Document {
        Document {
            version: 0,
            statement_ranges: Vec::new(),
            line_index: LineIndex::new(""),
            text: "".to_string(),
            url,
        }
    }

    pub fn new(params: DocumentParams) -> Document {
        Document {
            version: 0,
            // todo: use errors returned by split
            statement_ranges: pg_statement_splitter::split(&params.text)
                .ranges
                .iter()
                .map(|range| range.clone())
                .collect(),
            line_index: LineIndex::new(&params.text),
            text: params.text,
            url: params.url,
        }
    }

    pub fn statement_at_offset(&self, offset: &TextSize) -> Option<StatementRef> {
        self.statement_ranges
            .iter()
            .position(|r| r.contains(offset))
            .map(|idx| self.statement_ref(idx))
    }

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

    pub fn statement_at_offset_with_range(
        &self,
        offset: &TextSize,
    ) -> Option<(TextRange, StatementRef)> {
        self.statement_ranges
            .iter()
            .position(|r| r.contains(offset))
            .map(|idx| self.statement_ref_with_range(idx))
    }

    pub fn drain_statements(&mut self) -> Vec<StatementRef> {
        self.statement_ranges
            .drain(..)
            .enumerate()
            .map(|(idx, range)| StatementRef {
                document_url: self.url.clone(),
                text: self.text[range.clone()].to_string(),
                idx,
            })
            .collect()
    }

    pub fn statement_refs_with_range(&self) -> Vec<(TextRange, StatementRef)> {
        self.statement_ranges
            .iter()
            .enumerate()
            .map(|(idx, range)| {
                (
                    range.clone(),
                    StatementRef {
                        document_url: self.url.clone(),
                        text: self.text[range.clone()].to_string(),
                        idx,
                    },
                )
            })
            .collect()
    }

    pub fn statement_refs(&self) -> Vec<StatementRef> {
        self.statement_ranges
            .iter()
            .enumerate()
            .map(|(idx, range)| StatementRef {
                document_url: self.url.clone(),
                text: self.text[range.clone()].to_string(),
                idx,
            })
            .collect()
    }

    pub fn statement_ref(&self, pos: usize) -> StatementRef {
        self.statement_ranges
            .get(pos)
            .map(|range| StatementRef {
                document_url: self.url.clone(),
                text: self.text[range.clone()].to_string(),
                idx: pos,
            })
            .unwrap()
    }

    pub fn statement_ref_with_range(&self, pos: usize) -> (TextRange, StatementRef) {
        self.statement_ranges
            .get(pos)
            .map(|range| {
                (
                    range.clone(),
                    StatementRef {
                        document_url: self.url.clone(),
                        text: self.text[range.clone()].to_string(),
                        idx: pos,
                    },
                )
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use text_size::{TextRange, TextSize};

    use crate::{Document, DocumentParams, PgLspPath};

    #[test]
    fn test_statements_at_range() {
        let url = PgLspPath::new("test.sql");

        let doc = Document::new(DocumentParams {
            url,
            text: "select unknown from contact;\n\nselect 12345;\n\nalter table test drop column id;\n".to_string()
        });

        let x = doc.statements_at_range(&TextRange::new(TextSize::from(2), TextSize::from(5)));

        assert_eq!(x.len(), 1);

        assert_eq!(x[0].text, "select unknown from contact;");
    }
}
