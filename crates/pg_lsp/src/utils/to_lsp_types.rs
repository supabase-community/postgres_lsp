use tower_lsp::lsp_types;

pub fn to_completion_kind(
    kind: pg_completions::CompletionItemKind,
) -> lsp_types::CompletionItemKind {
    match kind {
        pg_completions::CompletionItemKind::Table => lsp_types::CompletionItemKind::CLASS,
        pg_completions::CompletionItemKind::Function => lsp_types::CompletionItemKind::FUNCTION,
        pg_completions::CompletionItemKind::Column => lsp_types::CompletionItemKind::FIELD,
    }
}
