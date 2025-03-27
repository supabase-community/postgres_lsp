use crate::session::Session;
use pgt_text_size::TextSize;
use tower_lsp::lsp_types;

pub fn get_cursor_position(
    session: &Session,
    url: &lsp_types::Url,
    position: lsp_types::Position,
) -> anyhow::Result<TextSize> {
    let client_capabilities = session
        .client_capabilities()
        .expect("Client capabilities not established for current session.");

    let line_index = session
        .document(url)
        .map(|doc| doc.line_index)
        .map_err(|_| anyhow::anyhow!("Document not found."))?;

    let cursor_pos = pgt_lsp_converters::from_proto::offset(
        &line_index,
        position,
        pgt_lsp_converters::negotiated_encoding(client_capabilities),
    )?;

    Ok(cursor_pos)
}
