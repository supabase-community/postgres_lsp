use line_index::LineIndex;
use parser::get_statements;
use text_size::{TextLen, TextRange, TextSize};

use crate::{statement::Statement, utils::apply_text_change};

#[derive(Debug)]
pub struct FileChangesParams {
    pub version: i32,
    pub changes: Vec<FileChange>,
}

#[derive(Debug)]
pub struct FileChange {
    /// The range of the file that changed. If `None`, the whole file changed.
    pub range: Option<TextRange>,
    pub text: String,
}

impl FileChange {
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
}

#[derive(Debug)]
pub struct SourceFileParams {
    pub text: String,
}

pub struct SourceFile {
    pub text: String,
    pub version: i32,
    pub statements: Vec<(TextRange, Statement)>,
    pub line_index: LineIndex,
}

impl SourceFile {
    pub fn new(params: SourceFileParams) -> SourceFile {
        SourceFile {
            version: 0,
            statements: get_statements(&params.text)
                .iter()
                .map(|(range, content)| {
                    let statement = Statement::new(content.clone());
                    (range.clone(), statement)
                })
                .collect(),
            line_index: LineIndex::new(&params.text),
            text: params.text,
        }
    }

    pub fn apply_changes(&mut self, params: FileChangesParams) {
        // TODO: optimize this by searching for the last change without a range and start applying
        // from there

        params.changes.iter().for_each(|c| {
            self.apply_change(c);
        });
        self.version = params.version;
    }

    fn apply_change(&mut self, change: &FileChange) {
        if change.range.is_none() {
            self.text = change.text.clone();
            self.line_index = LineIndex::new(&self.text);
            self.statements = get_statements(&self.text)
                .iter()
                .map(|(range, content)| {
                    let statement = Statement::new(content.clone());
                    (range.clone(), statement)
                })
                .collect();
            return;
        }

        self.text = apply_text_change(&self.text, &change);
        self.line_index = LineIndex::new(&self.text);

        if let Some(changed_stmt_pos) = self
            .statements
            .iter()
            .position(|(range, _)| range.contains_range(change.range.unwrap()))
        {
            let stmt_range = self.statements[changed_stmt_pos].0;
            self.statements
                .get_mut(changed_stmt_pos)
                .unwrap()
                .1
                .apply_change(FileChange {
                    text: change.text.clone(),
                    // range must be relative to the start of the statement
                    range: change.range.unwrap().checked_sub(stmt_range.start()),
                });
            self.statements
                .iter_mut()
                .skip_while(|(range, _)| range.start() <= change.range.unwrap().end())
                .for_each(|(range, _)| {
                    if range.start() > stmt_range.end() {
                        if change.is_addition() {
                            range.checked_add(change.diff_size());
                        } else if change.is_deletion() {
                            range.checked_sub(change.diff_size());
                        }
                    }
                });
        } else {
            // remove all statements that are affected by this change,
            let mut min = change.range.unwrap().start();
            let mut max = change.range.unwrap().end();

            let change_range = change.range.unwrap();

            for (range, _) in self.statements.extract_if(|(range, _)| {
                (change_range.start() < range.end()) && (change_range.end() > range.start())
            }) {
                if range.start() < min {
                    min = range.start();
                }
                if range.end() > max {
                    max = range.end();
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
            // and reparse
            self.statements
                .extend(
                    get_statements(extracted_text)
                        .iter()
                        .map(|(range, content)| {
                            let statement = Statement::new(content.clone());
                            (range.clone(), statement)
                        }),
                );
        }
    }
}
