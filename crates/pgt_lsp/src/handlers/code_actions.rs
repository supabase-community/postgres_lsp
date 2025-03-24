use crate::session::Session;
use tower_lsp::lsp_types::{self, CodeAction, CodeActionDisabled, CodeActionOrCommand, Command};

use pgt_workspace::code_actions::{CodeActionKind, CommandAction, CommandActionCategory};

pub fn get_actions(
    session: &Session,
    params: lsp_types::CodeActionParams,
) -> Result<lsp_types::CodeActionResponse> {
    let url = params.text_document.uri;
    let path = session.file_path(&url)?;

    let workspace_actions = session.workspace.pull_code_actions(CodeActionsParams {
        path,
        range: params.range,
        only: vec![],
        skip: vec![],
    })?;

    let actions: Vec<CodeAction> = workspace_actions
        .actions
        .into_iter()
        .filter_map(|action| match action.kind {
            CodeActionKind::Command(command) => {
                let title = match command.category {
                    CommandActionCategory::ExecuteStatement(stmt) => {
                        format!("Execute Statement: {}...", stmt[..50])
                    }
                };

                return Some(CodeAction {
                    title,
                    command: Some(Command {
                        title,
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
