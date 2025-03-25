use crate::session::Session;
use anyhow::Result;
use pgt_text_size::{TextRange, TextSize};
use tower_lsp::lsp_types::{self, CodeAction, CodeActionDisabled, CodeActionOrCommand, Command};

use pgt_workspace::code_actions::{
    CodeActionKind, CodeActionsParams, CommandAction, CommandActionCategory,
};

use super::helper;

pub fn get_actions(
    session: &Session,
    params: lsp_types::CodeActionParams,
) -> Result<lsp_types::CodeActionResponse> {
    let url = params.text_document.uri;
    let path = session.file_path(&url)?;

    let cursor_position = helper::get_cursor_position(session, url, params.range.start)?;

    let workspace_actions = session.workspace.pull_code_actions(CodeActionsParams {
        path,
        cursor_position,
        only: vec![],
        skip: vec![],
    })?;

    let actions: Vec<CodeAction> = workspace_actions
        .actions
        .into_iter()
        .filter_map(|action| match action.kind {
            CodeActionKind::Command(command) => {
                let title = match &command.category {
                    CommandActionCategory::ExecuteStatement(stmt) => {
                        format!(
                            "Execute Statement: {}...",
                            stmt.chars().take(50).collect::<String>()
                        )
                    }
                };

                return Some(CodeAction {
                    title: title.clone(),
                    command: Some(Command {
                        title: title.clone(),
                        command: command.category.into(),
                        arguments: None,
                    }),
                    disabled: action
                        .disabled_reason
                        .map(|reason| CodeActionDisabled { reason }),
                    ..Default::default()
                });
            }
            _ => todo!(),
        })
        .collect();

    Ok(actions
        .into_iter()
        .map(|ac| CodeActionOrCommand::CodeAction(ac))
        .collect())
}
