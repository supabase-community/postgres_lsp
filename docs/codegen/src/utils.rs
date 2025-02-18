use pglt_analyse::{GroupCategory, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata};
use regex::Regex;
use std::collections::BTreeMap;

pub(crate) fn replace_section(
    content: &str,
    section_identifier: &str,
    replacement: &str,
) -> String {
    let pattern = format!(
        r"(\[//\]: # \(BEGIN {}\)\n)(?s).*?(\n\[//\]: # \(END {}\))",
        section_identifier, section_identifier
    );
    let re = Regex::new(&pattern).unwrap();
    re.replace_all(content, format!("${{1}}{}${{2}}", replacement))
        .to_string()
}

#[derive(Default)]
pub(crate) struct LintRulesVisitor {
    /// This is mapped to:
    /// group (e.g. "safety") -> <list of rules>
    /// where <list of rules> is:
    /// <rule name> -> metadata
    pub(crate) groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
}

impl LintRulesVisitor {
    fn push_rule<R>(&mut self)
    where
        R: Rule<Options: Default> + 'static,
    {
        let group = self
            .groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default();

        group.insert(R::METADATA.name, R::METADATA);
    }
}

impl RegistryVisitor for LintRulesVisitor {
    fn record_category<C: GroupCategory>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule + 'static,
    {
        self.push_rule::<R>()
    }
}
