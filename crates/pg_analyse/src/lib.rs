mod categories;
pub mod context;
mod filter;
pub mod macros;
mod options;
mod registry;
mod rule;

// Re-exported for use in the `declare_group` macro
pub use pg_diagnostics::category_concat;

use registry::RegistryRuleParams;

pub use crate::categories::{
    ActionCategory, RefactorKind, RuleCategories, RuleCategoriesBuilder, RuleCategory,
    SourceActionKind, SUPPRESSION_ACTION_CATEGORY,
};
pub use crate::filter::{AnalysisFilter, GroupKey, RuleKey};
pub use crate::options::{AnalyzerConfiguration, AnalyzerOptions, AnalyzerRules};
pub use crate::registry::{MetadataRegistry, RegistryVisitor, RuleRegistry, RuleRegistryBuilder};
pub use crate::rule::{
    GroupCategory, Rule, RuleDiagnostic, RuleGroup, RuleMeta, RuleMetadata, RuleSource,
};

pub struct Analyzer<'analyzer> {
    /// Holds the metadata for all the rules statically known to the analyzer
    /// we need this later when we add suppression support
    #[allow(dead_code)]
    metadata: &'analyzer MetadataRegistry,
}

pub struct AnalyzerContext<'a> {
    pub root: pg_query_ext::NodeEnum,
    pub options: &'a AnalyzerOptions,
    pub registry: RuleRegistry,
}

impl<'analyzer> Analyzer<'analyzer> {
    /// Construct a new instance of the analyzer with the given rule registry
    pub fn new(metadata: &'analyzer MetadataRegistry) -> Self {
        Self { metadata }
    }

    pub fn run(self, ctx: AnalyzerContext) -> Vec<RuleDiagnostic> {
        let params = RegistryRuleParams {
            root: &ctx.root,
            options: ctx.options,
        };

        ctx.registry
            .into_iter()
            .flat_map(|rule| (rule.run)(&params))
            .collect::<Vec<_>>()
    }
}
