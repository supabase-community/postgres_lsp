mod rules;

use base_db::{Document, Statement};

pub trait LintFeature {
    fn lint(&self);
}

impl LintFeature for Statement {
    fn lint(&self) {
        todo!(
            "to make it real simple to implement these features, we should provide a way to get the ast node at a position. this can be done by a codegen method similar to get_nodes() that stores the node alongside a range."
        )
    }
}

impl LintFeature for Document {
    fn lint(&self) {
        self.statements
            .iter()
            .for_each(|statement| statement.lint());
    }
}
