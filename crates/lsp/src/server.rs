mod dispatch;
pub mod options;

use base_db::{Document, DocumentChangesParams, DocumentParams, PgLspPath, Statement};
use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::IDE;
use lsp_server::{Connection, Message};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
        DidSaveTextDocument, Notification as _,
    },
    request::{RegisterCapability, WorkspaceConfiguration},
    ConfigurationItem, ConfigurationParams, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    InitializeParams, InitializeResult, Registration, RegistrationParams, SaveOptions,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
};
use std::sync::Arc;
use threadpool::ThreadPool;

use crate::{
    client::{client_flags::ClientFlags, LspClient},
    utils::{file_path, from_proto, normalize_uri},
};

use self::options::Options;

#[derive(Debug)]
enum InternalMessage {
    Diagnostics,
    SetOptions(Options),
}

pub struct Server {
    connection: Arc<Connection>,
    client: LspClient,
    internal_tx: Sender<InternalMessage>,
    internal_rx: Receiver<InternalMessage>,
    pool: ThreadPool,
    client_flags: Arc<ClientFlags>,
    ide: IDE,
    // TODO add "watcher" like for db schema
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

        let server = Self {
            connection: Arc::new(connection),
            internal_rx,
            internal_tx,
            client,
            pool: threadpool::Builder::new().build(),
            client_flags: Arc::new(from_proto::client_flags(
                params.capabilities,
                params.client_info,
            )),

            ide: IDE::new(),
        };

        server.run()?;
        Ok(())
    }

    fn update_options(&mut self, options: Options) {
        // let mut workspace = self.workspace.write();
        // workspace.set_config(Config::from(options));
        // TODO update pglsp watcher
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

    fn publish_diagnostics(&mut self) -> anyhow::Result<()> {
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

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        let path = file_path(&uri);

        self.documents.insert(
            path,
            Document::new(DocumentParams {
                text: params.text_document.text,
            }),
        );
        let stmts = self.test.get(&path).unwrap();

        Ok(())
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        let path = file_path(&uri);

        self.documents.entry(path).and_modify(|document| {
            let changes = from_proto::content_changes(&document, params.content_changes);

            document.apply_changes(DocumentChangesParams {
                version: params.text_document.version,
                changes,
            });
        });

        // TODO: update diagnostics

        Ok(())
    }

    fn did_save(&mut self, params: DidSaveTextDocumentParams) -> anyhow::Result<()> {
        // on save we want to run static analysis and ultimately publish diagnostics

        Ok(())
    }

    fn did_close(&mut self, params: DidCloseTextDocumentParams) -> anyhow::Result<()> {
        // this just means that the document is no longer open in the client
        // if we would listen to fs events, we would use this to overwrite the files owner to be
        // "server" instead of "client". for now we will ignore this notification since we dont
        // need to watch files that are not open.

        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);
        let path = file_path(&uri);

        self.documents.remove(&path);
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
                section: Some("pglsp".to_string()),
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
        // TODO: setup pg notify listener ddand add job to refresh schema cache
        self.register_configuration();
        self.pull_options();
        self.process_messages()?;
        self.pool.join();
        Ok(())
    }
}
