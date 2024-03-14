use super::line_index_ext::LineIndexExt;
use base_db::{FileChange, SourceFile};

pub fn content_changes(
    source_file: &SourceFile,
    changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> Vec<FileChange> {
    changes
        .iter()
        .map(|change| FileChange {
            range: change
                .range
                .map(|range| source_file.line_index.offset_lsp_range(range).unwrap()),
            text: change.text.clone(),
        })
        .collect()
}
