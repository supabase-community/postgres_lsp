use anyhow::bail;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use futures::channel::mpsc::{channel, Sender};
use futures::Sink;
use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use pglt_lsp::LSPServer;
use pglt_lsp::ServerFactory;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_value};
use std::any::type_name;
use std::fmt::Display;
use std::time::Duration;
use tower::timeout::Timeout;
use tower::{Service, ServiceExt};
use tower_lsp::jsonrpc;
use tower_lsp::jsonrpc::Response;
use tower_lsp::lsp_types as lsp;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::InitializedParams;
use tower_lsp::lsp_types::PublishDiagnosticsParams;
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;
use tower_lsp::lsp_types::TextDocumentIdentifier;
use tower_lsp::lsp_types::TextDocumentItem;
use tower_lsp::lsp_types::VersionedTextDocumentIdentifier;
use tower_lsp::lsp_types::{ClientCapabilities, Url};
use tower_lsp::lsp_types::{DidChangeConfigurationParams, DidChangeTextDocumentParams};
use tower_lsp::LspService;
use tower_lsp::{jsonrpc::Request, lsp_types::InitializeParams};

/// Statically build an [Url] instance that points to the file at `$path`
/// within the workspace. The filesystem path contained in the return URI is
/// guaranteed to be a valid path for the underlying operating system, but
/// doesn't have to refer to an existing file on the host machine.
macro_rules! url {
    ($path:literal) => {
        if cfg!(windows) {
            lsp::Url::parse(concat!("file:///z%3A/workspace/", $path)).unwrap()
        } else {
            lsp::Url::parse(concat!("file:///workspace/", $path)).unwrap()
        }
    };
}

struct Server {
    service: Timeout<LspService<LSPServer>>,
}

impl Server {
    fn new(service: LspService<LSPServer>) -> Self {
        Self {
            service: Timeout::new(service, Duration::from_secs(1)),
        }
    }

    async fn notify<P>(&mut self, method: &'static str, params: P) -> Result<()>
    where
        P: Serialize,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn request<P, R>(
        &mut self,
        method: &'static str,
        id: &'static str,
        params: P,
    ) -> Result<Option<R>>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .id(id)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")?
            .map(|res| {
                let (_, body) = res.into_parts();

                let body =
                    body.with_context(|| format!("response to {method:?} contained an error"))?;

                from_value(body.clone()).with_context(|| {
                    format!(
                        "failed to deserialize type {} from response {body:?}",
                        type_name::<R>()
                    )
                })
            })
            .transpose()
    }

    /// Basic implementation of the `initialize` request for tests
    // The `root_path` field is deprecated, but we still need to specify it
    #[allow(deprecated)]
    async fn initialize(&mut self) -> Result<()> {
        let _res: InitializeResult = self
            .request(
                "initialize",
                "_init",
                InitializeParams {
                    process_id: None,
                    root_path: None,
                    root_uri: Some(url!("")),
                    initialization_options: None,
                    capabilities: ClientCapabilities::default(),
                    trace: None,
                    workspace_folders: None,
                    client_info: None,
                    locale: None,
                },
            )
            .await?
            .context("initialize returned None")?;

        Ok(())
    }

    /// Basic implementation of the `initialized` notification for tests
    async fn initialized(&mut self) -> Result<()> {
        self.notify("initialized", InitializedParams {}).await
    }

    /// Basic implementation of the `shutdown` notification for tests
    async fn shutdown(&mut self) -> Result<()> {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(Request::build("shutdown").finish())
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn open_document(&mut self, text: impl Display) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: url!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    async fn open_untitled_document(&mut self, text: impl Display) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: url!("untitled-1"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    /// Opens a document with given contents and given name. The name must contain the extension too
    async fn open_named_document(
        &mut self,
        text: impl Display,
        document_name: Url,
        language: impl Display,
    ) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: document_name,
                    language_id: language.to_string(),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    /// When calling this function, remember to insert the file inside the memory file system
    async fn load_configuration(&mut self) -> Result<()> {
        self.notify(
            "workspace/didChangeConfiguration",
            DidChangeConfigurationParams {
                settings: to_value(()).unwrap(),
            },
        )
        .await
    }

    async fn change_document(
        &mut self,
        version: i32,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<()> {
        self.notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: url!("document.js"),
                    version,
                },
                content_changes,
            },
        )
        .await
    }

    async fn close_document(&mut self) -> Result<()> {
        self.notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: url!("document.js"),
                },
            },
        )
        .await
    }

    /// Basic implementation of the `pglt/shutdown` request for tests
    async fn pglt_shutdown(&mut self) -> Result<()> {
        self.request::<_, ()>("pglt/shutdown", "_pglt_shutdown", ())
            .await?
            .context("pglt/shutdown returned None")?;
        Ok(())
    }
}

/// Number of notifications buffered by the server-to-client channel before it starts blocking the current task
const CHANNEL_BUFFER_SIZE: usize = 8;

#[derive(Debug, PartialEq, Eq)]
enum ServerNotification {
    PublishDiagnostics(PublishDiagnosticsParams),
}

/// Basic handler for requests and notifications coming from the server for tests
async fn client_handler<I, O>(
    mut stream: I,
    mut sink: O,
    mut notify: Sender<ServerNotification>,
) -> Result<()>
where
    // This function has to be generic as `RequestStream` and `ResponseSink`
    // are not exported from `tower_lsp` and cannot be named in the signature
    I: Stream<Item = Request> + Unpin,
    O: Sink<Response> + Unpin,
{
    while let Some(req) = stream.next().await {
        if req.method() == "textDocument/publishDiagnostics" {
            let params = req.params().expect("invalid request");
            let diagnostics = from_value(params.clone()).expect("invalid params");
            let notification = ServerNotification::PublishDiagnostics(diagnostics);
            match notify.send(notification).await {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let id = match req.id() {
            Some(id) => id,
            None => continue,
        };

        let res = Response::from_error(id.clone(), jsonrpc::Error::method_not_found());

        sink.send(res).await.ok();
    }

    Ok(())
}

#[tokio::test]
async fn basic_lifecycle() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create(None).into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}
