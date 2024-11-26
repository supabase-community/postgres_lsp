use std::ops::Sub;
use text_size::{TextLen, TextRange, TextSize};

use crate::workspace::{ChangeFileParams, ChangeParams};

use super::{store::Statement, Document, StatementRef};

#[derive(Debug)]
pub enum StatementChange {
    Added(Statement),
    Deleted(StatementRef),
    Modified(ChangedStatement),
}

#[derive(Debug)]
pub struct ChangedStatement {
    pub old: Statement,
    pub new_ref: StatementRef,

    pub range: TextRange,
    pub text: String,
}

fn apply_text_change(text: &str, range: Option<TextRange>, change_text: &str) -> String {
    if range.is_none() {
        return change_text.to_string();
    }

    let range = range.unwrap();
    let start = usize::from(range.start());
    let end = usize::from(range.end());

    let mut new_text = String::new();
    new_text.push_str(&text[..start]);
    new_text.push_str(change_text);
    new_text.push_str(&text[end..]);

    new_text
}

impl ChangeParams {
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

    pub fn apply_to_text(&self, text: &str) -> String {
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

    fn apply_full_change(&self, doc: &mut Document) -> Vec<StatementChange> {
        let mut changed: Vec<StatementChange> = vec![];

        changed.extend(
            doc.statements
                .drain(..)
                .map(|(id, _)| {
                    StatementChange::Deleted(StatementRef {
                        id,
                        path: doc.path.clone(),
                    })
                })
                .collect::<Vec<StatementChange>>(),
        );

        doc.content = self.text.clone();

        doc.statements = pg_statement_splitter::split(&doc.content)
            .ranges
            .iter()
            .map(|r| (doc.id_generator.next(), *r))
            .collect();

        doc.statements.iter().for_each(|(id, range)| {
            changed.push(StatementChange::Added(Statement {
                ref_: StatementRef {
                    path: doc.path.clone(),
                    id: *id,
                },
                text: doc.content[*range].to_string(),
            }));
        });

        changed
    }

    fn apply_single_statement_change(
        &self,
        doc: &mut Document,
        changed_stmt_pos: usize,
    ) -> Vec<StatementChange> {
        let mut changed: Vec<StatementChange> = vec![];

        // save the old statement
        let old = doc.statement_at(changed_stmt_pos);
        let old_range = doc.statements[changed_stmt_pos].1;

        // first mutate the target statement
        let new_range = if self.is_addition() {
            Some(TextRange::new(
                doc.statements[changed_stmt_pos].1.start(),
                doc.statements[changed_stmt_pos].1.end() + self.diff_size(),
            ))
        } else if self.is_deletion() {
            Some(TextRange::new(
                doc.statements[changed_stmt_pos].1.start(),
                doc.statements[changed_stmt_pos].1.end() - self.diff_size(),
            ))
        } else {
            None
        };
        if let Some(new_range) = new_range {
            doc.mut_statement_at(changed_stmt_pos, new_range);
        }
        changed.push(StatementChange::Modified(ChangedStatement {
            old,
            new_ref: doc.statement_ref_at(changed_stmt_pos),
            // change must be relative to statement
            range: self.range.unwrap().sub(old_range.start()),
            text: self.text.clone(),
        }));

        // then move the rest of the statements accordingly
        doc.statements
            .iter_mut()
            .skip(changed_stmt_pos + 1)
            .for_each(|(_, range)| {
                if self.is_addition() {
                    *range += self.diff_size();
                } else if self.is_deletion() {
                    *range -= self.diff_size();
                }
            });

        doc.content = self.apply_to_text(&doc.content);

        changed
    }

    fn apply_change_across_statements(&self, doc: &mut Document) -> Vec<StatementChange> {
        let mut changed: Vec<StatementChange> = vec![];

        let mut min = self.range.unwrap().start();
        let mut max = self.range.unwrap().end();
        let mut from_idx = 0;
        let mut to_idx = doc.statements.len() - 1;

        for (idx, inner_ref) in doc
            .statements
            .iter()
            .enumerate()
            .skip_while(|(_, (_, r))| {
                // skip until first changed stmt
                self.range.unwrap().start() > r.end()
            })
            .take_while(|(_, (_, r))| {
                // take until after last changed stmt
                self.range.unwrap().end() >= r.end()
            })
        {
            changed.push(StatementChange::Deleted(doc.statement_ref(inner_ref)));

            if inner_ref.1.start() < min {
                min = inner_ref.1.start();
                from_idx = idx;
            }
            if inner_ref.1.end() > max {
                max = inner_ref.1.end();
                to_idx = idx;
            }
        }

        doc.content = self.apply_to_text(&doc.content);

        if doc.content.text_len() < max {
            max = doc.content.text_len();
        }

        // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
        let extracted_text = doc
            .content
            .as_str()
            .get(usize::from(min)..usize::from(max))
            .unwrap();

        doc.statements.drain(
            from_idx..(to_idx + 1)
        );

        for range in pg_statement_splitter::split(extracted_text).ranges {
            match doc
                .statements
                .binary_search_by(|(_, r)| r.start().cmp(&range.start()))
            {
                Ok(_) => {}
                Err(pos) => {
                    doc.insert_statement_at(pos, range);
                    changed.push(StatementChange::Added(doc.statement_at(pos)));
                }
            }
        }

        changed
    }

    pub fn apply(&self, doc: &mut Document) -> Vec<StatementChange> {
        if self.range.is_none() {
            // full change
            return self.apply_full_change(doc);
        } else if let Some(changed_stmt_pos) = doc
            .statements
            .iter()
            .position(|(_, range)| range.contains_range(self.range.unwrap()))
        {
            return self.apply_single_statement_change(doc, changed_stmt_pos);
        } else {
            // change across stmts
            return self.apply_change_across_statements(doc);
        }
    }
}

#[derive(Debug)]
pub struct DocumentChange {
    pub version: i32,
    pub changes: Vec<ChangeParams>,

    changed_statements: Vec<StatementChange>,

    applied: bool,
}

impl From<ChangeFileParams> for DocumentChange {
    fn from(change: ChangeFileParams) -> Self {
        DocumentChange {
            version: change.version,
            changes: change.changes,
            changed_statements: vec![],
            applied: false,
        }
    }
}

impl DocumentChange {
    pub fn apply(&mut self, doc: &mut Document) {
        assert!(!self.applied, "DocumentChange already applied");

        let latest_change_without_range = self
            .changes
            .iter()
            .rposition(|c| c.range.is_none())
            .map(|pos| pos + 1);

        if let Some(pos) = latest_change_without_range {
            self.changed_statements
                .extend(self.changes[pos..].iter().flat_map(|c| c.apply(doc)));
        } else  {
            self.changed_statements
                .extend(self.changes.iter().flat_map(|c| c.apply(doc)));
        }

        doc.version = self.version;
        self.applied = true;
    }

    pub fn collect(&mut self) -> Vec<StatementChange> {
        assert!(self.applied, "DocumentChange not yet applied");
        self.changed_statements.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use text_size::{TextRange, TextSize};

    use super::{super::StatementRef, Document, DocumentChange};
    use pg_fs::PgLspPath;

    #[test]
    fn test_document_apply_changes() {
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.statements.len(), 2);

        let mut change = DocumentChange::new(
            1,
            vec![Change {
                text: ",test from users\nselect 1;".to_string(),
                range: Some(TextRange::new(9.into(), 45.into())),
            }],
        );

        change.apply(&mut d);
        let changed = change.collect_statement_changes();

        assert_eq!(changed.len(), 4);
        assert_eq!(
            changed[0].statement().to_owned(),
            StatementRef {
                document_url: PgLspPath::new("test.sql"),
                text: "select id from users;".to_string(),
                idx: 0
            }
        );
        assert_eq!(
            changed[1].statement().to_owned(),
            StatementRef {
                document_url: PgLspPath::new("test.sql"),
                text: "select * from contacts;".to_string(),
                idx: 1
            }
        );

        assert_eq!("select id,test from users\nselect 1;", d.text);
        assert_eq!(d.statement_ranges.len(), 2);

        for r in &pg_statement_splitter::split(&d.text).ranges {
            assert!(
                d.statement_ranges.iter().any(|x| r == x),
                "should have stmt with range {:#?}",
                r
            );
        }

        assert_eq!(d.statement_ranges[0], TextRange::new(0.into(), 25.into()));
        assert_eq!(d.statement_ranges[1], TextRange::new(26.into(), 35.into()));
    }

    #[test]
    fn test_document_apply_changes_at_end_of_statement() {
        let input = "select id from\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), Some(input.to_string()));

        assert_eq!(d.statement_ranges.len(), 2);

        let stmt_1_range = d.statement_ranges[0];
        let stmt_2_range = d.statement_ranges[1];

        let update_text = " contacts;";

        let update_range = TextRange::new(14.into(), 14.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let mut change = DocumentChange::new(
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
    fn test_document_apply_changes_replacement() {
        let path = PgLspPath::new("test.sql");

        let mut doc = Document::new(path, None);

        let mut c = DocumentChange::new(
            1,
            vec![Change {
                range: None,
                text: "select 1;\nselect 2;".to_string(),
            }],
        );

        c.apply(&mut doc);

        assert_eq!(doc.statement_ref(0).text, "select 1;".to_string());
        assert_eq!(doc.statement_ref(1).text, "select 2;".to_string());
        assert_eq!(
            doc.statement_ranges[0],
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statement_ranges[1],
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );

        let mut c = DocumentChange::new(
            2,
            vec![Change {
                range: Some(TextRange::new(7.into(), 8.into())),
                text: "".to_string(),
            }],
        );

        c.apply(&mut doc);

        assert_eq!(doc.text, "select ;\nselect 2;");
        assert_eq!(doc.statement_refs().len(), 2);
        assert_eq!(doc.statement_ref(0).text, "select ;".to_string());
        assert_eq!(doc.statement_ref(1).text, "select 2;".to_string());
        assert_eq!(
            doc.statement_ranges[0],
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.statement_ranges[1],
            TextRange::new(TextSize::new(9), TextSize::new(18))
        );

        let mut c = DocumentChange::new(
            3,
            vec![Change {
                range: Some(TextRange::new(7.into(), 7.into())),
                text: "!".to_string(),
            }],
        );

        c.apply(&mut doc);

        assert_eq!(doc.text, "select !;\nselect 2;");
        assert_eq!(doc.statement_refs().len(), 2);
        assert_eq!(
            doc.statement_ranges[0],
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statement_ranges[1],
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );

        let mut c = DocumentChange::new(
            4,
            vec![Change {
                range: Some(TextRange::new(7.into(), 8.into())),
                text: "".to_string(),
            }],
        );

        c.apply(&mut doc);

        assert_eq!(doc.text, "select ;\nselect 2;");
        assert_eq!(doc.statement_refs().len(), 2);
        assert_eq!(
            doc.statement_ranges[0],
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.statement_ranges[1],
            TextRange::new(TextSize::new(9), TextSize::new(18))
        );

        let mut c = DocumentChange::new(
            5,
            vec![Change {
                range: Some(TextRange::new(7.into(), 7.into())),
                text: "1".to_string(),
            }],
        );
        c.apply(&mut doc);

        assert_eq!(doc.text, "select 1;\nselect 2;");
        assert_eq!(doc.statement_refs().len(), 2);
        assert_eq!(
            doc.statement_ranges[0],
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statement_ranges[1],
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );
    }

    #[test]
    fn test_document_apply_changes_within_statement() {
        let input = "select id  from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), Some(input.to_string()));

        assert_eq!(d.statement_ranges.len(), 2);

        let stmt_1_range = d.statement_ranges[0];
        let stmt_2_range = d.statement_ranges[1];

        let update_text = ",test";

        let update_range = TextRange::new(9.into(), 10.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let mut change = DocumentChange::new(
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
