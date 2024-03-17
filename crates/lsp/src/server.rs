mod dispatch;

use base_db::{DocumentChangesParams, PgLspPath, SourceFile, SourceFileParams};
use crossbeam_channel::{unbounded, Receiver, Sender};
use dashmap::DashMap;
use lsp_server::{Connection, ErrorCode, Message, RequestId};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
        PublishDiagnostics,
    },
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializeParams, InitializeResult, PublishDiagnosticsParams,
    SaveOptions, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use threadpool::ThreadPool;

use crate::{
    client::LspClient,
    utils::{file_path, from_proto, normalize_uri},
};

#[derive(Debug)]
enum InternalMessage {
    Diagnostics,
}

pub struct Server {
    connection: Arc<Connection>,
    client: LspClient,
    internal_tx: Sender<InternalMessage>,
    internal_rx: Receiver<InternalMessage>,
    pool: ThreadPool,

    // it might make sense to move this into its own struct at some point
    // do we need a dashmap?
    documents: DashMap<PgLspPath, SourceFile>,
}

impl Server {
    pub fn init(connection: Connection) -> anyhow::Result<()> {
        let client = LspClient::new(connection.sender.clone());

        let (internal_tx, internal_rx) = unbounded();

        let (id, params) = connection.initialize_start()?;
        let params: InitializeParams = serde_json::from_value(params)?;

        // TODO: setup pg notify listener and refresh schema cache

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

            documents: DashMap::new(),
        };

        server.run()?;
        Ok(())
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
            SourceFile::new(SourceFileParams {
                text: params.text_document.text,
            }),
        );

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

        // let mut uri = params.text_document.uri;
        // normalize_uri(&mut uri);
        //
        // if self.workspace.read().config().build.on_save {
        //     let text_document = TextDocumentIdentifier::new(uri.clone());
        //     let params = BuildParams {
        //         text_document,
        //         position: None,
        //     };
        //
        //     self.build(None, params)?;
        // }
        //
        // self.publish_diagnostics_with_delay();
        //
        // if self.workspace.read().config().diagnostics.chktex.on_save {
        //     self.run_chktex(&uri);
        // }

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

        // let mut uri = params.text_document.uri;
        // normalize_uri(&mut uri);
        // self.workspace.write().close(&uri);
        // self.publish_diagnostics_with_delay();
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
                    };
                }
            };
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.process_messages()?;
        self.pool.join();
        Ok(())
    }
}
