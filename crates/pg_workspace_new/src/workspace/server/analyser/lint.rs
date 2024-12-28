use pg_analyse::{AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, RuleCategories, RuleFilter};
use pg_configuration::RuleSelector;
use pg_diagnostics::serde::Diagnostic;

use crate::{configuration::to_analyzer_rules, settings::SettingsHandle};

use super::AnalyzerVisitorBuilder;

#[derive(Debug)]
pub(crate) struct LinterParams<'a> {
    pub(crate) settings: &'a SettingsHandle<'a>,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) categories: RuleCategories,
}

pub(crate) struct LinterResults {
    pub(crate) diagnostics: Vec<pg_diagnostics::serde::Diagnostic>,
}

pub(crate) struct Linter<'a> {
    categories: RuleCategories,
    options: AnalyzerOptions,
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
}

impl<'a> Linter<'a> {
    pub fn new(params: LinterParams) -> Self {
        let (enabled_rules, disabled_rules) = AnalyzerVisitorBuilder::new(params.settings.as_ref())
            .with_linter_rules(&params.only, &params.skip)
            .finish();

        let configuration = AnalyzerConfiguration {
            rules: to_analyzer_rules(params.settings.as_ref()),
        };

        let options = AnalyzerOptions { configuration };

        Self { options, enabled_rules, disabled_rules, categories: params.categories }
    }

    pub fn run(&self, stmt: &pg_query_ext::NodeEnum) -> LinterResults {
        let filter = AnalysisFilter {
            categories: self.categories,
            enabled_rules: Some(self.enabled_rules.as_slice()),
            disabled_rules: &self.disabled_rules,
        };


        let diagnostics = pg_linter::lint(stmt, filter, &self.options);

        LinterResults {
            diagnostics: diagnostics
                .into_iter()
                .map(Diagnostic::new)
                .collect(),
        }
    }
}

