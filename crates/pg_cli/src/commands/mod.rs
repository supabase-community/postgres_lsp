use crate::cli_options::{cli_options, CliOptions, ColorsArg};
use crate::diagnostics::DeprecatedConfigurationFile;
use crate::logging::LoggingKind;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, LoggingLevel, VERSION,
};
use pg_configuration::PartialConfiguration;
use pg_console::{markup, Console, ConsoleExt};
use pg_diagnostics::{Diagnostic, PrintDiagnostic};
use pg_fs::FileSystem;
use pg_workspace_new::configuration::{
    load_configuration, LoadedConfiguration,
};
use pg_workspace_new::settings::PartialConfigurationExt;
use pg_workspace_new::workspace::UpdateSettingsParams;
use pg_workspace_new::{DynRef, Workspace, WorkspaceError};
use bpaf::Bpaf;
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) mod clean;
pub(crate) mod daemon;
pub(crate) mod init;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
/// PgLsp official CLI. Use it to check the health of your project or run it to check single files.
pub enum PgLspCommand {
    /// Shows the Biome version information and quit.
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    /// Starts the Biome daemon server process.
    #[bpaf(command)]
    Start {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("PGLSP_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,

        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("PGLSP_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(pg_fs::ensure_cache_dir().join("pglsp-logs")),
        )]
        log_path: PathBuf,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `pglsp.toml`
        #[bpaf(env("PGLSP_LOG_PREFIX_NAME"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
    },

    /// Stops the daemon server process.
    #[bpaf(command)]
    Stop,

    /// Bootstraps a new project. Creates a configuration file with some defaults.
    #[bpaf(command)]
    Init,

    /// Acts as a server for the Language Server Protocol over stdin/stdout.
    #[bpaf(command("lsp-proxy"))]
    LspProxy {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("PGLSP_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,
        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("PGLSP_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(pg_fs::ensure_cache_dir().join("pglsp-logs")),
        )]
        log_path: PathBuf,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `pglsp.toml`
        #[bpaf(env("PGLSP_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
        /// Bogus argument to make the command work with vscode-languageclient
        #[bpaf(long("stdio"), hide, hide_usage, switch)]
        stdio: bool,
    },

    #[bpaf(command)]
    /// Cleans the logs emitted by the daemon.
    Clean,

    #[bpaf(command("__run_server"), hide)]
    RunServer {
        /// Allows to change the prefix applied to the file name of the logs.
        #[bpaf(
            env("PGLSP_LOG_PREFIX_NAME"),
            long("log-prefix-name"),
            argument("STRING"),
            hide_usage,
            fallback(String::from("server.log")),
            display_fallback
        )]
        log_prefix_name: String,
        /// Allows to change the folder where logs are stored.
        #[bpaf(
            env("PGLSP_LOG_PATH"),
            long("log-path"),
            argument("PATH"),
            hide_usage,
            fallback(pg_fs::ensure_cache_dir().join("pglsp-logs")),
        )]
        log_path: PathBuf,

        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,
        /// Allows to set a custom file path to the configuration file,
        /// or a custom directory path to find `pglsp.toml`
        #[bpaf(env("PGLSP_CONFIG_PATH"), long("config-path"), argument("PATH"))]
        config_path: Option<PathBuf>,
    },
    #[bpaf(command("__print_socket"), hide)]
    PrintSocket,
}

impl PgLspCommand {
    const fn cli_options(&self) -> Option<&CliOptions> {
        match self {
            PgLspCommand::Version(cli_options) => Some(cli_options),
            PgLspCommand::LspProxy { .. }
            | PgLspCommand::Start { .. }
            | PgLspCommand::Stop
            | PgLspCommand::Init
            | PgLspCommand::RunServer { .. }
            | PgLspCommand::Clean { .. }
            | PgLspCommand::PrintSocket => None,
        }
    }

    pub const fn get_color(&self) -> Option<&ColorsArg> {
        match self.cli_options() {
            Some(cli_options) => {
                // To properly display GitHub annotations we need to disable colors
                // if matches!(cli_options.reporter, CliReporter::GitHub) {
                //     return Some(&ColorsArg::Off);
                // }
                // We want force colors in CI, to give e better UX experience
                // Unless users explicitly set the colors flag
                // if matches!(self, PgLspCommand::Ci { .. }) && cli_options.colors.is_none() {
                //     return Some(&ColorsArg::Force);
                // }
                // Normal behaviors
                cli_options.colors.as_ref()
            }
            None => None,
        }
    }

    pub const fn should_use_server(&self) -> bool {
        match self.cli_options() {
            Some(cli_options) => cli_options.use_server,
            None => false,
        }
    }

    pub const fn has_metrics(&self) -> bool {
        false
    }

    pub fn is_verbose(&self) -> bool {
        self.cli_options()
            .map_or(false, |cli_options| cli_options.verbose)
    }

    pub fn log_level(&self) -> LoggingLevel {
        self.cli_options()
            .map_or(LoggingLevel::default(), |cli_options| cli_options.log_level)
    }

    pub fn log_kind(&self) -> LoggingKind {
        self.cli_options()
            .map_or(LoggingKind::default(), |cli_options| cli_options.log_kind)
    }
}

/// It accepts a [LoadedPartialConfiguration] and it prints the diagnostics emitted during parsing and deserialization.
///
/// If it contains [errors](Severity::Error) or higher, it returns an error.
pub(crate) fn validate_configuration_diagnostics(
    loaded_configuration: &LoadedConfiguration,
    console: &mut dyn Console,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    if let Some(file_path) = loaded_configuration
        .file_path
        .as_ref()
        .and_then(|f| f.file_name())
        .and_then(|f| f.to_str())
    {
        if file_path == "rome.json" {
            let diagnostic = DeprecatedConfigurationFile::new(file_path);
            if diagnostic.tags().is_verbose() && verbose {
                console.error(markup! {{PrintDiagnostic::verbose(&diagnostic)}})
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}})
            }
        }
    }

    // let diagnostics = loaded_configuration.as_diagnostics_iter();
    // for diagnostic in diagnostics {
    //     if diagnostic.tags().is_verbose() && verbose {
    //         console.error(markup! {{PrintDiagnostic::verbose(diagnostic)}})
    //     } else {
    //         console.error(markup! {{PrintDiagnostic::simple(diagnostic)}})
    //     }
    // }
    //
    // if loaded_configuration.has_errors() {
    //     return Err(CliDiagnostic::workspace_error(
    //         ConfigurationDiagnostic::invalid_configuration(
    //             "Exited because the configuration resulted in errors. Please fix them.",
    //         )
    //         .into(),
    //     ));
    // }

    Ok(())
}

/// Generic interface for executing commands.
///
/// Consumers must implement the following methods:
///
/// - [CommandRunner::merge_configuration]
/// - [CommandRunner::get_files_to_process]
/// - [CommandRunner::get_stdin_file_path]
/// - [CommandRunner::should_write]
/// - [CommandRunner::get_execution]
///
/// Optional methods:
/// - [CommandRunner::check_incompatible_arguments]
pub(crate) trait CommandRunner: Sized {
    const COMMAND_NAME: &'static str;

    /// The main command to use.
    fn run(&mut self, session: CliSession, cli_options: &CliOptions) -> Result<(), CliDiagnostic> {
        setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);
        let fs = &session.app.fs;
        let console = &mut *session.app.console;
        let workspace = &*session.app.workspace;
        self.check_incompatible_arguments()?;
        let (execution, paths) = self.configure_workspace(fs, console, workspace, cli_options)?;
        execute_mode(execution, session, cli_options, paths)
    }

    /// This function prepares the workspace with the following:
    /// - Loading the configuration file.
    /// - Configure the VCS integration
    /// - Computes the paths to traverse/handle. This changes based on the VCS arguments that were passed.
    /// - Register a project folder using the working directory.
    /// - Resolves the closets manifest AKA `package.json` and registers it.
    /// - Updates the settings that belong to the project registered
    fn configure_workspace(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
        cli_options: &CliOptions,
    ) -> Result<(Execution, Vec<OsString>), CliDiagnostic> {
        let loaded_configuration =
            load_configuration(fs, cli_options.as_configuration_path_hint())?;
        if self.should_validate_configuration_diagnostics() {
            validate_configuration_diagnostics(
                &loaded_configuration,
                console,
                cli_options.verbose,
            )?;
        }
        let configuration_path = loaded_configuration.directory_path.clone();
        let configuration = self.merge_configuration(loaded_configuration, fs, console)?;
        let vcs_base_path = configuration_path.or(fs.working_directory());
        let (vcs_base_path, gitignore_matches) =
            configuration.retrieve_gitignore_matches(fs, vcs_base_path.as_deref())?;
        let paths = self.get_files_to_process(fs, &configuration)?;

        workspace.update_settings(UpdateSettingsParams {
            workspace_directory: fs.working_directory(),
            configuration,
            vcs_base_path,
            gitignore_matches
        })?;

        let execution = self.get_execution(cli_options, console, workspace)?;
        Ok((execution, paths))
    }

    // Below, the methods that consumers must implement.

    /// Implements this method if you need to merge CLI arguments to the loaded configuration.
    ///
    /// The CLI arguments take precedence over the option configured in the configuration file.
    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError>;

    /// It returns the paths that need to be handled/traversed.
    fn get_files_to_process(
        &self,
        fs: &DynRef<'_, dyn FileSystem>,
        configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic>;

    /// It returns the file path to use in `stdin` mode.
    fn get_stdin_file_path(&self) -> Option<&str>;

    /// Whether the command should write the files.
    fn should_write(&self) -> bool;

    /// Returns the [Execution] mode.
    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic>;

    // Below, methods that consumers can implement

    /// Optional method that can be implemented to check if some CLI arguments aren't compatible.
    ///
    /// The method is called before loading the configuration from disk.
    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        Ok(())
    }

    /// Checks whether the configuration has errors.
    fn should_validate_configuration_diagnostics(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that all CLI options adhere to the invariants expected by `bpaf`.
    #[test]
    fn check_options() {
        pg_lsp_command().check_invariants(false);
    }
}

