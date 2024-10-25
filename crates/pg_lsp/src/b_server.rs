use notification::ShowMessage;
use pg_commands::CommandType;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::client::client_flags::ClientFlags;
use crate::server::options::ClientConfigurationOptions;
use crate::utils::file_path;
use crate::utils::normalize_uri;
use crate::workspace_handler::WorkspaceHandler;

struct Server {
    client: Client,
    workspace_handler: WorkspaceHandler,
    client_capabilities: RwLock<Option<ClientFlags>>,
}

impl Server {
    pub async fn new(client: Client) -> Self {
        Self {
            client,
            workspace_handler: WorkspaceHandler::new(),
            client_capabilities: RwLock::new(None),
        }
    }

    /// When the client sends a didChangeConfiguration notification, we need to parse the received JSON.
    fn parse_options_from_client(
        &self,
        mut value: serde_json::Value,
    ) -> Option<ClientConfigurationOptions> {
        let options = match value.get_mut("pglsp") {
            Some(section) => section.take(),
            None => value,
        };

        match serde_json::from_value::<ClientConfigurationOptions>(options) {
            Ok(new_options) => Some(new_options),
            Err(why) => {
                let message = format!(
                    "The texlab configuration is invalid; using the default settings instead.\nDetails: {why}"
                );
                let typ = MessageType::WARNING;
                self.client
                    .send_notification::<ShowMessage>(ShowMessageParams { message, typ });
                None
            }
        }
    }

    async fn request_opts_from_client(&self) -> Option<ClientConfigurationOptions> {
        let params = ConfigurationParams {
            items: vec![ConfigurationItem {
                section: Some("pglsp".to_string()),
                scope_uri: None,
            }],
        };

        match self
            .client
            .send_request::<request::WorkspaceConfiguration>(params)
            .await
        {
            Ok(json) => {
                // The client reponse fits the requested `ConfigurationParams.items`,
                // so the first value is what we're looking for.
                let relevant = json
                    .into_iter()
                    .next()
                    .expect("workspace/configuration request did not yield expected response.");

                let opts = self.parse_options_from_client(relevant);

                opts
            }
            Err(why) => {
                let message = format!(
                    "Unable to pull client options via workspace/configuration request: {}",
                    why
                );
                println!("{}", message);
                self.client.log_message(MessageType::ERROR, message);
                None
            }
        }
    }

    async fn publish_diagnostics(&self, mut uri: Url) -> anyhow::Result<()> {
        normalize_uri(&mut uri);

        let diagnostics = self
            .workspace_handler
            .get_diagnostics(file_path(&uri))
            .await;

        self.client
            .send_notification::<ShowMessage>(ShowMessageParams {
                typ: MessageType::INFO,
                message: format!("diagnostics {}", diagnostics.len()),
            })
            .await;

        let params = PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        };

        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        let flags = ClientFlags::from_initialize_request_params(&params);
        self.client_capabilities.blocking_write().replace(flags);

        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: CommandType::ALL
                        .iter()
                        .map(|c| c.id().to_string())
                        .collect(),
                    ..Default::default()
                }),
                inlay_hint_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Postgres LSP Connected!")
            .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        self.client
            .log_message(MessageType::INFO, "Postgres LSP terminated.")
            .await;
        Ok(())
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let capabilities = self.client_capabilities.read().await;

        if capabilities.as_ref().unwrap().supports_pull_opts {
            let opts = self.request_opts_from_client().await;
            if opts
                .as_ref()
                .is_some_and(|o| o.db_connection_string.is_some())
            {
                let conn_str = opts.unwrap().db_connection_string.unwrap();
                self.workspace_handler.change_db(conn_str).await;
                return;
            }
        }

        let opts = self.parse_options_from_client(params.settings);

        if opts
            .as_ref()
            .is_some_and(|o| o.db_connection_string.is_some())
        {
            let conn_str = opts.unwrap().db_connection_string.unwrap();
            self.workspace_handler.change_db(conn_str).await;
        }
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> jsonrpc::Result<Option<serde_json::Value>> {
        match CommandType::from_id(params.command.replace("pglsp.", "").as_str()) {
            Some(CommandType::ExecuteStatement) => {
                if params.arguments.is_empty() {
                    return jsonrpc::Result::Err(jsonrpc::Error::invalid_request());
                }

                let params = params.arguments.into_iter().next().unwrap();
                let stmt = serde_json::from_value(params)
                    .map_err(|_| jsonrpc::Error::invalid_request())?;

                match self.workspace_handler.run_stmt(stmt).await {
                    Ok(rows_affected) => {
                        self.client
                            .send_notification::<ShowMessage>(ShowMessageParams {
                                typ: MessageType::INFO,
                                message: format!("Success! Affected rows: {}", rows_affected),
                            })
                            .await;
                    }
                    Err(why) => {
                        self.client
                            .send_notification::<ShowMessage>(ShowMessageParams {
                                typ: MessageType::ERROR,
                                message: format!("Error! Statement exectuion failed: {}", why),
                            })
                            .await;
                    }
                };
            }
            None => {
                self.client
                    .show_message(
                        MessageType::ERROR,
                        format!("Unknown command: {}", params.command),
                    )
                    .await;
            }
        };

        Ok(None)
    }
}
