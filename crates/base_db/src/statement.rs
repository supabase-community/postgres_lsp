use std::panic::catch_unwind;

use text_size::{TextRange, TextSize};
use tracing::{span, Level};

use crate::{
    diagnostics::Diagnostic,
    document_change::DocumentChange,
    utils::{apply_text_change, edit_from_change},
    PgLspPath,
};

pub struct StatementParams {
    pub text: String,
    pub range: Option<TextRange>,
}

#[derive(Debug, Hash)]
pub struct StatementRef {
    pub version: i32,
    pub range: TextRange,
    pub document_url: PgLspPath,
}

impl Statement {
    pub fn new(params: StatementParams) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_sql::language())
            .expect("Error loading sql language");

        // TODO: ast and ts tree parsing could be done in parallel
        let tree = parser.parse(&params.text, None).unwrap();

        let (ast, native_diagnostics) = {
            let res = parser::parse_sql_statement(&params.text);
            match res {
                Ok(ast) => (Some(ast), None),
                Err(e) => (None, Some(e)),
            }
        };

        let diagnostics = native_diagnostics
            .map(|e| vec![e.into()])
            .unwrap_or_default();

        let cst = if ast.is_some() {
            let res = catch_unwind(|| parser::build_cst(&params.text, ast.as_ref().unwrap()));
            // todo add to diagnostics
            match res {
                Ok(cst) => Some(cst),
                Err(_) => None,
            }
        } else {
            None
        };

        Self {
            range: params.range.unwrap_or(TextRange::new(
                0.into(),
                TextSize::from(u32::try_from(params.text.len()).unwrap()),
            )),
            text: params.text,
            version: 0,
            parser,
            tree,
            ast,
            cst,
        }
    }

    pub fn apply_change(&mut self, change: DocumentChange) {
        assert!(change.range.is_some(), "Change should include range");

        let _span = span!(Level::TRACE, "Statement.apply_change").entered();

        let range = change.range.unwrap();

        let edit = edit_from_change(
            &self.text.as_str(),
            usize::from(range.start()),
            usize::from(range.end()),
            change.text.as_str(),
        );

        self.tree.edit(&edit);

        self.text = apply_text_change(&self.text, &change);

        if change.is_addition() {
            self.range = TextRange::new(
                self.range.start(),
                self.range.end() + TextSize::from(u32::try_from(change.diff_size()).unwrap()),
            );
        } else if change.is_deletion() {
            self.range = TextRange::new(
                self.range.start(),
                self.range.end() - TextSize::from(u32::try_from(change.diff_size()).unwrap()),
            );
        }

        self.tree = self.parser.parse(&self.text, Some(&self.tree)).unwrap();

        let (ast, native_diagnostics) = {
            let res = parser::parse_sql_statement(&self.text);
            match res {
                Ok(ast) => (Some(ast), None),
                Err(e) => (None, Some(e)),
            }
        };

        self.ast = ast;

        self.diagnostics = native_diagnostics
            .map(|e| vec![e.into()])
            .unwrap_or_default();

        self.cst = if self.ast.is_some() {
            let res = catch_unwind(|| parser::build_cst(&self.text, self.ast.as_ref().unwrap()));
            // todo add to diagnostics
            match res {
                Ok(cst) => Some(cst),
                Err(_) => None,
            }
        } else {
            None
        };

        self.version += 1;
    }
}
