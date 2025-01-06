use std::ops::{Add, Sub};
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
    #[allow(dead_code)]
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

        let mut changed: Vec<StatementChange> = Vec::with_capacity(self.statements.len());

        tracing::info!("applying change: {:?}", change);

        if change.range.is_none() {
            // apply full text change and return early
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

            return changed;
        }

        // no matter where the change is, we can never be sure if its a modification or a deletion/addition
        // e.g. if a statement is "select 1", and the change is "select 2; select 2", its an addition even though its in the middle of the statement.
        // hence we only have three "real" cases:
        // 1. the change touches no statement at all (addition)
        // 2. the change touches exactly one statement AND splitting the statement results in just
        //    one statement (modification)
        // 3. the change touches more than one statement (addition/deletion)

        let new_content = change.apply_to_text(&self.content);

        let mut affected = vec![];

        for (idx, (id, r)) in self.statements.iter_mut().enumerate() {
            if r.intersect(change.range.unwrap()).is_some() {
                affected.push((idx, (*id, *r)));
            } else if r.start() > change.range.unwrap().end() {
                if change.is_addition() {
                    *r += change.diff_size();
                } else if change.is_deletion() {
                    *r -= change.diff_size();
                }
            }
        }

        // special case: if no statement is affected, the affected range is between the prev and
        // the next statement
        if affected.is_empty() {
            // since we do not now whether the change should be part of the previous statement, we
            // will take the range form the start of the previous statement to the start of the
            // next statement. if the resulting split has length one, we will modify it instead.

            let (from_stmt_index, from_stmt_range) = self
                .statements
                .iter()
                .enumerate()
                .rev()
                .find(|(_, (_, r))| r.start() <= change.range.unwrap().start())
                .map(|(i, (_, r))| (i, *r))
                .unwrap_or((0, TextRange::empty(TextSize::new(0))));


            let start = from_stmt_range.start();

            let end = self
                .statements
                .iter()
                .find(|(_, r)| r.start() >= change.range.unwrap().end())
                .map(|(_, r)| r.start())
                .unwrap_or_else(|| self.content.text_len());

            let affected = new_content
                .as_str()
                .get(usize::from(start)..usize::from(end))
                .unwrap();

            let new_ranges = pg_statement_splitter::split(affected).ranges;

            if new_ranges.len() == 1 && !from_stmt_range.is_empty() {
                if !change.is_whitespace() {
                    // modify previous statement
                    let new_stmt = &new_ranges[0];

                    let new_id = self.id_generator.next();
                    let old_stmt = self.statement(&self.statements[from_stmt_index]);
                    self.statements[from_stmt_index] = (new_id, new_stmt.add(start));

                    println!("change prev");
                    let changed_stmt = ChangedStatement {
                        old: old_stmt,
                        new_ref: self.statement_ref(&self.statements[from_stmt_index]),
                        // change must be relative to statement
                        // TODO: range and text must be filled up with whitespaces
                        range: change.range.unwrap().sub(from_stmt_range.start()),
                        text: change.text.clone(),
                    };

                    changed.push(StatementChange::Modified(changed_stmt));
                }
            } else {
                // add new statements
                for range in new_ranges {
                    let doc_range = range + start;
                    match self
                        .statements
                        .binary_search_by(|(_, r)| r.start().cmp(&doc_range.start()))
                    {
                        Ok(_) => {}
                        Err(pos) => {
                            let new_id = self.id_generator.next();
                            self.statements.insert(pos, (new_id, doc_range));
                            changed.push(StatementChange::Added(Statement {
                                ref_: StatementRef {
                                    path: self.path.clone(),
                                    id: new_id,
                                },
                                text: new_content[doc_range].to_string(),
                            }));
                        }
                    }
                }
            }
        } else {
            // get full affected range
            let mut start = change.range.unwrap().start();
            let mut end = change.range.unwrap().end();

            if end > new_content.text_len() {
                end = new_content.text_len();
            }

            for (_, (_, r)) in &affected {
                // adjust the range to the new content
                let adjusted_start = if r.start() >= change.range.unwrap().end() {
                    r.start() + change.diff_size()
                } else {
                    r.start()
                };
                let adjusted_end = if r.end() >= change.range.unwrap().end() {
                    if change.is_addition() {
                        r.end() + change.diff_size()
                    } else {
                        r.end() - change.diff_size()
                    }
                } else {
                    r.end()
                };

                if adjusted_start < start {
                    start = adjusted_start;
                }
                if adjusted_end > end && adjusted_end <= new_content.text_len() {
                    end = adjusted_end;
                }
            }

            let changed_content = new_content
                .as_str()
                .get(usize::from(start)..usize::from(end))
                .unwrap();

            let ranges = pg_statement_splitter::split(changed_content).ranges;

            if affected.len() == 1 && ranges.len() == 1 {
                if !change.is_whitespace() {
                    // from one to one, so we do a modification
                    let stmt = &affected[0];
                    let new_stmt = &ranges[0];

                    let new_id = self.id_generator.next();
                    self.statements[stmt.0] = (new_id, new_stmt.add(start));

                        println!("change one to one");
                    let changed_stmt = ChangedStatement {
                        old: self.statement(&stmt.1),
                        new_ref: self.statement_ref(&self.statements[stmt.0]),
                        // change must be relative to statement
                        range: change.range.unwrap().sub(stmt.1 .1.start()),
                        text: change.text.clone(),
                    };

                    changed.push(StatementChange::Modified(changed_stmt));
                }
            } else {
                // delete and add new ones
                for (_, (id, r)) in &affected {
                    changed.push(StatementChange::Deleted(self.statement_ref(&(*id, *r))));
                }

                // remove affected statements
                self.statements
                    .retain(|(id, _)| !affected.iter().any(|(affected_id, _)| id == affected_id));

                // add new statements
                for range in ranges {
                    match self
                        .statements
                        .binary_search_by(|(_, r)| r.start().cmp(&range.start()))
                    {
                        Ok(_) => {}
                        Err(pos) => {
                            let new_id = self.id_generator.next();
                            self.statements.insert(pos, (new_id, range));
                            changed.push(StatementChange::Added(Statement {
                                ref_: StatementRef {
                                    path: self.path.clone(),
                                    id: new_id,
                                },
                                text: new_content[range].to_string(),
                            }));
                        }
                    }
                }
            }
        }

        self.content = new_content;

        self.debug_statements();

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
    if end < text.len() {
        new_text.push_str(&text[end..]);
    }

    new_text
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
    use text_size::{TextRange, TextSize};

    use crate::workspace::{server::document::Statement, ChangeFileParams, ChangeParams};

    use super::{super::StatementRef, Document, StatementChange};
    use pg_fs::PgLspPath;

    #[test]
    fn within_statements() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\n\n\n\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.statements.len(), 2);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: "select 1;".to_string(),
                range: Some(TextRange::new(23.into(), 23.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 1);
        assert!(
            matches!(&changed[0], StatementChange::Added(Statement { ref_: _, text }) if text == "select 1;")
        );

        assert_document_integrity(&d);
    }

    #[test]
    fn julians_sample() {
        let path = PgLspPath::new("test.sql");
        let input = "select\n  *\nfrom\n  test;\n\nselect\n\nalter table test\ndrop column id;";
        let mut d = Document::new(path.clone(), input.to_string(), 0);

        assert_eq!(d.statements.len(), 3);
        println!("{:#?}", d.statements);

        let change1 = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: " ".to_string(),
                range: Some(TextRange::new(31.into(), 31.into())),
            }],
        };

        let changed1 = d.apply_file_change(&change1);
        println!("after change 1");
        println!("{:#?}", d.content);
        println!("{:#?}", d.statements);
        println!("{:#?}", changed1);

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
        println!("after change 2");
        println!("{:#?}", d.content);
        println!("{:#?}", d.statements);
        println!("{:#?}", changed2);

        let change3 = ChangeFileParams {
            path: path.clone(),
            version: 3,
            changes: vec![ChangeParams {
                text: "".to_string(),
                range: Some(TextRange::new(32.into(), 33.into())),
            }],
        };

        let changed3 = d.apply_file_change(&change3);
        println!("after change 3");
        println!("{:#?}", d.content);
        println!("{:#?}", d.statements);
        println!("{:#?}", changed3);

        //
        // assert_eq!(changed.len(), 4);
        // assert!(matches!(
        //     changed[0],
        //     StatementChange::Deleted(StatementRef { id: 0, .. })
        // ));
        // assert!(matches!(
        //     changed[1],
        //     StatementChange::Deleted(StatementRef { id: 1, .. })
        // ));
        // assert!(
        //     matches!(&changed[2], StatementChange::Added(Statement { ref_: _, text }) if text == "select id,test from users;")
        // );
        // assert!(
        //     matches!(&changed[3], StatementChange::Added(Statement { ref_: _, text }) if text == "select 1;")
        // );
        //
        assert_document_integrity(&d);
    }

    #[test]
    fn across_statements() {
        let path = PgLspPath::new("test.sql");
        let input = "select id from users;\nselect * from contacts;";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.statements.len(), 2);

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
            StatementChange::Deleted(StatementRef { id: 0, .. })
        ));
        assert!(matches!(
            changed[1],
            StatementChange::Deleted(StatementRef { id: 1, .. })
        ));
        assert!(
            matches!(&changed[2], StatementChange::Added(Statement { ref_: _, text }) if text == "select id,test from users;")
        );
        assert!(
            matches!(&changed[3], StatementChange::Added(Statement { ref_: _, text }) if text == "select 1;")
        );

        assert_document_integrity(&d);
    }

    fn assert_document_integrity(d: &Document) {
        let ranges = pg_statement_splitter::split(&d.content).ranges;

        assert!(ranges.len() == d.statements.len());

        assert!(ranges
            .iter()
            .all(|r| { d.statements.iter().any(|(_, stmt_range)| stmt_range == r) }));
    }

    #[test]
    fn append_to_statement() {
        let path = PgLspPath::new("test.sql");
        let input = "select id";

        let mut d = Document::new(PgLspPath::new("test.sql"), input.to_string(), 0);

        assert_eq!(d.statements.len(), 1);

        let change = ChangeFileParams {
            path: path.clone(),
            version: 1,
            changes: vec![ChangeParams {
                text: " ".to_string(),
                range: Some(TextRange::new(9.into(), 10.into())),
            }],
        };

        let changed = d.apply_file_change(&change);

        assert_eq!(changed.len(), 1);
        matches!(changed[0], StatementChange::Modified(_));

        assert_document_integrity(&d);
    }

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

        assert_document_integrity(&d);
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

        let changes = d.apply_file_change(&change);

        assert_eq!(changes.len(), 1);

        assert!(matches!(changes[0], StatementChange::Modified(_)));

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

        assert_document_integrity(&doc);
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

        assert_document_integrity(&doc);
    }
}
