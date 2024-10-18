mod debouncer;
mod dispatch;
pub mod options;

use lsp_server::{Connection, ErrorCode, Message, RequestId};
use lsp_types::{
    notification::{
        DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
        DidSaveTextDocument, Notification as _, PublishDiagnostics, ShowMessage,
    },
    request::{
        CodeActionRequest, Completion, ExecuteCommand, HoverRequest, InlayHintRequest,
        RegisterCapability, WorkspaceConfiguration,
    },
    CompletionList, CompletionOptions, ConfigurationItem, ConfigurationParams,
    DidChangeConfigurationParams, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, ExecuteCommandOptions,
    ExecuteCommandParams, HoverProviderCapability, InitializeParams, InitializeResult,
    PublishDiagnosticsParams, Registration, RegistrationParams, SaveOptions, ServerCapabilities,
    ServerInfo, ShowMessageParams, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
};
use pg_base_db::{Change, DocumentChange};
use pg_commands::{Command, CommandType, ExecuteStatementCommand};
use pg_completions::CompletionParams;
use pg_hover::HoverParams;
use pg_schema_cache::SchemaCache;
use pg_workspace::Workspace;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashSet, future::Future, sync::Arc, time::Duration};
use text_size::TextSize;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    client::{client_flags::ClientFlags, LspClient},
    db_connection::DbConnection,
    utils::{file_path, from_proto, line_index_ext::LineIndexExt, normalize_uri, to_proto},
};

use self::{debouncer::EventDebouncer, options::Options};
use sqlx::{postgres::PgPool, Executor};

#[derive(Debug)]
enum InternalMessage {
    PublishDiagnostics(lsp_types::Url),
    SetOptions(Options),
    SetSchemaCache(SchemaCache),
    SetDatabaseConnection(DbConnection),
}

/// `lsp-servers` `Connection` type uses a crossbeam channel, which is not compatible with tokio's async runtime.
/// For now, we move it into a separate task and use tokio's channels to communicate.
fn get_client_receiver(
    connection: Connection,
    cancel_token: Arc<CancellationToken>,
) -> mpsc::UnboundedReceiver<Message> {
    let (message_tx, message_rx) = mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        // TODO: improve Result handling
        loop {
            let msg = connection.receiver.recv().unwrap();

            match msg {
                Message::Request(r) if connection.handle_shutdown(&r).unwrap() => {
                    cancel_token.cancel();
                    return;
                }

                _ => message_tx.send(msg).unwrap(),
            };
        }
    });

    message_rx
}

pub struct Server {
    client_rx: mpsc::UnboundedReceiver<Message>,
    cancel_token: Arc<tokio_util::sync::CancellationToken>,
    client: LspClient,
    internal_tx: mpsc::UnboundedSender<InternalMessage>,
    internal_rx: mpsc::UnboundedReceiver<InternalMessage>,
    client_flags: Arc<ClientFlags>,
    ide: Arc<Workspace>,
    db_conn: Option<DbConnection>,
    compute_debouncer: EventDebouncer<Option<PgPool>>,
}

impl Server {
    pub fn init(connection: Connection) -> anyhow::Result<Self> {
        let client = LspClient::new(connection.sender.clone());
        let cancel_token = Arc::new(CancellationToken::new());

        let (params, client_rx) = Self::establish_client_connection(connection, &cancel_token)?;

        let client_flags = Arc::new(ClientFlags::from_initialize_request_params(&params));

        let pool = Arc::new(threadpool::Builder::new().build());

        let ide = Arc::new(Workspace::new());

        let (internal_tx, internal_rx) = mpsc::unbounded_channel();

        let cloned_tx = internal_tx.clone();
        let cloned_ide = ide.clone();
        let cloned_pool = pool.clone();
        let cloned_client = client.clone();

        let server = Self {
            cancel_token,
            client_rx,
            internal_rx,
            internal_tx,
            client,
            client_flags,
            db_conn: None,
            ide,
            compute_debouncer: EventDebouncer::new(
                Duration::from_millis(500),
                move |conn: Option<PgPool>| {
                    let inner_cloned_ide = cloned_ide.clone();
                    let inner_cloned_tx = cloned_tx.clone();
                    let inner_cloned_client = cloned_client.clone();
                    cloned_pool.execute(move || {
                        inner_cloned_client
                            .send_notification::<ShowMessage>(ShowMessageParams {
                                typ: lsp_types::MessageType::INFO,
                                message: format!("Computing debounced {}", conn.is_some()),
                            })
                            .unwrap();
                        let r =
                            async_std::task::block_on(conn.as_ref().unwrap().execute("SELECT 1"));
                        inner_cloned_client
                            .send_notification::<ShowMessage>(ShowMessageParams {
                                typ: lsp_types::MessageType::INFO,
                                message: format!("res {:?}", r.unwrap()),
                            })
                            .unwrap();

                        let changed = inner_cloned_ide.compute(conn);

                        let urls = HashSet::<&str>::from_iter(
                            changed.iter().map(|f| f.document_url.to_str().unwrap()),
                        );
                        for url in urls.iter() {
                            inner_cloned_tx
                                .send(InternalMessage::PublishDiagnostics(
                                    lsp_types::Url::from_file_path(url).unwrap(),
                                ))
                                .unwrap();
                        }
                    });
                },
            ),
        };

        Ok(server)
    }

    fn compute_now(&self) {
        let conn = self.db_conn.as_ref().map(|p| p.pool.clone());
        let cloned_ide = self.ide.clone();
        let cloned_tx = self.internal_tx.clone();
        let client = self.client.clone();

        self.compute_debouncer.clear();

        self.spawn_with_cancel(async move {
            client
                .send_notification::<ShowMessage>(ShowMessageParams {
                    typ: lsp_types::MessageType::INFO,
                    message: format!("Computing now {}", conn.is_some()),
                })
                .unwrap();

            if conn.is_some() {
                client
                    .send_notification::<ShowMessage>(ShowMessageParams {
                        typ: lsp_types::MessageType::INFO,
                        message: format!("pool closed {}", conn.as_ref().unwrap().is_closed()),
                    })
                    .unwrap();

                let r = async_std::task::block_on(conn.as_ref().unwrap().execute("SELECT 1"));
                client
                    .send_notification::<ShowMessage>(ShowMessageParams {
                        typ: lsp_types::MessageType::INFO,
                        message: format!("res {:?}", r.unwrap()),
                    })
                    .unwrap();
            }
            let changed = cloned_ide.compute(conn);
            let urls = HashSet::<&str>::from_iter(
                changed.iter().map(|f| f.document_url.to_str().unwrap()),
            );

            for url in urls {
                cloned_tx
                    .send(InternalMessage::PublishDiagnostics(
                        lsp_types::Url::from_file_path(url).unwrap(),
                    ))
                    .unwrap();
            }
        });
    }

    fn update_db_connection(&self, options: Options) -> anyhow::Result<()> {
        if options.db_connection_string.is_none()
            || self
                .db_conn
                .as_ref()
                .is_some_and(|c| c.connected_to(options.db_connection_string.as_ref().unwrap()))
        {
            return Ok(());
        }

        let connection_string = options.db_connection_string.unwrap();

        let internal_tx = self.internal_tx.clone();
        let client = self.client.clone();
        self.spawn_with_cancel(async move {
            match DbConnection::new(connection_string.into()).await {
                Ok(conn) => {
                    internal_tx
                        .send(InternalMessage::SetDatabaseConnection(conn))
                        .unwrap();
                }
                Err(why) => {
                    client.send_info_notification(&format!("Unable to update database connection: {}", why));
                    
                }
            }
        });

        Ok(())
    }

    async fn listen_for_schema_updates(&mut self) -> anyhow::Result<()> {
        if self.db_conn.is_none() {
            eprintln!("Error trying to listen for schema updates: No database connection");
            return Ok(());
        }

        let internal_tx = self.internal_tx.clone();
        self.db_conn
            .as_mut()
            .unwrap()
            .listen_for_schema_updates(move |schema_cache| {
                internal_tx
                    .send(InternalMessage::SetSchemaCache(schema_cache))
                    .unwrap();
                // TODO: handle result
            })
            .await?;

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
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: CommandType::ALL
                    .iter()
                    .map(|c| c.id().to_string())
                    .collect(),
                ..Default::default()
            }),
            inlay_hint_provider: Some(lsp_types::OneOf::Left(true)),
            code_action_provider: Some(lsp_types::CodeActionProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions::default()),
            ..ServerCapabilities::default()
        }
    }

    // TODO allow option url and publish diagnostics for all files
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

        self.client
            .send_notification::<ShowMessage>(ShowMessageParams {
                typ: lsp_types::MessageType::INFO,
                message: format!("diagnostics {}", diagnostics.len()),
            })
            .unwrap();

        let params = PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        };

        self.client
            .send_notification::<PublishDiagnostics>(params)?;

        Ok(())
    }

    fn did_open(&self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
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

        self.compute_now();

        Ok(())
    }

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

        let conn = self.db_conn.as_ref().map(|p| p.pool.clone());
        self.compute_debouncer.put(conn);

        Ok(())
    }

    fn did_save(&self, params: DidSaveTextDocumentParams) -> anyhow::Result<()> {
        let mut uri = params.text_document.uri;
        normalize_uri(&mut uri);

        self.publish_diagnostics(uri)?;

        self.compute_now();

        Ok(())
    }

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

    fn code_actions(
        &self,
        id: RequestId,
        params: lsp_types::CodeActionParams,
    ) -> anyhow::Result<()> {
        let db_conn = self.db_conn.as_ref().map(|p| p.pool.clone());
        self.run_query(id, move |ide| {
            let mut actions = Vec::<lsp_types::CodeAction>::new();

            if db_conn.is_none() {
                return actions;
            }

            let mut uri = params.text_document.uri;
            normalize_uri(&mut uri);
            let path = file_path(&uri);

            let doc = ide.documents.get(&path);

            if doc.is_none() {
                return actions;
            }

            let doc = doc.unwrap();

            let range = doc.line_index.offset_lsp_range(params.range).unwrap();

            actions.extend(doc.statements_at_range(&range).iter().map(|stmt| {
                let cmd = ExecuteStatementCommand::command_type();
                let title = format!(
                    "Execute '{}'",
                    ExecuteStatementCommand::trim_statement(stmt.text.clone(), 50)
                );
                lsp_types::CodeAction {
                    title: title.clone(),
                    kind: None,
                    edit: None,
                    command: Some(lsp_types::Command {
                        title,
                        command: format!("pglsp.{}", cmd.id()),
                        arguments: Some(vec![serde_json::to_value(stmt.text.clone()).unwrap()]),
                    }),
                    diagnostics: None,
                    is_preferred: None,
                    disabled: None,
                    data: None,
                }
            }));

            actions
        });

        Ok(())
    }

    fn inlay_hint(
        &self,
        id: RequestId,
        mut params: lsp_types::InlayHintParams,
    ) -> anyhow::Result<()> {
        normalize_uri(&mut params.text_document.uri);

        let c = self.client.clone();

        self.run_query(id, move |ide| {
            let path = file_path(&params.text_document.uri);

            let doc = ide.documents.get(&path);

            if doc.is_none() {
                return Vec::new();
            }

            let doc = doc.unwrap();

            let range = doc.line_index.offset_lsp_range(params.range).unwrap();

            let schema_cache = ide.schema_cache.read().unwrap();

            c.send_notification::<ShowMessage>(ShowMessageParams {
                typ: lsp_types::MessageType::INFO,
                message: "querying inlay hints".to_string(),
            })
            .unwrap();

            doc.statements_at_range(&range)
                .iter()
                .flat_map(|stmt| {
                    ::pg_inlay_hints::inlay_hints(::pg_inlay_hints::InlayHintsParams {
                        ast: ide.pg_query.ast(&stmt).as_ref().map(|x| x.as_ref()),
                        enriched_ast: ide
                            .pg_query
                            .enriched_ast(&stmt)
                            .as_ref()
                            .map(|x| x.as_ref()),
                        tree: ide.tree_sitter.tree(&stmt).as_ref().map(|x| x.as_ref()),
                        cst: ide.pg_query.cst(&stmt).as_ref().map(|x| x.as_ref()),
                        schema_cache: &schema_cache,
                    })
                })
                .map(|hint| lsp_types::InlayHint {
                    position: doc.line_index.line_col_lsp(hint.offset).unwrap(),
                    label: match hint.content {
                        pg_inlay_hints::InlayHintContent::FunctionArg(arg) => {
                            lsp_types::InlayHintLabel::String(match arg.name {
                                Some(name) => format!("{} ({})", name, arg.type_name),
                                None => arg.type_name.clone(),
                            })
                        }
                    },
                    kind: match hint.content {
                        pg_inlay_hints::InlayHintContent::FunctionArg(_) => {
                            Some(lsp_types::InlayHintKind::PARAMETER)
                        }
                    },
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: None,
                    data: None,
                })
                .collect()
        });

        Ok(())
    }

    fn completion(
        &self,
        id: RequestId,
        mut params: lsp_types::CompletionParams,
    ) -> anyhow::Result<()> {
        normalize_uri(&mut params.text_document_position.text_document.uri);

        self.run_query(id, move |ide| {
            let path = file_path(&params.text_document_position.text_document.uri);

            let doc = ide.documents.get(&path)?;

            let pos = doc
                .line_index
                .offset_lsp(params.text_document_position.position)
                .unwrap();

            let (range, stmt) = doc.statement_at_offset_with_range(&pos)?;

            let schema = ide.schema_cache.read().unwrap();

            Some(CompletionList {
                is_incomplete: false,
                items: pg_completions::complete(&CompletionParams {
                    position: pos - range.start() - TextSize::from(1),
                    text: stmt.text.as_str(),
                    tree: ide.tree_sitter.tree(&stmt).as_ref().map(|x| x.as_ref()),
                    schema: &schema,
                })
                .items
                .iter()
                .map(|i| lsp_types::CompletionItem {
                    // TODO: add more data
                    label: i.data.label().to_string(),
                    label_details: None,
                    kind: Some(lsp_types::CompletionItemKind::CLASS),
                    detail: None,
                    documentation: None,
                    deprecated: None,
                    preselect: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: None,
                    insert_text_format: None,
                    insert_text_mode: None,
                    text_edit: None,
                    additional_text_edits: None,
                    commit_characters: None,
                    data: None,
                    tags: None,
                    command: None,
                })
                .collect(),
            })
        });

        Ok(())
    }

    fn hover(&self, id: RequestId, mut params: lsp_types::HoverParams) -> anyhow::Result<()> {
        normalize_uri(&mut params.text_document_position_params.text_document.uri);

        self.run_query(id, move |ide| {
            let path = file_path(&params.text_document_position_params.text_document.uri);
            let doc = ide.documents.get(&path)?;

            let pos = doc
                .line_index
                .offset_lsp(params.text_document_position_params.position)
                .unwrap();

            let (range, stmt) = doc.statement_at_offset_with_range(&pos)?;

            ::pg_hover::hover(HoverParams {
                position: pos - range.start(),
                source: stmt.text.as_str(),
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

    fn execute_command(&self, id: RequestId, params: ExecuteCommandParams) -> anyhow::Result<()> {
        match CommandType::from_id(params.command.replace("pglsp.", "").as_str()) {
            Some(CommandType::ExecuteStatement) => {
                let stmt = self.parse_command_params::<String>(params.arguments)?;

                let command = ExecuteStatementCommand::new(stmt);

                let conn = self.db_conn.as_ref().map(|p| p.pool.clone());

                let client = self.client.clone();

                self.run_fallible(id, move || {
                    // todo return the rows and do something with them
                    // maybe store them and add the table to the hover output?
                    let res = async_std::task::block_on(command.run(conn))?;

                    // todo if its a ddl statement, recompute schema cache

                    client
                        .send_notification::<ShowMessage>(ShowMessageParams {
                            typ: lsp_types::MessageType::INFO,
                            message: format!("Success! Affected rows: {}", res.rows_affected()),
                        })
                        .unwrap();

                    Ok(())
                });
            }
            None => {
                self.client
                    .send_error(
                        id,
                        ErrorCode::InvalidParams,
                        format!("Unknown command: {}", params.command),
                    )
                    .unwrap();
            }
        };

        Ok(())
    }

    fn run_fallible<R, Q>(&self, id: RequestId, query: Q)
    where
        R: Serialize,
        Q: FnOnce() -> anyhow::Result<R> + Send + 'static,
    {
        let client = self.client.clone();
        self.spawn_with_cancel(async move {
            match query() {
                Ok(result) => {
                    let response = lsp_server::Response::new_ok(id, result);
                    client.send_response(response).unwrap();
                }
                Err(why) => {
                    client
                        .send_error(id, ErrorCode::InternalError, why.to_string())
                        .unwrap();
                }
            }
        });
    }

    fn parse_command_params<T: DeserializeOwned>(
        &self,
        params: Vec<serde_json::Value>,
    ) -> anyhow::Result<T> {
        if params.is_empty() {
            anyhow::bail!("No argument provided!");
        }

        let value = params.into_iter().next().unwrap();
        let value = serde_json::from_value(value)?;
        Ok(value)
    }

    fn run_query<R, Q>(&self, id: RequestId, query: Q)
    where
        R: Serialize,
        Q: FnOnce(&Workspace) -> R + Send + 'static,
    {
        let client = self.client.clone();
        let ide = Arc::clone(&self.ide);

        self.spawn_with_cancel(async move {
            let response = lsp_server::Response::new_ok(id, query(&ide));
            client
                .send_response(response)
                .expect("Failed to send query to client");
        });
    }

    fn did_change_configuration(
        &mut self,
        params: DidChangeConfigurationParams,
    ) -> anyhow::Result<()> {
        if self.client_flags.has_configuration {
            self.pull_options();
        } else {
            let options = self.client.parse_options(params.settings)?;
            self.update_db_connection(options);
        }

        Ok(())
    }

    async fn process_messages(&mut self) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    // Close the loop, proceed to shutdown.
                    return Ok(())
                },

                msg = self.internal_rx.recv() => {
                    match msg {
                        None => panic!("The LSP's internal sender closed. This should never happen."),
                        Some(m) => self.handle_internal_message(m).await
                    }
                },

                msg = self.client_rx.recv() => {
                    match msg {
                        None => panic!("The LSP's client closed, but not via an 'exit' method. This should never happen."),
                        Some(m) => self.handle_message(m).await
                    }
                },
            }?;
        }
    }

    async fn handle_message(&mut self, msg: Message) -> anyhow::Result<()> {
        match msg {
            Message::Request(request) => {
                if let Some(response) = dispatch::RequestDispatcher::new(request)
                    .on::<InlayHintRequest, _>(|id, params| self.inlay_hint(id, params))?
                    .on::<HoverRequest, _>(|id, params| self.hover(id, params))?
                    .on::<ExecuteCommand, _>(|id, params| self.execute_command(id, params))?
                    .on::<Completion, _>(|id, params| self.completion(id, params))?
                    .on::<CodeActionRequest, _>(|id, params| self.code_actions(id, params))?
                    .default()
                {
                    self.client.send_response(response)?;
                }
            }
            Message::Notification(notification) => {
                dispatch::NotificationDispatcher::new(notification)
                    .on::<DidChangeConfiguration, _>(|params| {
                        self.did_change_configuration(params);
                        Ok(())
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
        }

        Ok(())
    }

    async fn handle_internal_message(&mut self, msg: InternalMessage) -> anyhow::Result<()> {
        match msg {
            InternalMessage::SetSchemaCache(c) => {
                self.client
                    .send_info_notification("Refreshing Schema Cache...");
                self.ide.set_schema_cache(c);
                self.client.send_info_notification("Updated Schema Cache.");
                self.compute_now();
            }
            InternalMessage::PublishDiagnostics(uri) => {
                self.publish_diagnostics(uri)?;
            }
            InternalMessage::SetOptions(options) => {
                self.update_db_connection(options)?;
            }
            InternalMessage::SetDatabaseConnection(conn) => {
                let current = self.db_conn.replace(conn);
                if current.is_some() {
                    current.unwrap().close().await
                }
                self.listen_for_schema_updates().await?;
            }
        }

        Ok(())
    }

    fn pull_options(&mut self) {
        let params = ConfigurationParams {
            items: vec![ConfigurationItem {
                section: Some("postgres_lsp".to_string()),
                scope_uri: None,
            }],
        };

        let client = self.client.clone();
        let internal_tx = self.internal_tx.clone();
        self.spawn_with_cancel(async move {
            match client.send_request::<WorkspaceConfiguration>(params) {
                Ok(mut json) => {
                    let options = client
                        .parse_options(json.pop().expect("invalid configuration request"))
                        .unwrap();

                    if let Err(why) = internal_tx.send(InternalMessage::SetOptions(options)) {
                        println!("Failed to set internal options: {}", why);
                    }
                }
                Err(why) => {
                    println!("Retrieving configuration failed: {}", why);
                }
            };
        });
    }

    fn register_configuration(&mut self) {
        let registration = Registration {
            id: "pull-config".to_string(),
            method: DidChangeConfiguration::METHOD.to_string(),
            register_options: None,
        };

        let params = RegistrationParams {
            registrations: vec![registration],
        };

        let client = self.client.clone();
        self.spawn_with_cancel(async move {
            if let Err(why) = client.send_request::<RegisterCapability>(params) {
                println!(
                    "Failed to register \"{}\" notification: {}",
                    DidChangeConfiguration::METHOD,
                    why
                );
            }
        });
    }

    fn establish_client_connection(
        connection: Connection,
        cancel_token: &Arc<CancellationToken>,
    ) -> anyhow::Result<(InitializeParams, mpsc::UnboundedReceiver<Message>)> {
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

        let client_rx = get_client_receiver(connection, cancel_token.clone());

        Ok((params, client_rx))
    }

    fn spawn_with_cancel<F, O>(&self, f: F) -> tokio::task::JoinHandle<Option<F::Output>>
    where
        F: Future<Output = O> + Send + 'static,
        O: Send + 'static,
    {
        let cancel_token = self.cancel_token.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = cancel_token.cancelled() => None,
                output = f => Some(output)
            }
        })
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        if self.client_flags.will_push_configuration {
            self.register_configuration();
        }

        if self.client_flags.has_configuration {
            self.pull_options();
        }

        self.process_messages().await?;

        Ok(())
    }
}
