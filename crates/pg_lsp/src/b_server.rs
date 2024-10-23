use std::sync::Arc;

use notification::ShowMessage;
use pg_commands::CommandType;
use pg_workspace::Workspace;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::client::client_flags::ClientFlags;
use crate::db_connection::DbConnection;
use crate::server::options::ClientConfigurationOptions;

struct Server {
    client: Client,
    db: RwLock<Option<DbConnection>>,
    ide: Arc<RwLock<Workspace>>,
    client_capabilities: RwLock<Option<ClientFlags>>,
}

impl Server {
    pub async fn new(client: Client) -> Self {
        let ide = Arc::new(RwLock::new(Workspace::new()));
        Self {
            client,
            db: RwLock::new(None),
            ide,
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

    /// `update_db_connection` will update `Self`'s database connection.
    /// If the passed-in connection string is the same that we're already connected to, it's a noop.
    /// Otherwise, it'll first open a new connection, replace `Self`'s connection, and then close
    /// the old one.
    async fn update_db_connection(
        &self,
        options: ClientConfigurationOptions,
    ) -> anyhow::Result<()> {
        if options.db_connection_string.is_none()
            || self
                .db
                .read()
                .await
                .as_ref()
                // if the connection is already connected to the same database, do nothing
                .is_some_and(|c| c.connected_to(options.db_connection_string.as_ref().unwrap()))
        {
            return Ok(());
        }

        let connection_string = options.db_connection_string.unwrap();

        let mut db = DbConnection::new(connection_string).await?;

        let ide = self.ide.clone();
        db.listen_for_schema_updates(move |schema| {
            let _guard = ide.blocking_write().set_schema_cache(schema);
        });

        let mut current_db = self.db.blocking_write();
        let old_db = current_db.replace(db);

        if old_db.is_some() {
            let old_db = old_db.unwrap();
            old_db.close().await;
        }

        Ok(())
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
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
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

    async fn shutdown(&self) -> anyhow::Result<()> {
        self.client
            .log_message(MessageType::INFO, "Postgres LSP terminated.")
            .await;
        Ok(())
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let capabilities = self.client_capabilities.read().await;

        if capabilities.as_ref().unwrap().supports_pull_opts {
            let opts = self.request_opts_from_client().await;
            if opts.is_some() {
                self.update_db_connection(opts.unwrap()).await;
                return;
            }
        }

        let opts = self.parse_options_from_client(params.settings);

        if opts.is_some() {
            self.update_db_connection(opts.unwrap()).await;
        }
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> tower_lsp::jsonrpc::Result<Option<serde_json::Value>> {
        match CommandType::from_id(params.command.replace("pglsp.", "").as_str()) {
            Some(CommandType::ExecuteStatement) => {
                if params.arguments.is_empty() {
                    return tower_lsp::jsonrpc::Result::Err(Error::new("No arguments provided!"));
                }

                let stmt = params
                    .arguments
                    .into_iter()
                    .next()
                    .map(|v| serde_json::from_value(v))
                    .unwrap()?;

                let conn = self.db.read().await;
                match conn
                    .as_ref()
                    .expect("No connection to the database.")
                    .run_stmt(stmt)
                    .await
                {
                    Ok(pg_result) => {
                        self.client
                            .send_notification::<ShowMessage>(ShowMessageParams {
                                typ: MessageType::INFO,
                                message: format!(
                                    "Success! Affected rows: {}",
                                    pg_result.rows_affected()
                                ),
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
