use crate::session::Session;
use anyhow::Result;
use pg_workspace_new::workspace::CompletionParams;
use text_size::TextSize;
use tower_lsp::lsp_types::{self, CompletionItem, CompletionItemLabelDetails};

pub fn get_completions(
    session: &Session,
    params: lsp_types::CompletionParams,
) -> Result<lsp_types::CompletionResponse> {
    let pos = params.text_document_position.position;
    let url = params.text_document_position.text_document.uri;

    let path = session.file_path(&url)?;

    let completion_result = session.workspace.get_completions(CompletionParams {
        path,
        position: TextSize::from(pos.character),
    })?;

    let items: Vec<CompletionItem> = completion_result
        .into_iter()
        .map(|i| CompletionItem {
            label: i.label,
            label_details: Some(CompletionItemLabelDetails {
                description: Some(i.description),
                detail: None,
            }),
            preselect: Some(i.preselected),
            kind: Some(to_lsp_types_completion_item_kind(i.kind)),
            ..CompletionItem::default()
        })
        .collect();

    Ok(lsp_types::CompletionResponse::Array(items))
}

fn to_lsp_types_completion_item_kind(
    pg_comp_kind: pg_completions::CompletionItemKind,
) -> lsp_types::CompletionItemKind {
    match pg_comp_kind {
        pg_completions::CompletionItemKind::Function
        | pg_completions::CompletionItemKind::Table => lsp_types::CompletionItemKind::CLASS,
    }
}
