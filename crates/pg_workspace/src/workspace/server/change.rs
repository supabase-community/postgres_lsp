use std::ops::{Add, Sub};
use text_size::{TextLen, TextRange, TextSize};

use crate::workspace::{ChangeFileParams, ChangeParams};

use super::{Document, Statement};

#[derive(Debug, PartialEq, Eq)]
pub enum StatementChange {
    Added(AddedStatement),
    Deleted(Statement),
    Modified(ModifiedStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddedStatement {
    pub stmt: Statement,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModifiedStatement {
    pub old_stmt: Statement,
    pub old_stmt_text: String,

    pub new_stmt: Statement,
    pub new_stmt_text: String,

    pub change_range: TextRange,
    pub change_text: String,
}

impl StatementChange {
    #[allow(dead_code)]
    pub fn statement(&self) -> &Statement {
        match self {
            StatementChange::Added(stmt) => &stmt.stmt,
            StatementChange::Deleted(stmt) => stmt,
            StatementChange::Modified(changed) => &changed.new_stmt,
        }
    }
}

/// Returns all relevant details about the change and its effects on the current state of the document.
struct Affected {
    /// Full range of the change, including the range of all statements that intersect with the change
    affected_range: TextRange,
    /// All indices of affected statement positions
    affected_indices: Vec<usize>,
    /// The index of the first statement position before the change, if any
    prev_index: Option<usize>,
    /// The index of the first statement position after the change, if any
    next_index: Option<usize>,
    /// the full affected range includng the prev and next statement
    full_affected_range: TextRange,
}

impl Document {
    /// Applies a file change to the document and returns the affected statements
    pub fn apply_file_change(&mut self, change: &ChangeFileParams) -> Vec<StatementChange> {
        let changes = change
            .changes
            .iter()
            .flat_map(|c| self.apply_change(c))
            .collect();

        self.version = change.version;

        changes
    }

    /// Applies a full change to the document and returns the affected statements
    fn apply_full_change(&mut self, text: &str) -> Vec<StatementChange> {
        let mut changes = Vec::new();

        changes.extend(self.positions.drain(..).map(|(id, _)| {
            StatementChange::Deleted(Statement {
                id,
                path: self.path.clone(),
            })
        }));

        self.content = text.to_string();

        changes.extend(
            pg_statement_splitter::split(&self.content)
                .ranges
                .into_iter()
                .map(|range| {
                    let id = self.id_generator.next();
                    let text = self.content[range].to_string();
                    self.positions.push((id, range));

                    StatementChange::Added(AddedStatement {
                        stmt: Statement {
                            path: self.path.clone(),
                            id,
                        },
                        text,
                    })
                }),
        );

        changes
    }

    fn insert_statement(&mut self, range: TextRange) -> usize {
        let pos = self
            .positions
            .binary_search_by(|(_, r)| r.start().cmp(&range.start()))
            .unwrap_err();

        let new_id = self.id_generator.next();
        self.positions.insert(pos, (new_id, range));

        new_id
    }

    /// Returns all relevant details about the change and its effects on the current state of the document.
    /// - The affected range is the full range of the change, including the range of all statements that intersect with the change
    /// - All indices of affected statement positions
    /// - The index of the first statement position before the change, if any
    /// - The index of the first statement position after the change, if any
    /// - the full affected range includng the prev and next statement
    fn get_affected(
        &self,
        change_range: TextRange,
        content_size: TextSize,
        diff_size: TextSize,
        is_addition: bool,
    ) -> Affected {
        let mut start = change_range.start();
        let mut end = change_range.end().min(content_size);

        let mut affected_indices = Vec::new();
        let mut prev_index = None;
        let mut next_index = None;

        for (index, (_, pos_range)) in self.positions.iter().enumerate() {
            if pos_range.intersect(change_range).is_some() {
                affected_indices.push(index);
                start = start.min(pos_range.start());
                end = end.max(pos_range.end());
            } else if pos_range.end() <= change_range.start() {
                prev_index = Some(index);
            } else if pos_range.start() >= change_range.end() && next_index.is_none() {
                next_index = Some(index);
                break;
            }
        }

        let start_incl = prev_index
            .map(|i| self.positions[i].1.start())
            .unwrap_or(start);
        let end_incl = next_index
            .map(|i| self.positions[i].1.end())
            .unwrap_or_else(|| end);

        let end_incl = if is_addition {
            end_incl.add(diff_size)
        } else {
            end_incl.sub(diff_size)
        };

        let end = if is_addition {
            end.add(diff_size)
        } else {
            end.sub(diff_size)
        };

        Affected {
            affected_range: TextRange::new(start, end.min(content_size)),
            affected_indices,
            prev_index,
            next_index,
            full_affected_range: TextRange::new(start_incl, end_incl.min(content_size)),
        }
    }

    fn move_ranges(&mut self, offset: TextSize, diff_size: TextSize, is_addition: bool) {
        self.positions
            .iter_mut()
            .skip_while(|(_, r)| offset > r.start())
            .for_each(|(_, range)| {
                let new_range = if is_addition {
                    range.add(diff_size)
                } else {
                    range.sub(diff_size)
                };

                *range = new_range;
            });
    }

    /// Applies a single change to the document and returns the affected statements
    fn apply_change(&mut self, change: &ChangeParams) -> Vec<StatementChange> {
        tracing::info!("applying change: {:?}", change);

        // if range is none, we have a full change
        if change.range.is_none() {
            return self.apply_full_change(&change.text);
        }

        // i spent a relatively large amount of time thinking about how to handle range changes
        // properly. there are quite a few edge cases to consider. I eventually skipped most of
        // them, because the complexity is not worth the return for now. we might want to revisit
        // this later though.

        let mut changed: Vec<StatementChange> = Vec::with_capacity(self.positions.len());

        let change_range = change.range.unwrap();
        let new_content = change.apply_to_text(&self.content);

        // we first need to determine the affected range and all affected statements, as well as
        // the index of the prev and the next statement, if any. The full affected range is the
        // affected range expanded to the start of the previous statement and the end of the next
        let Affected {
            affected_range,
            affected_indices,
            prev_index,
            next_index,
            full_affected_range,
        } = self.get_affected(
            change_range,
            new_content.text_len(),
            change.diff_size(),
            change.is_addition(),
        );

        // if within a statement, we can modify it if the change results in also a single statement
        if affected_indices.len() == 1 {
            let changed_content = new_content
                .as_str()
                .get(usize::from(affected_range.start())..usize::from(affected_range.end()))
                .unwrap();

            let new_ranges = pg_statement_splitter::split(changed_content).ranges;

            if new_ranges.len() == 1 {
                if change.is_whitespace() {
                    self.move_ranges(
                        affected_range.end(),
                        change.diff_size(),
                        change.is_addition(),
                    );

                    self.content = new_content;

                    return changed;
                }

                let affected_idx = affected_indices[0];
                let new_range = new_ranges[0].add(affected_range.start());
                let (old_id, old_range) = self.positions[affected_idx];

                // move all statements after the afffected range
                self.move_ranges(old_range.end(), change.diff_size(), change.is_addition());

                let new_id = self.id_generator.next();
                self.positions[affected_idx] = (new_id, new_range);

                changed.push(StatementChange::Modified(ModifiedStatement {
                    old_stmt: Statement {
                        id: old_id,
                        path: self.path.clone(),
                    },
                    old_stmt_text: self.content[old_range].to_string(),

                    new_stmt: Statement {
                        id: new_id,
                        path: self.path.clone(),
                    },
                    new_stmt_text: changed_content[new_ranges[0]].to_string(),
                    // change must be relative to the statement
                    change_text: change.text.clone(),
                    change_range,
                }));

                self.content = new_content;

                return changed;
            }
        }

        // in any other case, parse the full affected range
        let changed_content = new_content
            .as_str()
            .get(usize::from(full_affected_range.start())..usize::from(full_affected_range.end()))
            .unwrap();

        let new_ranges = pg_statement_splitter::split(changed_content).ranges;

        // delete and add new ones
        if let Some(next_index) = next_index {
            changed.push(StatementChange::Deleted(Statement {
                id: self.positions[next_index].0,
                path: self.path.clone(),
            }));
            self.positions.remove(next_index);
        }
        for idx in affected_indices.iter().rev() {
            changed.push(StatementChange::Deleted(Statement {
                id: self.positions[*idx].0,
                path: self.path.clone(),
            }));
            self.positions.remove(*idx);
        }
        if let Some(prev_index) = prev_index {
            changed.push(StatementChange::Deleted(Statement {
                id: self.positions[prev_index].0,
                path: self.path.clone(),
            }));
            self.positions.remove(prev_index);
        }

        new_ranges.iter().for_each(|range| {
            let actual_range = range.add(full_affected_range.start());
            let new_id = self.insert_statement(actual_range);
            changed.push(StatementChange::Added(AddedStatement {
                stmt: Statement {
                    id: new_id,
                    path: self.path.clone(),
                },
                text: new_content[actual_range].to_string(),
            }));
        });

        // move all statements after the afffected range
        self.move_ranges(
            full_affected_range.end(),
            change.diff_size(),
            change.is_addition(),
        );

        self.content = new_content;

        changed
    }
}

impl ChangeParams {
    pub fn is_whitespace(&self) -> bool {
        self.text.chars().count() > 0 && self.text.chars().all(char::is_whitespace)
    }

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
        if end < text.len() {
            new_text.push_str(&text[end..]);
        }

        new_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use text_size::TextRange;

    use crate::workspace::{ChangeFileParams, ChangeParams};

    use pg_fs::PgLspPath;

    impl Document {
        pub fn get_text(&self, idx: usize) -> String {
            self.content[self.positions[idx].1.start().into()..self.positions[idx].1.end().into()]
                .to_string()
        }
    }

    fn assert_document_integrity(d: &Document) {
        let ranges = pg_statement_splitter::split(&d.content).ranges;

        assert!(ranges.len() == d.positions.len());

        assert!(ranges
            .iter()
            .all(|r| { d.positions.iter().any(|(_, stmt_range)| stmt_range == r) }));
    }

    #[test]
    fn within_statements() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\n\n\n\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.positions.len(), 2);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: "select 1;".to_string(),
                range: Some(TextRange::new(23.into(), 23.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 5);
        assert_eq!(
            changed
                .iter()
                .filter(|c| matches!(c, StatementChange::Deleted(_)))
                .count(),
            2
        );
        assert_eq!(
            changed
                .iter()
                .filter(|c| matches!(c, StatementChange::Added(_)))
                .count(),
            3
        );

        assert_document_integrity(&d);
    }

    #[test]
    fn julians_sample() {
        let path = PgLspPath::new("test.sql");
        let input = "select\n  *\nfrom\n  test;\n\nselect\n\nalter table test\n\ndrop column id;";
        let mut d = Document::new(path.clone(), input.to_string(), 0);

        assert_eq!(d.positions.len(), 4);

        let change1 = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: " ".to_string(),
                range: Some(TextRange::new(31.into(), 31.into())),
            }],
        };

        let changed1 = d.apply_file_change(&change1);
        assert_eq!(
            changed1.len(),
            0,
            "should not emit change if its only whitespace"
        );
        assert_eq!(
            d.content,
            "select\n  *\nfrom\n  test;\n\nselect \n\nalter table test\n\ndrop column id;"
        );
        assert_document_integrity(&d);

        // problem: this creates a new statement
        let change2 = ChangeFileParams {
            path: path.clone(),
            version: 2,
            changes: vec![ChangeParams {
                text: ";".to_string(),
                range: Some(TextRange::new(32.into(), 32.into())),
            }],
        };

        let changed2 = d.apply_file_change(&change2);
        assert_eq!(changed2.len(), 4);
        assert_eq!(
            changed2
                .iter()
                .filter(|c| matches!(c, StatementChange::Deleted(_)))
                .count(),
            2
        );
        assert_eq!(
            changed2
                .iter()
                .filter(|c| matches!(c, StatementChange::Added(_)))
                .count(),
            2
        );
        assert_document_integrity(&d);

        let change3 = ChangeFileParams {
            path: path.clone(),
            version: 3,
            changes: vec![ChangeParams {
                text: "".to_string(),
                range: Some(TextRange::new(32.into(), 33.into())),
            }],
        };

        let changed3 = d.apply_file_change(&change3);
        assert_eq!(changed3.len(), 1);
        assert!(matches!(&changed3[0], StatementChange::Modified(_)));
        assert_eq!(
            d.content,
            "select\n  *\nfrom\n  test;\n\nselect \n\nalter table test\n\ndrop column id;"
        );
        match &changed3[0] {
            StatementChange::Modified(changed) => {
                assert_eq!(changed.old_stmt_text, "select ;");
                assert_eq!(changed.new_stmt_text, "select");
                assert_eq!(changed.change_text, "");
                assert_eq!(changed.change_range, TextRange::new(32.into(), 33.into()));
            }
            _ => panic!("expected modified statement"),
        }
        assert_document_integrity(&d);
    }

    #[test]
    fn across_statements() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.positions.len(), 2);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: ",test from users;\nselect 1;".to_string(),
                range: Some(TextRange::new(9.into(), 45.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 4);
        assert!(matches!(
            changed[0],
            StatementChange::Deleted(Statement { id: 1, .. })
        ));
        assert!(matches!(
            changed[1],
            StatementChange::Deleted(Statement { id: 0, .. })
        ));
        assert!(
            matches!(&changed[2], StatementChange::Added(AddedStatement { stmt: _, text }) if text == "select id,test from users;")
        );
        assert!(
            matches!(&changed[3], StatementChange::Added(AddedStatement { stmt: _, text }) if text == "select 1;")
        );

        assert_document_integrity(&d);
    }

    #[test]
    fn append_whitespace_to_statement() {
        let path = PgLspPath::new("test.sql");
        let input = "select id";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.positions.len(), 1);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: " ".to_string(),
                range: Some(TextRange::new(9.into(), 10.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 0);

        assert_document_integrity(&d);
    }

    #[test]
    fn apply_changes() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.positions.len(), 2);

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
            StatementChange::Deleted(Statement {
                path: path.clone(),
                id: 1
            })
        );
        assert_eq!(
            changed[1],
            StatementChange::Deleted(Statement {
                path: path.clone(),
                id: 0
            })
        );
        assert_eq!(
            changed[2],
            StatementChange::Added(AddedStatement {
                stmt: Statement {
                    path: path.clone(),
                    id: 2
                },
                text: "select id,test from users".to_string()
            })
        );
        assert_eq!(
            changed[3],
            StatementChange::Added(AddedStatement {
                stmt: Statement {
                    path: path.clone(),
                    id: 3
                },
                text: "select 1;".to_string()
            })
        );

        assert_eq!("select id,test from users\nselect 1;", d.content);

        assert_document_integrity(&d);
    }

    #[test]
    fn apply_changes_at_end_of_statement() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from\nselect * from contacts;";

        let mut d = Document::new(path.clone(), input.to_string(), 1);

        assert_eq!(d.positions.len(), 2);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 2,
            changes: vec![ChangeParams {
                text: " contacts;".to_string(),
                range: Some(TextRange::new(14.into(), 14.into())),
            }],
        };

        let changes = d.apply_file_change(&change);

        assert_eq!(changes.len(), 1);

        assert!(matches!(changes[0], StatementChange::Modified(_)));

        assert_eq!(
            "select id from contacts;\nselect * from contacts;",
            d.content
        );

        assert_document_integrity(&d);
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

        assert_eq!(doc.get_text(0), "select 1;".to_string());
        assert_eq!(doc.get_text(1), "select 2;".to_string());
        assert_eq!(
            doc.positions[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.positions[1].1,
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
        assert_eq!(doc.positions.len(), 2);
        assert_eq!(doc.get_text(0), "select ;".to_string());
        assert_eq!(doc.get_text(1), "select 2;".to_string());
        assert_eq!(
            doc.positions[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.positions[1].1,
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
        assert_eq!(doc.positions.len(), 2);
        assert_eq!(
            doc.positions[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.positions[1].1,
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
        assert_eq!(doc.positions.len(), 2);
        assert_eq!(
            doc.positions[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(8))
        );
        assert_eq!(
            doc.positions[1].1,
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
        assert_eq!(doc.positions.len(), 2);
        assert_eq!(
            doc.positions[0].1,
            TextRange::new(TextSize::new(0), TextSize::new(9))
        );
        assert_eq!(
            doc.positions[1].1,
            TextRange::new(TextSize::new(10), TextSize::new(19))
        );

        assert_document_integrity(&doc);
    }

    #[test]
    fn apply_changes_within_statement() {
        let input = "select id  from users;\nselect * from contacts;";
        let path = PgLspPath::new("test.sql");

        let mut doc = Document::new(path.clone(), input.to_string(), 0);

        assert_eq!(doc.positions.len(), 2);

        let stmt_1_range = doc.positions[0];
        let stmt_2_range = doc.positions[1];

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
        assert_eq!(doc.positions.len(), 2);
        assert_eq!(doc.positions[0].1.start(), stmt_1_range.1.start());
        assert_eq!(
            u32::from(doc.positions[0].1.end()),
            u32::from(stmt_1_range.1.end()) + update_addition
        );
        assert_eq!(
            u32::from(doc.positions[1].1.start()),
            u32::from(stmt_2_range.1.start()) + update_addition
        );
        assert_eq!(
            u32::from(doc.positions[1].1.end()),
            u32::from(stmt_2_range.1.end()) + update_addition
        );

        assert_document_integrity(&doc);
    }
}