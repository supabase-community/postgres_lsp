use std::{ops::Deref, sync::LazyLock};

use pgt_analyse::{
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
    pub root: &'a pgt_query_ext::NodeEnum,
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

#[cfg(test)]
mod tests {
    use core::slice;

    use pgt_analyse::{AnalyserOptions, AnalysisFilter, RuleFilter};
    use pgt_console::{
        Markup,
        fmt::{Formatter, Termcolor},
        markup,
    };
    use pgt_diagnostics::PrintDiagnostic;
    use termcolor::NoColor;

    use crate::Analyser;

    #[ignore]
    #[test]
    fn debug_test() {
        fn markup_to_string(markup: Markup) -> String {
            let mut buffer = Vec::new();
            let mut write = Termcolor(NoColor::new(&mut buffer));
            let mut fmt = Formatter::new(&mut write);
            fmt.write_markup(markup).unwrap();

            String::from_utf8(buffer).unwrap()
        }

        const SQL: &str = r#"alter table test drop column id;"#;
        let rule_filter = RuleFilter::Rule("safety", "banDropColumn");

        let filter = AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..Default::default()
        };

        let ast = pgt_query_ext::parse(SQL).expect("failed to parse SQL");

        let options = AnalyserOptions::default();

        let analyser = Analyser::new(crate::AnalyserConfig {
            options: &options,
            filter,
        });

        let results = analyser.run(crate::AnalyserContext { root: &ast });

        println!("*******************");
        for result in &results {
            let text = markup_to_string(markup! {
                {PrintDiagnostic::simple(result)}
            });
            eprintln!("{}", text);
        }
        println!("*******************");

        // assert_eq!(results, vec![]);
    }
}
