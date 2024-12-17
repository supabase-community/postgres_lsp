use std::fmt::{Display, Formatter, Debug};

use text_size::TextRange;

use crate::{categories::RuleCategories, rule::{GroupCategory, Rule, RuleGroup}, RuleFilter};

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Debug, Default, Clone, Copy)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules matching these names to emit signals
    /// If `enabled_rules` is set to `None`, then all rules are enabled.
    pub enabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Do not allow rules matching these names to emit signals
    pub disabled_rules: &'a [RuleFilter<'a>],
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

impl<'analysis> AnalysisFilter<'analysis> {
    /// It creates a new filter with the set of [enabled rules](RuleFilter) passed as argument
    pub fn from_enabled_rules(enabled_rules: &'analysis [RuleFilter<'analysis>]) -> Self {
        Self {
            enabled_rules: Some(enabled_rules),
            ..AnalysisFilter::default()
        }
    }

    /// Return `true` if the category `C` matches this filter
    pub fn match_category<C: GroupCategory>(&self) -> bool {
        self.categories.contains(C::CATEGORY)
    }

    /// Return `true` if the group `G` matches this filter
    pub fn match_group<G: RuleGroup>(&self) -> bool {
        self.match_category::<G::Category>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_group::<G>())
            })
            && !self
                .disabled_rules
                .iter()
                .any(|filter| matches!(filter, RuleFilter::Group(_)) && filter.match_group::<G>())
    }

    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R: Rule>(&self) -> bool {
        self.match_category::<<R::Group as RuleGroup>::Category>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_rule::<R>())
            })
            && !self
                .disabled_rules
                .iter()
                .any(|filter| filter.match_rule::<R>())
    }
}



impl<'a> RuleFilter<'a> {
    // Returns the group name of this filter.
    pub fn group(self) -> &'a str {
        match self {
            RuleFilter::Group(group) => group,
            RuleFilter::Rule(group, _) => group,
        }
    }
    /// Return `true` if the group `G` matches this filter
    pub fn match_group<G: RuleGroup>(self) -> bool {
        match self {
            RuleFilter::Group(group) => group == G::NAME,
            RuleFilter::Rule(group, _) => group == G::NAME,
        }
    }

    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R>(self) -> bool
    where
        R: Rule,
    {
        match self {
            RuleFilter::Group(group) => group == <R::Group as RuleGroup>::NAME,
            RuleFilter::Rule(group, rule) => {
                group == <R::Group as RuleGroup>::NAME && rule == R::METADATA.name
            }
        }
    }
}

impl<'a> Debug for RuleFilter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for RuleFilter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleFilter::Group(group) => {
                write!(f, "{group}")
            }
            RuleFilter::Rule(group, rule) => {
                write!(f, "{group}/{rule}")
            }
        }
    }
}

impl<'a> pg_console::fmt::Display for RuleFilter<'a> {
    fn fmt(&self, fmt: &mut pg_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            RuleFilter::Group(group) => {
                write!(fmt, "{group}")
            }
            RuleFilter::Rule(group, rule) => {
                write!(fmt, "{group}/{rule}")
            }
        }
    }
}

