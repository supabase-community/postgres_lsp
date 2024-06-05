mod dispatch;
pub mod options;

use async_std::task::{self};
use base_db::{Change, Document, DocumentChange, PgLspPath, StatementRef};
use crossbeam_channel::{unbounded, Receiver, Sender};
use hover::HoverParams;
use ide::IDE;
use lsp_server::{Connection, Message, Notification, RequestId};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
        DidSaveTextDocument, LogMessage, Notification as _, PublishDiagnostics, ShowMessage,
    },
    request::{HoverRequest, RegisterCapability, WorkspaceConfiguration},
    ConfigurationItem, ConfigurationParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, HoverProviderCapability, InitializeParams, InitializeResult,
    LogMessageParams, PublishDiagnosticsParams, Registration, RegistrationParams, SaveOptions,
    ServerCapabilities, ServerInfo, ShowMessageParams, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
};
use schema_cache::SchemaCache;
use serde::Serialize;
use std::sync::Arc;
use threadpool::ThreadPool;
use tracing::{event, instrument, Level};

use crate::{
    client::{client_flags::ClientFlags, LspClient},
    utils::{file_path, from_proto, line_index_ext::LineIndexExt, normalize_uri, to_proto},
};

use self::options::Options;
use sqlx::{
    postgres::{PgListener, PgPool},
    Statement,
};

#[derive(Debug)]
enum InternalMessage {
    PublishDiagnostics(lsp_types::Url),
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
    ide: Arc<IDE>,
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
            ide: Arc::new(IDE::new()),
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
            listener
                .listen_all(["postgres_lsp", "pgrst"])
                .await
                .unwrap();

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

        self.refresh_schema_cache();

        self.start_listening();
    }

    #[instrument(skip(self), name = "pglsp/update_options")]
    fn update_options(&mut self, options: Options) {
        async_std::task::block_on(self.update_db_connection(options.db_connection_string));
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
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            ..ServerCapabilities::default()
        }
    }

    fn publish_diagnostics(&self, uri: lsp_types::Url) -> anyhow::Result<()> {
        let mut url = uri.clone();
        normalize_uri(&mut url);

        let path = file_path(&url);

        let doc = self.ide.documents.get(&path);

        if doc.is_none() {
            return Ok(());
        }

        let diagnostics: Vec<lsp_types::Diagnostic> = self
            .ide
            .diagnostics(&path)
            .iter()
            .map(|d| to_proto::diagnostic(&doc.as_ref().unwrap(), d))
            .collect();

        let params = PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        };

        self.client
            .send_notification::<PublishDiagnostics>(params)?;

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

        self.publish_diagnostics(uri)?;

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

        Ok(())
    }

    #[instrument(skip(self), name = "pglsp/did_save")]
    fn did_save(&self, params: DidSaveTextDocumentParams) -> anyhow::Result<()> {
        // on save we want to run static analysis and ultimately publish diagnostics
        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        let cloned_uri = uri.clone();
        self.publish_diagnostics(uri)?;

        let ide = Arc::clone(&self.ide);
        let tx = self.internal_tx.clone();
        let conn = self.db_conn.as_ref().map(|p| p.pool.clone());

        self.pool.execute(move || {
            // TODO this should happen on change too but once at a time after some debounced delay
            // also on open
            // check chatgpt for sample for debouncer
            ide.compute(conn);
            tx.send(InternalMessage::PublishDiagnostics(cloned_uri));
        });

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

        self.ide.remove_document(path);

        Ok(())
    }

    fn hover(&mut self, id: RequestId, mut params: lsp_types::HoverParams) -> anyhow::Result<()> {
        normalize_uri(&mut params.text_document_position_params.text_document.uri);

        self.run_query(id, move |ide| {
            let path = file_path(&params.text_document_position_params.text_document.uri);
            let doc = ide.documents.get(&path)?;

            let pos = doc
                .line_index
                .offset_lsp(params.text_document_position_params.position)
                .unwrap();

            let (range, stmt) = doc.statement_at_offset_with_range(&pos)?;

            ::hover::hover(HoverParams {
                position: pos - range.start(),
                source: stmt.text.clone(),
                enriched_ast: ide
                    .pg_query
                    .enriched_ast(&stmt)
                    .as_ref()
                    .map(|x| x.as_ref()),
                tree: ide.tree_sitter.tree(&stmt).as_ref().map(|x| x.as_ref()),
                schema_cache: ide.schema_cache.read().unwrap().clone(),
            })
            .map(|hover| lsp_types::Hover {
                contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
                    hover.content,
                )),
                range: Some(doc.line_index.line_col_lsp_range(range).unwrap()),
            })
        });

        Ok(())
    }

    fn run_query<R, Q>(&self, id: RequestId, query: Q)
    where
        R: Serialize,
        Q: FnOnce(&IDE) -> R + Send + 'static,
    {
        let client = self.client.clone();
        let ide = Arc::clone(&self.ide);

        self.pool.execute(move || {
            let response = lsp_server::Response::new_ok(id, query(&ide));
            client.send_response(response).unwrap();
        });
    }

    #[instrument(skip(self), name = "pglsp/refresh_schema_cache")]
    fn refresh_schema_cache(&self) {
        if self.db_conn.is_none() {
            return;
        }

        let tx = self.internal_tx.clone();
        let conn = self.db_conn.as_ref().unwrap().pool.clone();
        let client = self.client.clone();

        async_std::task::spawn(async move {
            client
                .send_notification::<ShowMessage>(ShowMessageParams {
                    typ: lsp_types::MessageType::INFO,
                    message: "Refreshing schema cache...".to_string(),
                })
                .unwrap();
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
                                .on::<HoverRequest, _>(|id, params| self.hover(id, params))?
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
                            self.client
                                .send_notification::<ShowMessage>(ShowMessageParams {
                                    typ: lsp_types::MessageType::INFO,
                                    message: "Schema cache loaded".to_string(),
                                })
                                .unwrap();
                        }
                        InternalMessage::RefreshSchemaCache => {
                            self.refresh_schema_cache();
                        }
                        InternalMessage::PublishDiagnostics(uri) => {
                            self.publish_diagnostics(uri)?;
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
