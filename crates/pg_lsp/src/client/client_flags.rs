use tower_lsp::lsp_types::InitializeParams;

/// Contains information about the client's capabilities.
/// This is used to determine which features the server can use.
#[derive(Debug, Clone)]
pub struct ClientFlags {
    /// If `true`, the server can pull configuration from the client.
    pub supports_pull_opts: bool,

    /// If `true`, the client notifies the server when its configuration changes.
    pub supports_dynamic_registration: bool,
}

impl ClientFlags {
    pub(crate) fn from_initialize_request_params(params: &InitializeParams) -> Self {
        let supports_pull_opts = params
            .capabilities
            .workspace
            .as_ref()
            .and_then(|w| w.configuration)
            .unwrap_or(false);

        let supports_dynamic_registration = params
            .capabilities
            .workspace
            .as_ref()
            .and_then(|w| w.did_change_configuration)
            .and_then(|c| c.dynamic_registration)
            .unwrap_or(false);

        Self {
            supports_pull_opts,
            supports_dynamic_registration,
        }
    }
}
