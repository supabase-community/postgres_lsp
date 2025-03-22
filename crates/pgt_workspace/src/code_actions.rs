use pgt_configuration::RuleSelector;
use pgt_fs::PgTPath;
use pgt_text_size::TextRange;

use crate::Workspace;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeActionsParams {
    pub path: PgTPath,
    pub range: Option<TextRange>,
    pub only: Vec<RuleSelector>,
    pub skip: Vec<RuleSelector>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeActionsResult {
    pub actions: Vec<CodeAction>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    pub category: CodeActionCategory,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CodeActionCategory {
    ExecuteCommand(String),
}

impl CodeActionCategory {
    fn perform(self, workspace: &dyn Workspace) -> Result<(), String> {
        match self {
            Self::ExecuteCommand(stmt) => {}
        }
        Ok(())
    }
}
