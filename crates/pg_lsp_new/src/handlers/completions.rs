use crate::session::Session;
use anyhow::Result;
use pg_workspace_new::workspace;
use tower_lsp::lsp_types::{self, CompletionItem, CompletionItemLabelDetails};

#[tracing::instrument(level = "trace", skip_all)]
pub fn get_completions(
    session: &Session,
    params: lsp_types::CompletionParams,
) -> Result<lsp_types::CompletionResponse> {
    let url = params.text_document_position.text_document.uri;
    let path = session.file_path(&url)?;

    let client_capabilities = session
        .client_capabilities()
        .expect("Client capabilities not established for current session.");

    let line_index = session
        .document(&url)
        .map(|doc| doc.line_index)
        .map_err(|_| anyhow::anyhow!("Document not found."))?;

    let offset = pg_lsp_converters::from_proto::offset(
        &line_index,
        params.text_document_position.position,
        pg_lsp_converters::negotiated_encoding(client_capabilities),
    )?;

    let completion_result = session
        .workspace
        .get_completions(workspace::CompletionParams {
            path,
            position: offset,
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
        pg_completions::CompletionItemKind::Column => lsp_types::CompletionItemKind::FIELD,
    }
}
