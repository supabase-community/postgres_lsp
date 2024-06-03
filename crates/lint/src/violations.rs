use std::str::FromStr;

use crate::RULES;
use serde::{Deserialize, Serialize};
use text_size::TextRange;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq, Deserialize)]
pub enum RuleViolationKind {
    #[serde(rename = "ban-drop-column")]
    BanDropColumn,
}

impl std::fmt::Display for RuleViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_plain::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRuleName {
    val: String,
}

impl std::fmt::Display for UnknownRuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid rule name {}", self.val)
    }
}

impl std::str::FromStr for RuleViolationKind {
    type Err = UnknownRuleName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s).map_err(|_| UnknownRuleName { val: s.to_string() })
    }
}

impl std::convert::TryFrom<&str> for RuleViolationKind {
    type Error = UnknownRuleName;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        RuleViolationKind::from_str(s)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ViolationMessage {
    Note(String),
    Help(String),
}

#[derive(Debug, PartialEq)]
pub struct RuleViolation {
    pub kind: RuleViolationKind,
    pub range: Option<TextRange>,
    pub messages: Vec<ViolationMessage>,
}

impl RuleViolation {
    pub fn new(
        kind: RuleViolationKind,
        range: Option<TextRange>,
        messages: Option<Vec<ViolationMessage>>,
    ) -> Self {
        let messages = messages.unwrap_or_else(|| {
            RULES
                .iter()
                .find(|r| r.name == kind)
                .map_or_else(Vec::new, |x| x.messages.clone())
        });
        Self {
            kind,
            range,
            messages,
        }
    }
}
