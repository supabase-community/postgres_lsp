use std::ops::{AddAssign, RangeBounds, SubAssign};

use line_index::LineIndex;
use parser::get_statements;
use text_size::{TextLen, TextSize};

use crate::{
    statement::{Statement, StatementParams},
    utils::apply_text_change,
    DocumentChange, DocumentChangesParams,
};

extern crate test;

#[derive(Debug)]
pub struct DocumentParams {
    pub text: String,
}

pub struct Document {
    pub text: String,
    pub version: i32,
    // vector of statements sorted by range.start()
    pub statements: Vec<Statement>,
    pub line_index: LineIndex,
}

impl Document {
    pub fn new(params: DocumentParams) -> Document {
        Document {
            version: 0,
            statements: get_statements(&params.text)
                .iter()
                .map(|(range, content)| {
                    Statement::new(StatementParams {
                        text: content.clone(),
                        range: Some(range.clone()),
                    })
                })
                .collect(),
            line_index: LineIndex::new(&params.text),
            text: params.text,
        }
    }

    pub fn diagnostics(&self) -> Vec<crate::diagnostics::Diagnostic> {
        self.statements
            .iter()
            .flat_map(|stmt| stmt.diagnostics.iter())
            .cloned()
            .collect()
    }

    pub fn apply_changes(&mut self, params: DocumentChangesParams) {
        // TODO: optimize this by searching for the last change without a range and start applying
        // from there

        for change in params.changes {
            self.apply_change(change);
        }
        self.version = params.version;
    }

    pub fn statement_at_offset(&self, offset: TextSize) -> Option<&Statement> {
        self.statements
            .iter()
            .find(|stmt| stmt.range.contains(offset.into()))
    }

    fn apply_change(&mut self, change: DocumentChange) {
        self.line_index = LineIndex::new(&self.text);

        if change.range.is_none() {
            self.text = change.text.clone();
            self.statements = get_statements(&self.text)
                .iter()
                .map(|(range, content)| {
                    Statement::new(StatementParams {
                        text: content.clone(),
                        range: Some(range.clone()),
                    })
                })
                .collect();
            return;
        }

        self.text = apply_text_change(&self.text, &change);

        if let Some(changed_stmt_pos) = self
            .statements
            .iter()
            .position(|stmt| stmt.range.contains_range(change.range.unwrap()))
        {
            self.statements
                .iter_mut()
                .enumerate()
                .skip_while(|(_, stmt)| change.range.unwrap().end() < stmt.range.start())
                .for_each(|(pos, stmt)| {
                    if pos == changed_stmt_pos {
                        stmt.apply_change(DocumentChange {
                            text: change.text.clone(),
                            // range must be relative to the start of the statement
                            range: change.range.unwrap().checked_sub(stmt.range.start()),
                        });
                    } else {
                        if change.is_addition() {
                            stmt.range.add_assign(change.diff_size());
                        } else if change.is_deletion() {
                            stmt.range.sub_assign(change.diff_size());
                        }
                    }
                });
        } else {
            // remove all statements that are affected by this change,
            let mut min = change.range.unwrap().start();
            let mut max = change.range.unwrap().end();

            let change_range = change.range.unwrap();

            for stmt in self.statements.extract_if(|stmt| {
                (change_range.start() < stmt.range.end())
                    && (change_range.end() > stmt.range.start())
            }) {
                if stmt.range.start() < min {
                    min = stmt.range.start();
                }
                if stmt.range.end() > max {
                    max = stmt.range.end();
                }
            }
            if self.text.text_len() < max {
                max = self.text.text_len();
            }

            // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
            let extracted_text = self
                .text
                .as_str()
                .get(usize::from(min)..usize::from(max))
                .unwrap();

            for (range, text) in get_statements(extracted_text) {
                let statement = Statement::new(StatementParams {
                    text,
                    range: Some(range),
                });

                match self
                    .statements
                    .binary_search_by(|s| s.range.start().cmp(&statement.range.start()))
                {
                    Ok(_) => {}
                    Err(pos) => {
                        self.statements.insert(pos, statement);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use text_size::{TextRange, TextSize};

    use crate::{Document, DocumentChange, DocumentChangesParams, DocumentParams};

    #[test]
    fn test_document_apply_changes() {
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            text: input.to_string(),
        });

        assert_eq!(d.statements.len(), 2);

        d.apply_changes(DocumentChangesParams {
            version: 1,
            changes: vec![DocumentChange {
                text: ",test from users\nselect 1;".to_string(),
                range: Some(TextRange::new(9.into(), 45.into())),
            }],
        });

        assert_eq!("select id,test from users\nselect 1;", d.text);
        assert_eq!(d.statements.len(), 2);
    }

    #[bench]
    fn bench_typing(b: &mut Bencher) {
        let input = "select id  from users;\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            text: input.to_string(),
        });

        let mut ctr: i32 = 1;

        b.iter(|| {
            d.apply_changes(DocumentChangesParams {
                version: ctr,
                changes: vec![DocumentChange {
                    text: "a".to_string(),
                    range: Some(TextRange::new(
                        u32::try_from(8 + ctr).unwrap().into(),
                        u32::try_from(8 + ctr).unwrap().into(),
                    )),
                }],
            });
            ctr = ctr + 1;
        });
    }

    #[test]
    fn test_document_apply_changes_at_end_of_statement() {
        let input = "select id from\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            text: input.to_string(),
        });

        assert_eq!(d.statements.len(), 2);

        let stmt_1_range = d.statements[0].range.clone();
        let stmt_2_range = d.statements[1].range.clone();

        let update_text = " contacts;";

        let update_range = TextRange::new(14.into(), 14.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        d.apply_changes(DocumentChangesParams {
            version: 1,
            changes: vec![DocumentChange {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        });

        assert_eq!("select id from contacts;\nselect * from contacts;", d.text);
        assert_eq!(d.statements.len(), 2);
        assert_eq!(d.statements[0].range.start(), stmt_1_range.start());
        assert_eq!(
            u32::from(d.statements[0].range.end()),
            u32::from(stmt_1_range.end()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].range.start()),
            u32::from(stmt_2_range.start()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].range.end()),
            u32::from(stmt_2_range.end()) + update_addition
        );

        assert_eq!(
            d.statements
                .iter()
                .find(|s| s.range.start() == TextSize::from(0))
                .unwrap()
                .version,
            1,
            "should touch the first statement"
        );
        assert_eq!(
            d.statements
                .iter()
                .find(|s| s.range.start() != TextSize::from(0))
                .unwrap()
                .version,
            0,
            "should not touch the second statement"
        );
    }

    #[test]
    fn test_document_apply_changes_within_statement() {
        let input = "select id  from users;\nselect * from contacts;";

        let mut d = Document::new(DocumentParams {
            text: input.to_string(),
        });

        assert_eq!(d.statements.len(), 2);

        let stmt_1_range = d.statements[0].range.clone();
        let stmt_2_range = d.statements[1].range.clone();

        let update_text = ",test";

        let update_range = TextRange::new(9.into(), 10.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        d.apply_changes(DocumentChangesParams {
            version: 1,
            changes: vec![DocumentChange {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        });

        assert_eq!(
            "select id,test from users;\nselect * from contacts;",
            d.text
        );
        assert_eq!(d.statements.len(), 2);
        assert_eq!(d.statements[0].range.start(), stmt_1_range.start());
        assert_eq!(
            u32::from(d.statements[0].range.end()),
            u32::from(stmt_1_range.end()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].range.start()),
            u32::from(stmt_2_range.start()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].range.end()),
            u32::from(stmt_2_range.end()) + update_addition
        );

        assert_eq!(
            d.statements
                .iter()
                .find(|s| s.range.start() == TextSize::from(0))
                .unwrap()
                .version,
            1,
            "should touch the first statement"
        );
        assert_eq!(
            d.statements
                .iter()
                .find(|s| s.range.start() != TextSize::from(0))
                .unwrap()
                .version,
            0,
            "should not touch the second statement"
        );
    }
}
