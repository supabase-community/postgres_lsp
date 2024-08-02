use crate::client::client_flags::ClientFlags;

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

pub fn client_flags(capabilities: lsp_types::ClientCapabilities) -> ClientFlags {
    let configuration_pull = capabilities
        .workspace
        .as_ref()
        .and_then(|cap| cap.configuration)
        .unwrap_or(false);

    let configuration_push = capabilities
        .workspace
        .as_ref()
        .and_then(|cap| cap.did_change_configuration)
        .and_then(|cap| cap.dynamic_registration)
        .unwrap_or(false);

    ClientFlags {
        configuration_pull,
        configuration_push,
    }
}
