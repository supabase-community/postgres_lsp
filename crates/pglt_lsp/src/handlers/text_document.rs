use crate::{
    diagnostics::LspError, documents::Document, session::Session, utils::apply_document_changes,
};
use anyhow::Result;
use pglt_lsp_converters::from_proto::text_range;
use pglt_workspace::workspace::{
    ChangeFileParams, ChangeParams, CloseFileParams, GetFileContentParams, OpenFileParams,
};
use tower_lsp::lsp_types;
use tracing::{error, field};

/// Handler for `textDocument/didOpen` LSP notification
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(
        text_document_uri = display(params.text_document.uri.as_ref()),
        text_document_language_id = display(&params.text_document.language_id),
    )
)]
pub(crate) async fn did_open(
    session: &Session,
    params: lsp_types::DidOpenTextDocumentParams,
) -> Result<()> {
    let url = params.text_document.uri;
    let version = params.text_document.version;
    let content = params.text_document.text;

    let path = session.file_path(&url)?;
    let doc = Document::new(version, &content);

    session.workspace.open_file(OpenFileParams {
        path,
        version,
        content,
    })?;

    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

// Handler for `textDocument/didChange` LSP notification
#[tracing::instrument(level = "debug", skip_all, fields(url = field::display(&params.text_document.uri), version = params.text_document.version), err)]
pub(crate) async fn did_change(
    session: &Session,
    params: lsp_types::DidChangeTextDocumentParams,
) -> Result<(), LspError> {
    let url = params.text_document.uri;
    let version = params.text_document.version;

    let pglsp_path = session.file_path(&url)?;

    let old_doc = session.document(&url)?;
    let old_text = session.workspace.get_file_content(GetFileContentParams {
        path: pglsp_path.clone(),
    })?;

    let start = params
        .content_changes
        .iter()
        .rev()
        .position(|change| change.range.is_none())
        .map_or(0, |idx| params.content_changes.len() - idx - 1);

    let text = apply_document_changes(
        session.position_encoding(),
        old_text,
        &params.content_changes[start..],
    );

    session.workspace.change_file(ChangeFileParams {
        path: pglsp_path,
        version,
        changes: params.content_changes[start..]
            .iter()
            .map(|c| ChangeParams {
                range: c.range.and_then(|r| {
                    text_range(&old_doc.line_index, r, session.position_encoding()).ok()
                }),
                text: c.text.clone(),
            })
            .collect(),
    })?;

    session.insert_document(url.clone(), Document::new(version, &text));

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didClose` LSP notification
#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) async fn did_close(
    session: &Session,
    params: lsp_types::DidCloseTextDocumentParams,
) -> Result<()> {
    let url = params.text_document.uri;
    let pglsp_path = session.file_path(&url)?;

    session
        .workspace
        .close_file(CloseFileParams { path: pglsp_path })?;

    session.remove_document(&url);

    let diagnostics = vec![];
    let version = None;
    session
        .client
        .publish_diagnostics(url, diagnostics, version)
        .await;

    Ok(())
}
