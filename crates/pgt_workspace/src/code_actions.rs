use pgt_configuration::RuleSelector;
use pgt_fs::PgTPath;
use pgt_text_size::TextRange;

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
    pub kind: CodeActionKind,
    pub disabled_reason: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum CodeActionKind {
    Edit(EditAction),
    Command(CommandAction),
    EditAndCommand(EditAction, CommandAction),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct EditAction {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CommandAction {
    pub category: CommandActionCategory,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum CommandActionCategory {
    ExecuteStatement(String),
}

impl Into<String> for CommandActionCategory {
    fn into(self) -> String {
        match self {
            Self::ExecuteStatement(_) => "pgt.executeStatement".into(),
        }
    }
}
