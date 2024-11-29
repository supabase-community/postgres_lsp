use std::ops::Sub;
use text_size::{TextLen, TextRange, TextSize};

use crate::workspace::{ChangeFileParams, ChangeParams};

use super::{document::Statement, Document, StatementRef};

#[derive(Debug, PartialEq, Eq)]
pub enum StatementChange {
    Added(Statement),
    Deleted(StatementRef),
    Modified(ChangedStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChangedStatement {
    pub old: Statement,
    pub new_ref: StatementRef,

    pub range: TextRange,
    pub text: String,
}

impl ChangedStatement {
    pub fn new_statement(&self) -> Statement {
        Statement {
            ref_: self.new_ref.clone(),
            text: apply_text_change(&self.old.text, Some(self.range), &self.text),
        }
    }
}

impl StatementChange {
    pub fn statement_ref(&self) -> &StatementRef {
        match self {
            StatementChange::Added(stmt) => &stmt.ref_,
            StatementChange::Deleted(ref_) => ref_,
            StatementChange::Modified(changed) => &changed.new_ref,
        }
    }
}

impl Document {
    pub fn apply_file_change(&mut self, change: &ChangeFileParams) -> Vec<StatementChange> {
        let changes = change
            .changes
            .iter()
            .flat_map(|c| self.apply_change(c))
            .collect();

        self.version = change.version;

        changes
    }

    fn apply_change(&mut self, change: &ChangeParams) -> Vec<StatementChange> {
        self.debug_statements();

        tracing::info!("applying change: {:?}", change);

        let changes = if change.range.is_none() {
            // full change
            self.apply_full_change(change)
        } else if let Some(changed_stmt_pos) = self
            .statements
            .iter()
            .position(|(_, range)| range.contains_range(change.range.unwrap()))
        {
            self.apply_single_statement_change(change, changed_stmt_pos)
        } else if self.statements.iter().all(|(_, r)| {
            let intersection = r.intersect(change.range.unwrap());
            intersection.is_none() || intersection.unwrap().is_empty()
        }) {
            self.apply_unrelated_change(change)
        } else {
            // change across stmts
            self.apply_change_across_statements(change)
        };

        self.debug_statements();

        changes
    }

    fn apply_unrelated_change(&mut self, change: &ChangeParams) -> Vec<StatementChange> {
        tracing::info!("applying unrelated change");
        let mut changed: Vec<StatementChange> = vec![];

        // we need to get the full range between the next and the previous registered statement
        // we need to check equality because we also allow empty intersections to be considered as unrelated
        let prev = self
            .statements
            .iter()
            .rev()
            .find(|(_, r)| r.end() <= change.range.unwrap().start());
        let next = self
            .statements
            .iter()
            .find(|(_, r)| r.start() >= change.range.unwrap().end());

        tracing::info!("prev: {:?}, next: {:?}", prev, next);

        let start = prev.map(|(_, r)| r.end()).unwrap_or(TextSize::new(0));
        let end = next
            .map(|(_, r)| r.start())
            .unwrap_or_else(|| self.content.text_len());

        let extracted_text = self
            .content
            .as_str()
            .get(usize::from(start)..usize::from(end))
            .unwrap();

        tracing::info!("extracted text: {}", extracted_text);

        // insert new statements
        for range in pg_statement_splitter::split(extracted_text).ranges {
            let doc_range = range + start;

            match self
                .statements
                .binary_search_by(|(_, r)| r.start().cmp(&doc_range.start()))
            {
                Ok(_) => {}
                Err(pos) => {
                    let new_id = self.id_generator.next();
                    self.statements.insert(pos, (new_id, doc_range));
                    changed.push(StatementChange::Added(
                        self.statement(&self.statements[pos]),
                    ));
                }
            }
        }

        // then move the rest of the statements accordingly
        self.statements
            .iter_mut()
            .skip_while(|(_, r)| r.end() <= change.range.unwrap().start())
            .for_each(|(_, range)| {
                if change.is_addition() {
                    *range += change.diff_size();
                } else if change.is_deletion() {
                    *range -= change.diff_size();
                }
            });

        self.content = change.apply_to_text(&self.content);

        changed
    }

    fn apply_full_change(&mut self, change: &ChangeParams) -> Vec<StatementChange> {
        tracing::info!("applying full change");
        let mut changed: Vec<StatementChange> = vec![];

        changed.extend(
            self.statements
                .drain(..)
                .map(|(id, _)| {
                    StatementChange::Deleted(StatementRef {
                        id,
                        path: self.path.clone(),
                    })
                })
                .collect::<Vec<StatementChange>>(),
        );

        self.content = change.text.clone();

        for (id, range) in pg_statement_splitter::split(&self.content)
            .ranges
            .iter()
            .map(|r| (self.id_generator.next(), *r))
        {
            self.statements.push((id, range));
            changed.push(StatementChange::Added(Statement {
                ref_: StatementRef {
                    path: self.path.clone(),
                    id,
                },
                text: self.content[range].to_string(),
            }))
        }

        changed
    }

    fn apply_single_statement_change(
        &mut self,
        change: &ChangeParams,
        changed_stmt_pos: usize,
    ) -> Vec<StatementChange> {
        tracing::info!("applying single statement change");
        let mut changed: Vec<StatementChange> = vec![];

        // save the old statement
        let old = self.statement(&self.statements[changed_stmt_pos]);
        let old_range = self.statements[changed_stmt_pos].1;

        // first mutate the target statement
        let new_range = if change.is_addition() {
            Some(TextRange::new(
                self.statements[changed_stmt_pos].1.start(),
                self.statements[changed_stmt_pos].1.end() + change.diff_size(),
            ))
        } else if change.is_deletion() {
            Some(TextRange::new(
                self.statements[changed_stmt_pos].1.start(),
                self.statements[changed_stmt_pos].1.end() - change.diff_size(),
            ))
        } else {
            None
        };

        if let Some(new_range) = new_range {
            // if the new range is empty, remove the statement
            if new_range.is_empty() {
                self.statements.remove(changed_stmt_pos);
                changed.push(StatementChange::Deleted(StatementRef {
                    id: old.ref_.id,
                    path: self.path.clone(),
                }));
            } else {
                let new_id = self.id_generator.next();
                self.statements[changed_stmt_pos] = (new_id, new_range);

                let changed_stmt = ChangedStatement {
                    old,
                    new_ref: self.statement_ref(&self.statements[changed_stmt_pos]),
                    // change must be relative to statement
                    range: change.range.unwrap().sub(old_range.start()),
                    text: change.text.clone(),
                };

                // run it trough the splitter
                let ranges =
                    pg_statement_splitter::split(&changed_stmt.new_statement().text).ranges;
                if ranges.len() > 1 {
                    // if the statement was split, we need to remove the old one and add the new ones
                    self.statements.remove(changed_stmt_pos);
                    for (idx, range) in ranges.iter().enumerate() {
                        let new_id = self.id_generator.next();
                        self.statements.insert(changed_stmt_pos + idx, (new_id, *range));
                        changed.push(StatementChange::Added(
                            self.statement(&self.statements[changed_stmt_pos]),
                        ));
                    }
                } else {
                    changed.push(StatementChange::Modified(changed_stmt));
                }
            }
        }

        // then move the rest of the statements accordingly
        self.statements
            .iter_mut()
            .skip_while(|(_, r)| r.end() <= change.range.unwrap().start())
            .for_each(|(_, range)| {
                if change.is_addition() {
                    *range += change.diff_size();
                } else if change.is_deletion() {
                    *range -= change.diff_size();
                }
            });

        self.content = change.apply_to_text(&self.content);

        changed
    }

    fn apply_change_across_statements(&mut self, change: &ChangeParams) -> Vec<StatementChange> {
        tracing::info!("applying change across statements");
        let mut changed: Vec<StatementChange> = vec![];

        let mut min = change.range.unwrap().start();
        let mut max = change.range.unwrap().end();
        let mut from_idx = 0;
        let mut to_idx = self.statements.len() - 1;

        for (idx, inner_ref) in self
            .statements
            .iter()
            .enumerate()
            .skip_while(|(_, (_, r))| {
                // skip until first changed stmt
                change.range.unwrap().start() > r.end()
            })
            .take_while(|(_, (_, r))| {
                // take until after last changed stmt
                change.range.unwrap().end() >= r.end()
            })
        {
            changed.push(StatementChange::Deleted(self.statement_ref(inner_ref)));

            if inner_ref.1.start() < min {
                min = inner_ref.1.start();
                from_idx = idx;
            }
            if inner_ref.1.end() > max {
                max = inner_ref.1.end();
                to_idx = idx;
            }
        }

        self.content = change.apply_to_text(&self.content);

        if self.content.text_len() < max {
            max = self.content.text_len();
        }

        // get text from min(first_stmt_start, change_start) to max(last_stmt_end, change_end)
        let extracted_text = self
            .content
            .as_str()
            .get(usize::from(min)..usize::from(max))
            .unwrap();

        self.statements.drain(from_idx..(to_idx + 1));

        for range in pg_statement_splitter::split(extracted_text).ranges {
            match self
                .statements
                .binary_search_by(|(_, r)| r.start().cmp(&range.start()))
            {
                Ok(_) => {}
                Err(pos) => {
                    let new_id = self.id_generator.next();
                    self.statements.insert(pos, (new_id, range));
                    changed.push(StatementChange::Added(
                        self.statement(&self.statements[pos]),
                    ));
                }
            }
        }

        changed
    }
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
}

#[cfg(test)]
mod tests {
    use text_size::{TextRange, TextSize};

    use crate::workspace::{server::document::Statement, ChangeFileParams, ChangeParams};

    use super::{super::StatementRef, Document, StatementChange};
    use pg_fs::PgLspPath;

    #[test]
    fn apply_changes() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.statements.len(), 2);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: ",test from users\nselect 1;".to_string(),
                range: Some(TextRange::new(9.into(), 45.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 4);

        assert_eq!(
            changed[0],
            StatementChange::Deleted(StatementRef {
                path: path.clone(),
                id: 0
            })
        );
        assert_eq!(
            changed[1],
            StatementChange::Deleted(StatementRef {
                path: path.clone(),
                id: 1
            })
        );
        assert_eq!(
            changed[2],
            StatementChange::Added(Statement {
                ref_: StatementRef {
                    path: path.clone(),
                    id: 2
                },
                text: "select id,test from users".to_string()
            })
        );
        assert_eq!(
            changed[3],
            StatementChange::Added(Statement {
                ref_: StatementRef {
                    path: path.clone(),
                    id: 3
                },
                text: "select 1;".to_string()
            })
        );

        assert_eq!("select id,test from users\nselect 1;", d.content);
        assert_eq!(d.statements.len(), 2);

        for r in &pg_statement_splitter::split(&d.content).ranges {
            assert!(
                d.statements.iter().any(|x| r == &x.1),
                "should have stmt with range {:#?}",
                r
            );
        }

        assert_eq!(d.statements[0].1, TextRange::new(0.into(), 25.into()));
        assert_eq!(d.statements[1].1, TextRange::new(26.into(), 35.into()));
    }

    #[test]
    fn apply_changes_at_end_of_statement() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from\nselect * from contacts;";

        let mut d = Document::new(path.clone(), input.to_string(), 1);

        assert_eq!(d.statements.len(), 2);

        let stmt_1_range = d.statements[0];
        let stmt_2_range = d.statements[1];

        let update_text = " contacts;";

        let update_range = TextRange::new(14.into(), 14.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let change = ChangeFileParams {
            path: path.clone(),
            version: 2,
            changes: vec![ChangeParams {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        };

        d.apply_file_change(&change);

        assert_eq!(
            "select id from contacts;\nselect * from contacts;",
            d.content
        );
        assert_eq!(d.statements.len(), 2);
        assert_eq!(d.statements[0].1.start(), stmt_1_range.1.start());
        assert_eq!(
            u32::from(d.statements[0].1.end()),
            u32::from(stmt_1_range.1.end()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].1.start()),
            u32::from(stmt_2_range.1.start()) + update_addition
        );
        assert_eq!(
            u32::from(d.statements[1].1.end()),
            u32::from(stmt_2_range.1.end()) + update_addition
        );
    }

    #[test]
    fn apply_changes_replacement() {
        let path = PgLspPath::new("test.sql");

        let mut doc = Document::new(path.clone(), "".to_string(), 0);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: "select 1;\nselect 2;".to_string(),
                range: None,
            }],
        };

        doc.apply_file_change(&change);

        assert_eq!(
            doc.statement(&doc.statements[0]).text,
            "select 1;".to_string()
        );
        assert_eq!(
            doc.statement(&doc.statements[1]).text,
            "select 2;".to_string()
        );
        assert_eq!(
            doc.statements[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statements[1].1,
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );

        let change_2 = ChangeFileParams {
            path: path.clone(),
            version: 2,
            changes: vec![ChangeParams {
                text: "".to_string(),
                range: Some(TextRange::new(7.into(), 8.into())),
            }],
        };

        doc.apply_file_change(&change_2);

        assert_eq!(doc.content, "select ;\nselect 2;");
        assert_eq!(doc.statements.len(), 2);
        assert_eq!(
            doc.statement(&doc.statements[0]).text,
            "select ;".to_string()
        );
        assert_eq!(
            doc.statement(&doc.statements[1]).text,
            "select 2;".to_string()
        );
        assert_eq!(
            doc.statements[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.statements[1].1,
            TextRange::new(TextSize::new(9), TextSize::new(18))
        );

        let change_3 = ChangeFileParams {
            path: path.clone(),
            version: 3,
            changes: vec![ChangeParams {
                text: "!".to_string(),
                range: Some(TextRange::new(7.into(), 7.into())),
            }],
        };

        doc.apply_file_change(&change_3);

        assert_eq!(doc.content, "select !;\nselect 2;");
        assert_eq!(doc.statements.len(), 2);
        assert_eq!(
            doc.statements[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statements[1].1,
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );

        let change_4 = ChangeFileParams {
            path: path.clone(),
            version: 4,
            changes: vec![ChangeParams {
                text: "".to_string(),
                range: Some(TextRange::new(7.into(), 8.into())),
            }],
        };

        doc.apply_file_change(&change_4);

        assert_eq!(doc.content, "select ;\nselect 2;");
        assert_eq!(doc.statements.len(), 2);
        assert_eq!(
            doc.statements[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.statements[1].1,
            TextRange::new(TextSize::new(9), TextSize::new(18))
        );

        let change_5 = ChangeFileParams {
            path: path.clone(),
            version: 5,
            changes: vec![ChangeParams {
                text: "1".to_string(),
                range: Some(TextRange::new(7.into(), 7.into())),
            }],
        };

        doc.apply_file_change(&change_5);

        assert_eq!(doc.content, "select 1;\nselect 2;");
        assert_eq!(doc.statements.len(), 2);
        assert_eq!(
            doc.statements[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.statements[1].1,
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );
    }

    #[test]
    fn apply_changes_within_statement() {
        let input = "select id  from users;\nselect * from contacts;";
        let path = PgLspPath::new("test.sql");

        let mut doc = Document::new(path.clone(), input.to_string(), 0);

        assert_eq!(doc.statements.len(), 2);

        let stmt_1_range = doc.statements[0];
        let stmt_2_range = doc.statements[1];

        let update_text = ",test";

        let update_range = TextRange::new(9.into(), 10.into());

        let update_text_len = u32::try_from(update_text.chars().count()).unwrap();
        let update_addition = update_text_len - u32::from(update_range.len());

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: update_text.to_string(),
                range: Some(update_range),
            }],
        };

        doc.apply_file_change(&change);

        assert_eq!(
            "select id,test from users;\nselect * from contacts;",
            doc.content
        );
        assert_eq!(doc.statements.len(), 2);
        assert_eq!(doc.statements[0].1.start(), stmt_1_range.1.start());
        assert_eq!(
            u32::from(doc.statements[0].1.end()),
            u32::from(stmt_1_range.1.end()) + update_addition
        );
        assert_eq!(
            u32::from(doc.statements[1].1.start()),
            u32::from(stmt_2_range.1.start()) + update_addition
        );
        assert_eq!(
            u32::from(doc.statements[1].1.end()),
            u32::from(stmt_2_range.1.end()) + update_addition
        );
    }
}
