mod dispatch;
pub mod options;

use async_std::task::{self};
use base_db::{Change, DocumentChange};
use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::IDE;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
        DidSaveTextDocument, LogMessage, Notification as _, ShowMessage,
    },
    request::{RegisterCapability, WorkspaceConfiguration},
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializeParams, InitializeResult, LogMessageParams, Registration,
    RegistrationParams, SaveOptions, ServerCapabilities, ServerInfo, ShowMessageParams,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions,
};
use schema_cache::SchemaCache;
use std::sync::Arc;
use threadpool::ThreadPool;
use tracing::{event, instrument, Level};

use crate::{
    client::{client_flags::ClientFlags, LspClient},
    utils::{file_path, from_proto, normalize_uri},
};

use self::options::Options;
use sqlx::postgres::{PgListener, PgPool};

#[derive(Debug)]
enum InternalMessage {
    Diagnostics,
    SetOptions(Options),
    RefreshSchemaCache,
    SetSchemaCache(SchemaCache),
}

#[derive(Debug)]
struct DbConnection {
    pub pool: PgPool,
    connection_string: String,
}

impl DbConnection {
    pub async fn new(connection_string: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(connection_string).await?;
        Ok(Self {
            pool,
            connection_string: connection_string.to_owned(),
        })
    }
}

pub struct Server {
    connection: Arc<Connection>,
    client: LspClient,
    internal_tx: Sender<InternalMessage>,
    internal_rx: Receiver<InternalMessage>,
    pool: ThreadPool,
    client_flags: Arc<ClientFlags>,
    ide: IDE,
    db_conn: Option<DbConnection>,
}

impl Server {
    pub fn init(connection: Connection) -> anyhow::Result<()> {
        let client = LspClient::new(connection.sender.clone());

        let (internal_tx, internal_rx) = unbounded();

        let (id, params) = connection.initialize_start()?;
        let params: InitializeParams = serde_json::from_value(params)?;

        let result = InitializeResult {
            capabilities: Self::capabilities(),
            server_info: Some(ServerInfo {
                name: "Postgres LSP".to_owned(),
                version: Some(env!("CARGO_PKG_VERSION").to_owned()),
            }),
        };

        connection.initialize_finish(id, serde_json::to_value(result)?)?;

        let client_flags = Arc::new(from_proto::client_flags(
            params.capabilities,
            params.client_info,
        ));

        let server = Self {
            connection: Arc::new(connection),
            internal_rx,
            internal_tx,
            client,
            pool: threadpool::Builder::new().build(),
            client_flags,
            db_conn: None,
            ide: IDE::new(),
        };

        server.run()?;
        Ok(())
    }

    fn start_listening(&self) {
        if self.db_conn.is_none() {
            return;
        }

        let pool = self.db_conn.as_ref().unwrap().pool.clone();
        let tx = self.internal_tx.clone();

        task::spawn(async move {
            let mut listener = PgListener::connect_with(&pool).await.unwrap();
            listener.listen_all(["pglsp", "pgrst"]).await.unwrap();
            loop {
                match listener.recv().await {
                    Ok(notification) => {
                        if notification.payload().to_string() == "reload schema" {
                            tx.send(InternalMessage::RefreshSchemaCache).unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("Listener error: {}", e);
                        break;
                    }
                }
            }
        });
    }

    #[instrument(skip(self), name = "pglsp/update_db_connection")]
    async fn update_db_connection(&mut self, connection_string: Option<String>) {
        if connection_string == self.db_conn.as_ref().map(|c| c.connection_string.clone()) {
            return;
        }
        if let Some(conn) = self.db_conn.take() {
            conn.pool.close().await;
        }

        if connection_string.is_none() {
            return;
        }

        let new_conn = DbConnection::new(connection_string.unwrap().as_str()).await;

        if new_conn.is_err() {
            let err = new_conn.unwrap_err();
            event!(Level::ERROR, "Failed to connect to database: {:?}", err);
            return;
        }

        self.db_conn = Some(new_conn.unwrap());

        self.client
            .send_notification::<ShowMessage>(ShowMessageParams {
                typ: lsp_types::MessageType::INFO,
                message: "Connection to database established".to_string(),
            })
            .unwrap();
    }

    #[instrument(skip(self), name = "pglsp/update_options")]
    fn update_options(&mut self, options: Options) {
        async_std::task::block_on(self.update_db_connection(options.db_connection_string));
        self.start_listening();
    }

    fn capabilities() -> ServerCapabilities {
        ServerCapabilities {
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
            ..ServerCapabilities::default()
        }
    }

    fn publish_diagnostics(&self) -> anyhow::Result<()> {
        // let workspace = self.workspace.read();
        //
        // for (uri, diagnostics) in self.diagnostic_manager.get(&workspace) {
        //     let Some(document) = workspace.lookup(&uri) else {
        //         continue;
        //     };
        //
        //     let diagnostics = diagnostics
        //         .into_iter()
        //         .filter_map(|diagnostic| to_proto::diagnostic(&workspace, document, &diagnostic))
        //         .collect();
        //
        //     let version = None;
        //     let params = PublishDiagnosticsParams {
        //         uri,
        //         diagnostics,
        //         version,
        //     };
        //
        //     self.client
        //         .send_notification::<PublishDiagnostics>(params)?;
        // }

        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/did_open")]
    fn did_open(&self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        event!(Level::INFO, "did_open");

        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        let path = file_path(&uri);

        self.ide.apply_change(
            path,
            DocumentChange::new(
                params.text_document.version,
                vec![Change {
                    range: None,
                    text: params.text_document.text,
                }],
            ),
        );

        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/did_change")]
    fn did_change(&self, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        let path = file_path(&uri);

        let document = self.ide.documents.get(&path);

        if document.is_none() {
            return Ok(());
        }

        let changes = from_proto::content_changes(&document.unwrap(), params.content_changes);

        self.ide.apply_change(
            path,
            DocumentChange::new(params.text_document.version, changes),
        );

        // TODO: update diagnostics

        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/did_save")]
    fn did_save(&self, params: DidSaveTextDocumentParams) -> anyhow::Result<()> {
        // on save we want to run static analysis and ultimately publish diagnostics

        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/did_close")]
    fn did_close(&self, params: DidCloseTextDocumentParams) -> anyhow::Result<()> {
        // this just means that the document is no longer open in the client
        // if we would listen to fs events, we would use this to overwrite the files owner to be
        // "server" instead of "client". for now we will ignore this notification since we dont
        // need to watch files that are not open.

        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);
        let path = file_path(&uri);

        self.ide.documents.remove(&path);
        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/refresh_schema_cache")]
    fn refresh_schema_cache(&self) {
        if self.db_conn.is_none() {
            return;
        }

        let tx = self.internal_tx.clone();
        let conn = self.db_conn.as_ref().unwrap().pool.clone();

        async_std::task::spawn(async move {
            let schema_cache = SchemaCache::load(&conn).await;
            tx.send(InternalMessage::SetSchemaCache(schema_cache))
                .unwrap();
        });
    }

    fn did_change_configuration(
        &mut self,
        params: DidChangeConfigurationParams,
    ) -> anyhow::Result<()> {
        if self.client_flags.configuration_pull {
            self.pull_options();
        } else {
            let options = self.client.parse_options(params.settings)?;
            self.update_options(options);
        }

        Ok(())
    }

    fn process_messages(&mut self) -> anyhow::Result<()> {
        loop {
            crossbeam_channel::select! {
                recv(&self.connection.receiver) -> msg => {
                    match msg? {
                        Message::Request(request) => {
                            if self.connection.handle_shutdown(&request)? {
                                return Ok(());
                            }

                            if let Some(response) = dispatch::RequestDispatcher::new(request)
                                .default()
                            {
                                self.client.send_response(response)?;
                            }
                        }
                        Message::Notification(notification) => {
                            dispatch::NotificationDispatcher::new(notification)
                                .on::<DidChangeConfiguration, _>(|params| {
                                    self.did_change_configuration(params)
                                })?
                                .on::<DidCloseTextDocument, _>(|params| self.did_close(params))?
                                .on::<DidOpenTextDocument, _>(|params| self.did_open(params))?
                                .on::<DidChangeTextDocument, _>(|params| self.did_change(params))?
                                .on::<DidSaveTextDocument, _>(|params| self.did_save(params))?
                                .on::<DidCloseTextDocument, _>(|params| self.did_close(params))?
                                .default();
                        }
                        Message::Response(response) => {
                            self.client.recv_response(response)?;
                        }
                    };
                },
                recv(&self.internal_rx) -> msg => {
                    match msg? {
                        InternalMessage::SetSchemaCache(c) => {
                            self.ide.set_schema_cache(c);
                        }
                        InternalMessage::RefreshSchemaCache => {
                            self.refresh_schema_cache();
                        }
                        InternalMessage::Diagnostics => {
                            self.publish_diagnostics()?;
                        }
                        InternalMessage::SetOptions(options) => {
                            self.update_options(options);
                        }
                    };
                }
            };
        }
    }

    fn pull_options(&mut self) {
        if !self.client_flags.configuration_pull {
            return;
        }

        let params = ConfigurationParams {
            items: vec![ConfigurationItem {
                section: Some("postgres_lsp".to_string()),
                scope_uri: None,
            }],
        };

        let client = self.client.clone();
        let sender = self.internal_tx.clone();
        self.pool.execute(move || {
            match client.send_request::<WorkspaceConfiguration>(params) {
                Ok(mut json) => {
                    let options = client
                        .parse_options(json.pop().expect("invalid configuration request"))
                        .unwrap();

                    sender.send(InternalMessage::SetOptions(options)).unwrap();
                }
                Err(why) => {
                    // log::error!("Retrieving configuration failed: {}", why);
                }
            };
        });
    }

    fn register_configuration(&mut self) {
        if self.client_flags.configuration_push {
            let registration = Registration {
                id: "pull-config".to_string(),
                method: DidChangeConfiguration::METHOD.to_string(),
                register_options: None,
            };

            let params = RegistrationParams {
                registrations: vec![registration],
            };

            let client = self.client.clone();
            self.pool.execute(move || {
                if let Err(why) = client.send_request::<RegisterCapability>(params) {
                    // log::error!(
                    //     "Failed to register \"{}\" notification: {}",
                    //     DidChangeConfiguration::METHOD,
                    //     why
                    // );
                }
            });
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.register_configuration();
        self.pull_options();
        self.process_messages()?;
        self.pool.join();
        Ok(())
    }
}
