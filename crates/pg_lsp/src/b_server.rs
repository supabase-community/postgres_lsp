use std::sync::Arc;

use notification::ShowMessage;
use pg_commands::CommandType;
use pg_workspace::Workspace;
use tokio::sync::{Mutex, RwLock};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::client::client_flags::ClientFlags;
use crate::db_connection::DbConnection;
use crate::server::options::ClientConfigurationOptions;

struct Server {
    client: Client,
    db: Mutex<Option<DbConnection>>,
    ide: Arc<RwLock<Workspace>>,
    client_capabilities: RwLock<Option<ClientFlags>>,
}

impl Server {
    pub async fn new(client: Client) -> Self {
        let ide = Arc::new(RwLock::new(Workspace::new()));
        Self {
            client,
            db: Mutex::new(None),
            ide,
            client_capabilities: RwLock::new(None),
        }
    }

    /// When the client sends a didChangeConfiguration notification, we need to parse the received JSON.
    fn parse_options_from_client(
        &self,
        mut value: serde_json::Value,
    ) -> Result<ClientConfigurationOptions> {
        let options = match value.get_mut("pglsp") {
            Some(section) => section.take(),
            None => value,
        };

        let options = match serde_json::from_value::<ClientConfigurationOptions>(options) {
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
        };

        Ok(options.unwrap_or_default())
    }

    async fn update_db_connection(
        &self,
        options: ClientConfigurationOptions,
    ) -> anyhow::Result<()> {
        if options.db_connection_string.is_none()
            || self
                .db
                .lock()
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

        let mut current_db = self.db.lock().await;
        let old_db = current_db.replace(db);

        if old_db.is_some() {
            let old_db = old_db.unwrap();
            old_db.close().await;
        }

        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
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

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "Postgres LSP terminated.")
            .await;
        Ok(())
    }


    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        match self.parse_options_from_client(params.settings) {
            Ok(opts) => {
                self.update_db_connection(opts).await;
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Error parsing configuration: {}", e))
                    .await;
            }
        };

    }
}
