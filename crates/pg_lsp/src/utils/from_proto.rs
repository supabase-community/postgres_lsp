use super::line_index_ext::LineIndexExt;
use pg_base_db::{Change, Document};

pub fn content_changes(
    document: &Document,
    changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> Vec<Change> {
    changes
        .iter()
        .map(|change| Change {
            range: change
                .range
                .map(|range| document.line_index.offset_lsp_range(range).unwrap()),
            text: change.text.clone(),
        })
        .collect()
}
