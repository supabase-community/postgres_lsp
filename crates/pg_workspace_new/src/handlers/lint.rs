use pg_fs::PgLspPath;

#[derive(Debug)]
pub(crate) struct LintParams<'a> {
    pub(crate) stmt: &'a pg_query_ext::NodeEnum,
    pub(crate) workspace: &'a WorkspaceSettingsHandle<'a>,
    pub(crate) max_diagnostics: u32,
    pub(crate) path: &'a PgLspPath,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) categories: RuleCategories,
    pub(crate) suppression_reason: Option<String>,
}

pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<pg_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u32,
}
