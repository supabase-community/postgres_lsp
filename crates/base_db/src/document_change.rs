use parser::get_statements;
use text_size::{TextLen, TextRange, TextSize};

use crate::document::{Document, StatementRef};

#[derive(Debug)]
pub struct DocumentChange {
    pub version: i32,
    pub changes: Vec<Change>,
}

#[derive(Debug)]
pub struct Change {
    /// The range of the file that changed. If `None`, the whole file changed.
    pub range: Option<TextRange>,
    pub text: String,
}

impl Change {
    pub fn diff_size(&self) -> TextSize {
        match self.range {
            Some(range) => {
                let range_length: usize = range.len().into();
                let text_length = self.text.chars().count();
                let diff = (text_length as i64 - range_length as i64).abs();
                TextSize::from(u32::try_from(diff).unwrap())
            }
            None => TextSize::from(u32::try_from(self.text.chars().count()).unwrap()),
        }
    }

    pub fn is_addition(&self) -> bool {
        self.range.is_some() && self.text.len() > self.range.unwrap().len().into()
    }

    pub fn is_deletion(&self) -> bool {
        self.range.is_some() && self.text.len() < self.range.unwrap().len().into()
    }

    pub fn apply_to_text(&self, text: &String) -> String {
        if self.range.is_none() {
            return self.text.clone();
        }

        let range = self.range.unwrap();
        let start = usize::from(range.start());
        let end = usize::from(range.end());

        let mut new_text = String::new();
        new_text.push_str(&text[..start]);
        new_text.push_str(&self.text);
        new_text.push_str(&text[end..]);

        new_text
    }

    pub fn apply(&self, doc: &mut Document) -> Vec<StatementRef> {
        let mut changed_statements: Vec<StatementRef> = Vec::new();

        if self.range.is_none() {
            // whole file changed
            changed_statements.extend(doc.drain_statements());
            doc.statement_ranges = get_statements(&self.text)
                .iter()
                .map(|(range, _)| range.clone())
                .collect();
            doc.text = self.text.clone();
        } else if let Some(changed_stmt_pos) = doc
            .statement_ranges
            .iter()
            .position(|r| r.contains_range(self.range.unwrap()))
        {
            // change within a single statement
            doc.statement_ranges
                .iter_mut()
                .enumerate()
                .skip_while(|(_, r)| self.range.unwrap().end() < r.start())
                .for_each(|(pos, r)| {
                    if pos == changed_stmt_pos {
                        // only this ones ref is different, the rest do not have any text
                        // changes
                        changed_statements.push(StatementRef {
                            idx: pos,
                            text: doc.text[r.clone()].to_string(),
                            document_url: doc.url.clone(),
                        });

                        // if addition, expand the range
                        // if deletion, shrink the range
                        *r = TextRange::new(r.start(), r.end() + TextSize::from(self.diff_size()));
                    } else if self.is_addition() {
                        *r += self.diff_size();
                    } else if self.is_deletion() {
                        *r -= self.diff_size();
                    }
                });

            doc.text = self.apply_to_text(&doc.text);
        } else {
            // change across stmts

            let mut min = self.range.unwrap().start();
            let mut max = self.range.unwrap().end();

            for (idx, r) in doc
                .statement_ranges
                .iter()
                .enumerate()
                .skip_while(|(_, r)| {
                    // skip until first changed stmt
                    self.range.unwrap().start() > r.end()
                })
                .take_while(|(_, r)| {
                    // take until after last changed stmt
                    self.range.unwrap().end() >= r.end()
                })
            {
                changed_statements.push(StatementRef {
                    idx,
                    text: doc.text[r.clone()].to_string(),
                    document_url: doc.url.clone(),
                });

                if r.start() < min {
                    min = r.start();
                }
                if r.end() > max {
                    max = r.end();
                }
            }

            doc.text = self.apply_to_text(&doc.text);

            if doc.text.text_len() < max {
                max = doc.text.text_len();
            }

            // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
            let extracted_text = doc
                .text
                .as_str()
                .get(usize::from(min)..usize::from(max))
                .unwrap();

            doc.statement_ranges.drain(
                changed_statements.iter().min_by_key(|s| s.idx).unwrap().idx
                    ..changed_statements.iter().max_by_key(|s| s.idx).unwrap().idx + 1,
            );

            for (range, _) in get_statements(extracted_text) {
                match doc
                    .statement_ranges
                    .binary_search_by(|r| r.start().cmp(&range.start()))
                {
                    Ok(_) => {}
                    Err(pos) => {
                        doc.statement_ranges.insert(pos, range);
                    }
                }
            }
        }

        changed_statements
    }
}

impl DocumentChange {
    fn new(version: i32, changes: Vec<Change>) -> DocumentChange {
        DocumentChange { version, changes }
    }

    pub fn apply(&self, doc: &mut Document) -> Vec<StatementRef> {
        // TODO: optimize this by searching for the last change without a range and start applying
        // from there
        let changed_statements = self.changes.iter().flat_map(|c| c.apply(doc)).collect();

        doc.version = self.version;

        changed_statements
    }
}

#[cfg(test)]
mod tests {
    use parser::get_statements;
    use text_size::{TextRange, TextSize};

    use crate::{
        document::StatementRef, document_change::Change, Document, DocumentChange, DocumentParams,
        PgLspPath,
    };

    #[test]
    fn test_document_apply_changes() {
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            url: PgLspPath::new("test.sql"),
            text: input.to_string(),
        });

        assert_eq!(d.statement_ranges.len(), 2);

        let change = DocumentChange::new(
            1,
            vec![Change {
                text: ",test from users\nselect 1;".to_string(),
                range: Some(TextRange::new(9.into(), 45.into())),
            }],
        );

        let changed = change.apply(&mut d);

        assert_eq!(changed.len(), 2);
        assert_eq!(
            changed[0],
            StatementRef {
                document_url: PgLspPath::new("test.sql"),
                text: "select id from users;".to_string(),
                idx: 0
            }
        );
        assert_eq!(
            changed[1],
            StatementRef {
                document_url: PgLspPath::new("test.sql"),
                text: "select * from contacts;".to_string(),
                idx: 1
            }
        );

        assert_eq!("select id,test from users\nselect 1;", d.text);
        assert_eq!(d.statement_ranges.len(), 2);

        for (r, _) in &get_statements(&d.text) {
            assert_eq!(
                d.statement_ranges.iter().position(|x| r == x).is_some(),
                true,
                "should have stmt with range {:#?}",
                r
            );
        }

        assert_eq!(d.statement_ranges[0], TextRange::new(0.into(), 26.into()));
        assert_eq!(d.statement_ranges[1], TextRange::new(26.into(), 35.into()));
    }

    #[test]
    fn test_document_apply_changes_at_end_of_statement() {
        let input = "select id from\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            url: PgLspPath::new("test.sql"),
            text: input.to_string(),
        });

        assert_eq!(d.statement_ranges.len(), 2);

        let stmt_1_range = d.statement_ranges[0].clone();
        let stmt_2_range = d.statement_ranges[1].clone();

        let update_text = " contacts;";

        let update_range = TextRange::new(14.into(), 14.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let change = DocumentChange::new(
            1,
            vec![Change {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        );

        change.apply(&mut d);

        assert_eq!("select id from contacts;\nselect * from contacts;", d.text);
        assert_eq!(d.statement_ranges.len(), 2);
        assert_eq!(d.statement_ranges[0].start(), stmt_1_range.start());
        assert_eq!(
            u32::from(d.statement_ranges[0].end()),
            u32::from(stmt_1_range.end()) + update_addition
        );
        assert_eq!(
            u32::from(d.statement_ranges[1].start()),
            u32::from(stmt_2_range.start()) + update_addition
        );
        assert_eq!(
            u32::from(d.statement_ranges[1].end()),
            u32::from(stmt_2_range.end()) + update_addition
        );
    }

    #[test]
    fn test_document_apply_changes_within_statement() {
        let input = "select id  from users;\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            url: PgLspPath::new("test.sql"),
            text: input.to_string(),
        });

        assert_eq!(d.statement_ranges.len(), 2);

        let stmt_1_range = d.statement_ranges[0].clone();
        let stmt_2_range = d.statement_ranges[1].clone();

        let update_text = ",test";

        let update_range = TextRange::new(9.into(), 10.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let change = DocumentChange::new(
            1,
            vec![Change {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        );

        change.apply(&mut d);

        assert_eq!(
            "select id,test from users;\nselect * from contacts;",
            d.text
        );
        assert_eq!(d.statement_ranges.len(), 2);
        assert_eq!(d.statement_ranges[0].start(), stmt_1_range.start());
        assert_eq!(
            u32::from(d.statement_ranges[0].end()),
            u32::from(stmt_1_range.end()) + update_addition
        );
        assert_eq!(
            u32::from(d.statement_ranges[1].start()),
            u32::from(stmt_2_range.start()) + update_addition
        );
        assert_eq!(
            u32::from(d.statement_ranges[1].end()),
            u32::from(stmt_2_range.end()) + update_addition
        );
    }
}
