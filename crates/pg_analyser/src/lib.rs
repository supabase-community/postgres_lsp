use std::{ops::Deref, sync::LazyLock};

use pg_analyse::{
    AnalyserOptions, AnalysisFilter, MetadataRegistry, RegistryRuleParams, RuleDiagnostic,
    RuleRegistry,
};
pub use registry::visit_registry;

mod lint;
pub mod options;
mod registry;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

/// Main entry point to the analyser.
pub struct Analyser<'a> {
    /// Holds the metadata for all the rules statically known to the analyser
    /// we need this later when we add suppression support
    #[allow(dead_code)]
    metadata: &'a MetadataRegistry,

    /// Holds all rule options
    options: &'a AnalyserOptions,

    /// Holds all rules
    registry: RuleRegistry,
}

pub struct AnalyserContext<'a> {
    pub root: &'a pg_query_ext::NodeEnum,
}

pub struct AnalyserConfig<'a> {
    pub options: &'a AnalyserOptions,
    pub filter: AnalysisFilter<'a>,
}

impl<'a> Analyser<'a> {
    pub fn new(conf: AnalyserConfig<'a>) -> Self {
        let mut builder = RuleRegistry::builder(&conf.filter);
        visit_registry(&mut builder);
        let registry = builder.build();

        Self {
            metadata: METADATA.deref(),
            registry,
            options: conf.options,
        }
    }

    pub fn run(&self, ctx: AnalyserContext) -> Vec<RuleDiagnostic> {
        let params = RegistryRuleParams {
            root: ctx.root,
            options: self.options,
        };

        self.registry
            .rules
            .iter()
            .flat_map(|rule| (rule.run)(&params))
            .collect::<Vec<_>>()
    }
}
