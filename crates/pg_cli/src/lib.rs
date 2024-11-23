//! # Module
//!
//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that was passed.

use pg_console::{markup, ColorMode, Console, ConsoleExt};
use pg_fs::OsFileSystem;
use pg_workspace_new::{App, DynRef, Workspace, WorkspaceRef};
use std::env;

mod logging;
mod cli_options;
mod commands;
mod metrics;
mod panic;
mod execute;
mod reporter;
mod diagnostics;
mod service;

use crate::commands::CommandRunner;
use crate::cli_options::{CliOptions, ColorsArg};
pub use execute::{execute_mode, Execution, TraversalMode, VcsTargeted};
pub use panic::setup_panic_handler;
pub use reporter::{DiagnosticsPayload, Reporter, ReporterVisitor, TraversalSummary};
pub use crate::logging::{setup_cli_subscriber, LoggingLevel};
pub use service::{open_transport, SocketTransport};
pub use diagnostics::CliDiagnostic;
pub use crate::commands::{pg_lsp_command, PgLspCommand};

pub(crate) const VERSION: &str = match option_env!("PGLSP_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// Global context for an execution of the CLI
pub struct CliSession<'app> {
    /// Instance of [App] used by this run of the CLI
    pub app: App<'app>,
}

impl<'app> CliSession<'app> {
    pub fn new(
        workspace: &'app dyn Workspace,
        console: &'app mut dyn Console,
    ) -> Result<Self, CliDiagnostic> {
        Ok(Self {
            app: App::new(
                DynRef::Owned(Box::<OsFileSystem>::default()),
                console,
                WorkspaceRef::Borrowed(workspace),
            ),
        })
    }

    /// Main function to run the CLI
    pub fn run(self, command: PgLspCommand) -> Result<(), CliDiagnostic> {
        let has_metrics = command.has_metrics();
        if has_metrics {
            crate::metrics::init_metrics();
        }

        let result = match command {
            PgLspCommand::Version(_) => commands::version::full_version(self),
            PgLspCommand::Clean => commands::clean::clean(self),
            PgLspCommand::Start {
                config_path,
                log_path,
                log_prefix_name,
            } => commands::daemon::start(self, config_path, Some(log_path), Some(log_prefix_name)),
            PgLspCommand::Stop => commands::daemon::stop(self),
            PgLspCommand::Init => commands::init::init(self),
            PgLspCommand::LspProxy {
                config_path,
                log_path,
                log_prefix_name,
                ..
            } => commands::daemon::lsp_proxy(config_path, Some(log_path), Some(log_prefix_name)),
            PgLspCommand::RunServer {
                stop_on_disconnect,
                config_path,
                log_path,
                log_prefix_name,
            } => commands::daemon::run_server(
                stop_on_disconnect,
                config_path,
                Some(log_path),
                Some(log_prefix_name),
            ),
            PgLspCommand::PrintSocket => commands::daemon::print_socket(),
        };

        if has_metrics {
            metrics::print_metrics();
        }

        result
    }
}

pub fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
    match color {
        Some(ColorsArg::Off) => ColorMode::Disabled,
        Some(ColorsArg::Force) => ColorMode::Enabled,
        None => ColorMode::Auto,
    }
}

pub(crate) fn run_command(
    session: CliSession,
    cli_options: &CliOptions,
    mut command: impl CommandRunner,
) -> Result<(), CliDiagnostic> {
    let command = &mut command;
    command.run(session, cli_options)
}

