use crate::capabilities::server_capabilities;
use crate::diagnostics::{handle_lsp_error, LspError};
use crate::handlers;
use crate::session::{
    CapabilitySet, CapabilityStatus, ClientInformation, Session, SessionHandle, SessionKey,
};
use crate::utils::{into_lsp_error, panic_to_lsp_error};
use futures::future::ready;
use futures::FutureExt;
use pg_diagnostics::panic::PanicError;
use pg_fs::{ConfigName, FileSystem, OsFileSystem};
use pg_workspace_new::{workspace, DynRef, Workspace};
use rustc_hash::FxHashMap;
use serde_json::json;
use std::panic::RefUnwindSafe;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Notify;
use tokio::task::spawn_blocking;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{lsp_types::*, ClientSocket};
use tower_lsp::{LanguageServer, LspService, Server};
use tracing::{error, info};

pub struct LSPServer {
    session: SessionHandle,
    /// Map of all sessions connected to the same [ServerFactory] as this [LSPServer].
    sessions: Sessions,
    /// If this is true the server will broadcast a shutdown signal once the
    /// last client disconnected
    stop_on_disconnect: bool,
    /// This shared flag is set to true once at least one session has been
    /// initialized on this server instance
    is_initialized: Arc<AtomicBool>,
}

impl RefUnwindSafe for LSPServer {}

impl LSPServer {
    fn new(
        session: SessionHandle,
        sessions: Sessions,
        stop_on_disconnect: bool,
        is_initialized: Arc<AtomicBool>,
    ) -> Self {
        Self {
            session,
            sessions,
            stop_on_disconnect,
            is_initialized,
        }
    }

    async fn setup_capabilities(&self) {
        let mut capabilities = CapabilitySet::default();

        capabilities.add_capability(
            "pglsp_did_change_extension_settings",
            "workspace/didChangeConfiguration",
            if self.session.can_register_did_change_configuration() {
                CapabilityStatus::Enable(None)
            } else {
                CapabilityStatus::Disable
            },
        );

        capabilities.add_capability(
            "pglsp_did_change_workspace_settings",
            "workspace/didChangeWatchedFiles",
            if let Some(base_path) = self.session.base_path() {
                CapabilityStatus::Enable(Some(json!(DidChangeWatchedFilesRegistrationOptions {
                    watchers: vec![FileSystemWatcher {
                        glob_pattern: GlobPattern::String(format!(
                            "{}/pglsp.toml",
                            base_path.display()
                        )),
                        kind: Some(WatchKind::all()),
                    },],
                })))
            } else {
                CapabilityStatus::Disable
            },
        );

        self.session.register_capabilities(capabilities).await;
    }

    async fn map_op_error<T>(
        &self,
        result: Result<Result<Option<T>, LspError>, PanicError>,
    ) -> LspResult<Option<T>> {
        match result {
            Ok(result) => match result {
                Ok(result) => Ok(result),
                Err(err) => handle_lsp_error(err, &self.session.client).await,
            },

            Err(err) => Err(into_lsp_error(err)),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LSPServer {
    #[allow(deprecated)]
    #[tracing::instrument(
        level = "trace",
        skip_all,
        fields(
            root_uri = params.root_uri.as_ref().map(display),
            capabilities = debug(&params.capabilities),
            client_info = params.client_info.as_ref().map(debug),
            workspace_folders = params.workspace_folders.as_ref().map(debug),
        )
    )]
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        info!("Starting Language Server...");
        self.is_initialized.store(true, Ordering::Relaxed);

        let server_capabilities = server_capabilities(&params.capabilities);

        self.session.initialize(
            params.capabilities,
            params.client_info.map(|client_info| ClientInformation {
                name: client_info.name,
                version: client_info.version,
            }),
            params.root_uri,
            params.workspace_folders,
        );

        //
        let init = InitializeResult {
            capabilities: server_capabilities,
            server_info: Some(ServerInfo {
                name: String::from(env!("CARGO_PKG_NAME")),
                version: Some(pg_configuration::VERSION.to_string()),
            }),
        };

        Ok(init)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn initialized(&self, params: InitializedParams) {
        let _ = params;

        info!("Attempting to load the configuration from 'pglsp.toml' file");

        futures::join!(self.session.load_workspace_settings());

        let msg = format!("Server initialized with PID: {}", std::process::id());
        self.session
            .client
            .log_message(MessageType::INFO, msg)
            .await;

        self.setup_capabilities().await;

        // Diagnostics are disabled by default, so update them after fetching workspace config
        // self.session.update_all_diagnostics().await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let _ = params;
        self.session.load_workspace_settings().await;
        self.setup_capabilities().await;
        // self.session.update_all_diagnostics().await;
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let file_paths = params
            .changes
            .iter()
            .map(|change| change.uri.to_file_path());
        for file_path in file_paths {
            match file_path {
                Ok(file_path) => {
                    let base_path = self.session.base_path();
                    if let Some(base_path) = base_path {
                        let possible_config_toml = file_path.strip_prefix(&base_path);
                        if let Ok(watched_file) = possible_config_toml {
                            if ConfigName::file_names()
                                .contains(&&*watched_file.display().to_string())
                            {
                                self.session.load_workspace_settings().await;
                                self.setup_capabilities().await;
                                // self.session.update_all_diagnostics().await;
                                // for now we are only interested to the configuration file,
                                // so it's OK to exist the loop
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    error!("The Workspace root URI {file_path:?} could not be parsed as a filesystem path");
                    continue;
                }
            }
        }
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handlers::text_document::did_open(&self.session, params)
            .await
            .ok();
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // handlers::text_document::did_change(&self.session, params)
        //     .await
        //     .ok();
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // handlers::text_document::did_save(&self.session, params)
        //     .await
        //     .ok();
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        handlers::text_document::did_close(&self.session, params)
            .await
            .ok();
    }
}

impl Drop for LSPServer {
    fn drop(&mut self) {
        if let Ok(mut sessions) = self.sessions.lock() {
            let _removed = sessions.remove(&self.session.key);
            debug_assert!(_removed.is_some(), "Session did not exist.");

            if self.stop_on_disconnect
                && sessions.is_empty()
                && self.is_initialized.load(Ordering::Relaxed)
            {
                self.session.cancellation.notify_one();
            }
        }
    }
}

/// Map of active sessions connected to a [ServerFactory].
type Sessions = Arc<Mutex<FxHashMap<SessionKey, SessionHandle>>>;

/// Helper method for wrapping a [Workspace] method in a `custom_method` for
/// the [LSPServer]
macro_rules! workspace_method {
    ( $builder:ident, $method:ident ) => {
        $builder = $builder.custom_method(
            concat!("pglsp/", stringify!($method)),
            |server: &LSPServer, params| {
                let span = tracing::trace_span!(concat!("pglsp/", stringify!($method)), params = ?params).or_current();
                tracing::info!("Received request: {}", stringify!($method));

                let workspace = server.session.workspace.clone();
                let result = spawn_blocking(move || {
                    let _guard = span.entered();
                    workspace.$method(params)
                });

                result.map(move |result| {
                    // The type of `result` is `Result<Result<R, RomeError>, JoinError>`,
                    // where the inner result is the return value of `$method` while the
                    // outer one is added by `spawn_blocking` to catch panics or
                    // cancellations of the task
                    match result {
                        Ok(Ok(result)) => Ok(result),
                        Ok(Err(err)) => Err(into_lsp_error(err)),
                        Err(err) => match err.try_into_panic() {
                            Ok(err) => Err(panic_to_lsp_error(err)),
                            Err(err) => Err(into_lsp_error(err)),
                        },
                    }
                })
            },
        );
    };
}

/// Factory data structure responsible for creating [ServerConnection] handles
/// for each incoming connection accepted by the server
#[derive(Default)]
pub struct ServerFactory {
    /// Synchronization primitive used to broadcast a shutdown signal to all
    /// active connections
    cancellation: Arc<Notify>,
    /// Optional [Workspace] instance shared between all clients. Currently
    /// this field is always [None] (meaning each connection will get its own
    /// workspace) until we figure out how to handle concurrent access to the
    /// same workspace from multiple client
    workspace: Option<Arc<dyn Workspace>>,

    /// The sessions of the connected clients indexed by session key.
    sessions: Sessions,

    /// Session key generator. Stores the key of the next session.
    next_session_key: AtomicU64,

    /// If this is true the server will broadcast a shutdown signal once the
    /// last client disconnected
    stop_on_disconnect: bool,
    /// This shared flag is set to true once at least one sessions has been
    /// initialized on this server instance
    is_initialized: Arc<AtomicBool>,
}

impl ServerFactory {
    pub fn new(stop_on_disconnect: bool) -> Self {
        Self {
            cancellation: Arc::default(),
            workspace: None,
            sessions: Sessions::default(),
            next_session_key: AtomicU64::new(0),
            stop_on_disconnect,
            is_initialized: Arc::default(),
        }
    }

    pub fn create(&self, config_path: Option<PathBuf>) -> ServerConnection {
        self.create_with_fs(config_path, DynRef::Owned(Box::<OsFileSystem>::default()))
    }

    /// Create a new [ServerConnection] from this factory
    pub fn create_with_fs(
        &self,
        config_path: Option<PathBuf>,
        fs: DynRef<'static, dyn FileSystem>,
    ) -> ServerConnection {
        let workspace = self
            .workspace
            .clone()
            .unwrap_or_else(workspace::server_sync);

        let session_key = SessionKey(self.next_session_key.fetch_add(1, Ordering::Relaxed));

        let mut builder = LspService::build(move |client| {
            let mut session = Session::new(
                session_key,
                client,
                workspace,
                self.cancellation.clone(),
                fs,
            );
            if let Some(path) = config_path {
                session.set_config_path(path);
            }
            let handle = Arc::new(session);

            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_key, handle.clone());

            LSPServer::new(
                handle,
                self.sessions.clone(),
                self.stop_on_disconnect,
                self.is_initialized.clone(),
            )
        });

        // "shutdown" is not part of the Workspace API
        builder = builder.custom_method("pglsp/shutdown", |server: &LSPServer, (): ()| {
            info!("Sending shutdown signal");
            server.session.broadcast_shutdown();
            ready(Ok(Some(())))
        });

        workspace_method!(builder, open_file);
        workspace_method!(builder, change_file);
        workspace_method!(builder, close_file);

        let (service, socket) = builder.finish();
        ServerConnection { socket, service }
    }

    /// Return a handle to the cancellation token for this server process
    pub fn cancellation(&self) -> Arc<Notify> {
        self.cancellation.clone()
    }
}

/// Handle type created by the server for each incoming connection
pub struct ServerConnection {
    socket: ClientSocket,
    service: LspService<LSPServer>,
}

impl ServerConnection {
    /// Destructure a connection into its inner service instance and socket
    pub fn into_inner(self) -> (LspService<LSPServer>, ClientSocket) {
        (self.service, self.socket)
    }

    /// Accept an incoming connection and run the server async I/O loop to
    /// completion
    pub async fn accept<I, O>(self, stdin: I, stdout: O)
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite,
    {
        Server::new(stdin, stdout, self.socket)
            .serve(self.service)
            .await;
    }
}