use std::{ops::Deref, sync::LazyLock};

use pg_analyse::{AnalysisFilter, AnalyzerOptions, MetadataRegistry, RuleDiagnostic, RuleRegistry};
use registry::visit_registry;

mod lint;
pub mod options;
mod registry;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

pub fn analyse<'a, B>(
    root: &pg_query_ext::NodeEnum,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
) -> Vec<RuleDiagnostic>
where
    B: 'a,
{
    let mut builder = RuleRegistry::builder(&filter);
    visit_registry(&mut builder);
    let registry = builder.build();

    let analyser = pg_analyse::Analyzer::new(METADATA.deref());

    analyser.run(pg_analyse::AnalyzerContext {
        root: root.clone(),
        options,
        registry,
    })
}
