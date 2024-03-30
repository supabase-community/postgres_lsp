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
            changed_statements.extend(doc.extract_statements());
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
                        *r =
                            TextRange::new(r.start(), r.start() + TextSize::from(self.diff_size()));
                    } else if self.is_addition() {
                        *r += self.diff_size();
                    } else if self.is_deletion() {
                        *r -= self.diff_size();
                    }
                });
        } else {
            println!("change across stmts");
            // change across stmts

            // get all changed ranges + their idx

            let mut min = self.range.unwrap().start();
            let mut max = self.range.unwrap().end();

            println!("stmt ranges {:#?}", doc.statement_ranges);
            println!("change range {:#?}", self.range.unwrap());

            for (idx, r) in doc
                .statement_ranges
                .iter()
                .enumerate()
                .skip_while(|(idx, r)| {
                    println!(
                        "skip {} {:#?} {}",
                        idx,
                        r,
                        self.range.unwrap().start() > r.end()
                    );
                    // skip until first changed stmt
                    self.range.unwrap().start() > r.end()
                })
                .take_while(|(idx, r)| {
                    println!(
                        "take {} {:#?} {}",
                        idx,
                        r,
                        self.range.unwrap().end() >= r.end()
                    );
                    // take until after last changed stmt
                    self.range.unwrap().end() >= r.end()
                })
            {
                println!(
                    "changes stmt at idx {} {}",
                    idx,
                    doc.text[r.clone()].to_string()
                );
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

            if doc.text.text_len() < max {
                max = doc.text.text_len();
            }

            // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
            let extracted_text = doc
                .text
                .as_str()
                .get(usize::from(min)..usize::from(max))
                .unwrap();

            println!("affected text '{}'", extracted_text);

            for (range, _) in get_statements(extracted_text) {
                match doc
                    .statement_ranges
                    .binary_search_by(|r| r.start().cmp(&range.start()))
                {
                    Ok(_) => {}
                    Err(pos) => {
                        println!("insert stmt at pos {} with range {:#?}", pos, range);
                        doc.statement_ranges.insert(pos, range);
                    }
                }
            }
        }

        doc.text = self.apply_to_text(&doc.text);

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
    use text_size::TextRange;

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
    }

    // #[bench]
    // fn bench_typing(b: &mut Bencher) {
    //     let input = "select id  from users;\nselect * from contacts;";
    //
    //     let mut d = Document::new(DocumentParams {
    //         text: input.to_string(),
    //     });
    //
    //     let mut ctr: i32 = 1;
    //
    //     b.iter(|| {
    //         d.apply_changes(DocumentChangesParams {
    //             version: ctr,
    //             changes: vec![DocumentChange {
    //                 text: "a".to_string(),
    //                 range: Some(TextRange::new(
    //                     u32::try_from(8 + ctr).unwrap().into(),
    //                     u32::try_from(8 + ctr).unwrap().into(),
    //                 )),
    //             }],
    //         });
    //         ctr = ctr + 1;
    //     });
    // }
    //
    // #[test]
    // fn test_document_apply_changes_at_end_of_statement() {
    //     let input = "select id from\nselect * from contacts;";
    //
    //     let mut d = Document::new(DocumentParams {
    //         text: input.to_string(),
    //     });
    //
    //     assert_eq!(d.statements.len(), 2);
    //
    //     let stmt_1_range = d.statements[0].range.clone();
    //     let stmt_2_range = d.statements[1].range.clone();
    //
    //     let update_text = " contacts;";
    //
    //     let update_range = TextRange::new(14.into(), 14.into());
    //
    //     let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
    //     let update_addition = update_text_len - u32::from(update_range.len());
    //
    //     d.apply_changes(DocumentChangesParams {
    //         version: 1,
    //         changes: vec![DocumentChange {
    //             text: update_text.to_string(),
    //             range: Some(update_range),
    //         }],
    //     });
    //
    //     assert_eq!("select id from contacts;\nselect * from contacts;", d.text);
    //     assert_eq!(d.statements.len(), 2);
    //     assert_eq!(d.statements[0].range.start(), stmt_1_range.start());
    //     assert_eq!(
    //         u32::from(d.statements[0].range.end()),
    //         u32::from(stmt_1_range.end()) + update_addition
    //     );
    //     assert_eq!(
    //         u32::from(d.statements[1].range.start()),
    //         u32::from(stmt_2_range.start()) + update_addition
    //     );
    //     assert_eq!(
    //         u32::from(d.statements[1].range.end()),
    //         u32::from(stmt_2_range.end()) + update_addition
    //     );
    //
    //     assert_eq!(
    //         d.statements
    //             .iter()
    //             .find(|s| s.range.start() == TextSize::from(0))
    //             .unwrap()
    //             .version,
    //         1,
    //         "should touch the first statement"
    //     );
    //     assert_eq!(
    //         d.statements
    //             .iter()
    //             .find(|s| s.range.start() != TextSize::from(0))
    //             .unwrap()
    //             .version,
    //         0,
    //         "should not touch the second statement"
    //     );
    // }
    //
    // #[test]
    // fn test_document_apply_changes_within_statement() {
    //     let input = "select id  from users;\nselect * from contacts;";
    //
    //     let mut d = Document::new(DocumentParams {
    //         text: input.to_string(),
    //     });
    //
    //     assert_eq!(d.statements.len(), 2);
    //
    //     let stmt_1_range = d.statements[0].range.clone();
    //     let stmt_2_range = d.statements[1].range.clone();
    //
    //     let update_text = ",test";
    //
    //     let update_range = TextRange::new(9.into(), 10.into());
    //
    //     let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
    //     let update_addition = update_text_len - u32::from(update_range.len());
    //
    //     d.apply_changes(DocumentChangesParams {
    //         version: 1,
    //         changes: vec![DocumentChange {
    //             text: update_text.to_string(),
    //             range: Some(update_range),
    //         }],
    //     });
    //
    //     assert_eq!(
    //         "select id,test from users;\nselect * from contacts;",
    //         d.text
    //     );
    //     assert_eq!(d.statements.len(), 2);
    //     assert_eq!(d.statements[0].range.start(), stmt_1_range.start());
    //     assert_eq!(
    //         u32::from(d.statements[0].range.end()),
    //         u32::from(stmt_1_range.end()) + update_addition
    //     );
    //     assert_eq!(
    //         u32::from(d.statements[1].range.start()),
    //         u32::from(stmt_2_range.start()) + update_addition
    //     );
    //     assert_eq!(
    //         u32::from(d.statements[1].range.end()),
    //         u32::from(stmt_2_range.end()) + update_addition
    //     );
    //
    //     assert_eq!(
    //         d.statements
    //             .iter()
    //             .find(|s| s.range.start() == TextSize::from(0))
    //             .unwrap()
    //             .version,
    //         1,
    //         "should touch the first statement"
    //     );
    //     assert_eq!(
    //         d.statements
    //             .iter()
    //             .find(|s| s.range.start() != TextSize::from(0))
    //             .unwrap()
    //             .version,
    //         0,
    //         "should not touch the second statement"
    //     );
    // }
}
