pub mod linter;

pub use crate::analyser::linter::*;
use biome_deserialize::Merge;
use biome_deserialize_macros::Deserializable;
use pgt_analyse::RuleFilter;
use pgt_analyse::options::RuleOptions;
use pgt_diagnostics::Severity;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration<T: Default> {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithOptions<T>),
}
impl<T: Default> RuleConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RulePlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RulePlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => Some(RuleOptions::new(options.options.clone())),
        }
    }
}
impl<T: Default> Default for RuleConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleFixConfiguration<T: Default> {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithFixOptions<T>),
}
impl<T: Default> Default for RuleFixConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}
impl<T: Default> RuleFixConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RulePlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RulePlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleFixConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleFixConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => Some(RuleOptions::new(options.options.clone())),
        }
    }
}
impl<T: Default> From<&RuleConfiguration<T>> for Severity {
    fn from(conf: &RuleConfiguration<T>) -> Self {
        match conf {
            RuleConfiguration::Plain(p) => (*p).into(),
            RuleConfiguration::WithOptions(conf) => {
                let level = &conf.level;
                (*level).into()
            }
        }
    }
}
impl From<RulePlainConfiguration> for Severity {
    fn from(conf: RulePlainConfiguration) -> Self {
        match conf {
            RulePlainConfiguration::Warn => Severity::Warning,
            RulePlainConfiguration::Error => Severity::Error,
            RulePlainConfiguration::Info => Severity::Information,
            RulePlainConfiguration::Off => {
                unreachable!("the rule is turned off, it should not step in here")
            }
        }
    }
}
impl From<RuleAssistPlainConfiguration> for Severity {
    fn from(conf: RuleAssistPlainConfiguration) -> Self {
        match conf {
            RuleAssistPlainConfiguration::On => Severity::Hint,
            RuleAssistPlainConfiguration::Off => {
                unreachable!("the rule is turned off, it should not step in here")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    #[default]
    Warn,
    Error,
    Info,
    Off,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleAssistConfiguration<T: Default> {
    Plain(RuleAssistPlainConfiguration),
    WithOptions(RuleAssistWithOptions<T>),
}
impl<T: Default> RuleAssistConfiguration<T> {
    pub fn is_disabled(&self) -> bool {
        matches!(self.level(), RuleAssistPlainConfiguration::Off)
    }
    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
    pub fn level(&self) -> RuleAssistPlainConfiguration {
        match self {
            Self::Plain(plain) => *plain,
            Self::WithOptions(options) => options.level,
        }
    }
    pub fn set_level(&mut self, level: RuleAssistPlainConfiguration) {
        match self {
            Self::Plain(plain) => *plain = level,
            Self::WithOptions(options) => options.level = level,
        }
    }
}
// Rule configuration has a custom [Merge] implementation so that overriding the
// severity doesn't override the options.
impl<T: Clone + Default> Merge for RuleAssistConfiguration<T> {
    fn merge_with(&mut self, other: Self) {
        match self {
            Self::Plain(_) => *self = other,
            Self::WithOptions(this) => match other {
                Self::Plain(level) => {
                    this.level = level;
                }
                Self::WithOptions(other) => {
                    this.merge_with(other);
                }
            },
        }
    }
}
impl<T: Clone + Default + 'static> RuleAssistConfiguration<T> {
    pub fn get_options(&self) -> Option<RuleOptions> {
        match self {
            Self::Plain(_) => None,
            Self::WithOptions(options) => Some(RuleOptions::new(options.options.clone())),
        }
    }
}
impl<T: Default> Default for RuleAssistConfiguration<T> {
    fn default() -> Self {
        Self::Plain(RuleAssistPlainConfiguration::Off)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RuleAssistPlainConfiguration {
    #[default]
    On,
    Off,
}
impl RuleAssistPlainConfiguration {
    pub const fn is_enabled(&self) -> bool {
        matches!(self, Self::On)
    }

    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Off)
    }
}
impl Merge for RuleAssistPlainConfiguration {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleAssistWithOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RuleAssistPlainConfiguration,
    /// Rule's options
    pub options: T,
}
impl<T: Default> Merge for RuleAssistWithOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.options = other.options;
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RulePlainConfiguration,
    /// Rule's options
    pub options: T,
}
impl<T: Default> Merge for RuleWithOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.options = other.options;
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithFixOptions<T: Default> {
    /// The severity of the emitted diagnostics by the rule
    pub level: RulePlainConfiguration,
    /// Rule's options
    pub options: T,
}

impl<T: Default> Merge for RuleWithFixOptions<T> {
    fn merge_with(&mut self, other: Self) {
        self.level = other.level;
        self.options = other.options;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RuleSelector {
    Group(linter::RuleGroup),
    Rule(linter::RuleGroup, &'static str),
}

impl From<RuleSelector> for RuleFilter<'static> {
    fn from(value: RuleSelector) -> Self {
        match value {
            RuleSelector::Group(group) => RuleFilter::Group(group.as_str()),
            RuleSelector::Rule(group, name) => RuleFilter::Rule(group.as_str(), name),
        }
    }
}

impl<'a> From<&'a RuleSelector> for RuleFilter<'static> {
    fn from(value: &'a RuleSelector) -> Self {
        match value {
            RuleSelector::Group(group) => RuleFilter::Group(group.as_str()),
            RuleSelector::Rule(group, name) => RuleFilter::Rule(group.as_str(), name),
        }
    }
}

impl FromStr for RuleSelector {
    type Err = &'static str;
    fn from_str(selector: &str) -> Result<Self, Self::Err> {
        let selector = selector.strip_prefix("lint/").unwrap_or(selector);
        if let Some((group_name, rule_name)) = selector.split_once('/') {
            let group = linter::RuleGroup::from_str(group_name)?;
            if let Some(rule_name) = Rules::has_rule(group, rule_name) {
                Ok(RuleSelector::Rule(group, rule_name))
            } else {
                Err("This rule doesn't exist.")
            }
        } else {
            match linter::RuleGroup::from_str(selector) {
                Ok(group) => Ok(RuleSelector::Group(group)),
                Err(_) => Err(
                    "This group doesn't exist. Use the syntax `<group>/<rule>` to specify a rule.",
                ),
            }
        }
    }
}

impl serde::Serialize for RuleSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RuleSelector::Group(group) => serializer.serialize_str(group.as_str()),
            RuleSelector::Rule(group, rule_name) => {
                let group_name = group.as_str();
                serializer.serialize_str(&format!("{group_name}/{rule_name}"))
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for RuleSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = RuleSelector;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("<group>/<ruyle_name>")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match RuleSelector::from_str(v) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(serde::de::Error::custom(error)),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for RuleSelector {
    fn schema_name() -> String {
        "RuleCode".to_string()
    }
    fn json_schema(r#gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(r#gen)
    }
}
