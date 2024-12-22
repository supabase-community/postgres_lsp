//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyser::linter::*;
use pg_analyse::{AnalyzerRules, MetadataRegistry};
pub fn push_to_analyser_rules(
    rules: &Rules,
    metadata: &MetadataRegistry,
    analyser_rules: &mut AnalyzerRules,
) {
    if let Some(rules) = rules.safety.as_ref() {
        for rule_name in Safety::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("safety", rule_name) {
                    analyser_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
}
