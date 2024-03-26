use super::line_index_ext::LineIndexExt;
use base_db::{Document, DocumentChange};

pub fn content_changes(
    document: &Document,
    changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> Vec<DocumentChange> {
    changes
        .iter()
        .map(|change| DocumentChange {
            range: change
                .range
                .map(|range| document.line_index.offset_lsp_range(range).unwrap()),
            text: change.text.clone(),
        })
        .collect()
}
