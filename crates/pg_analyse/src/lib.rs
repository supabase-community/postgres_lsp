mod categories;
mod context;
mod diagnostics;
mod filter;
mod matcher;
mod options;
mod registry;
mod rule;
mod signals;

pub use crate::categories::{
    ActionCategory, RefactorKind, RuleCategories, RuleCategoriesBuilder, RuleCategory,
    SourceActionKind, SUPPRESSION_ACTION_CATEGORY,
};
// pub use crate::diagnostics::{AnalyzerDiagnostic, RuleError, SuppressionDiagnostic};
pub use crate::matcher::RuleKey;
pub use crate::options::{AnalyzerConfiguration, AnalyzerOptions, AnalyzerRules};
// pub use crate::query::{AddVisitor, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{
    MetadataRegistry, RegistryVisitor,
    RuleRegistry, RuleRegistryBuilder,
};
pub use crate::rule::{Rule};
// pub use crate::rule::{
//     GroupCategory, Rule, RuleAction, RuleDiagnostic, RuleGroup, RuleMeta, RuleMetadata, RuleSource,
//     RuleSourceKind, SuppressAction,
// };

/// Allow filtering a single rule or group of rules by their names
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuleFilter<'a> {
    Group(&'a str),
    Rule(&'a str, &'a str),
}
