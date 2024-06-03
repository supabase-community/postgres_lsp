use lazy_static::lazy_static;
use text_size::TextSize;
pub use violations::{RuleViolation, RuleViolationKind, ViolationMessage};

use crate::rules::ban_drop_column::ban_drop_column;

mod rules;
mod violations;

pub struct LinterParams<'a> {
    pub ast: &'a sql_parser::AstNode,
    pub enriched_ast: Option<&'a sql_parser::EnrichedAst>,
}

#[derive(Clone)]
pub struct LintRule {
    pub name: RuleViolationKind,
    func: fn(&LinterParams) -> Vec<RuleViolation>,
    pub messages: Vec<ViolationMessage>,
}

lazy_static! {
    pub static ref RULES: Vec<LintRule> = vec![LintRule {
        name: RuleViolationKind::BanDropColumn,
        func: ban_drop_column,
        messages: vec![ViolationMessage::Note(
            "Dropping a column may break existing clients.".into()
        ),],
    }];
}

pub fn check_sql(params: LinterParams) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for rule in RULES.iter() {
        errs.extend((rule.func)(&params));
    }

    errs.sort_by_key(|v| match v.range {
        Some(r) => r.start(),
        None => TextSize::new(0),
    });

    errs
}
