use crate::changed::{get_changed_files, get_staged_files};
use crate::cli_options::{cli_options, CliOptions, CliReporter, ColorsArg};
use crate::execute::Stdin;
use crate::logging::LoggingKind;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, LoggingLevel, VERSION,
};
use bpaf::Bpaf;
use pg_configuration::{partial_configuration, PartialConfiguration};
use pg_console::Console;
use pg_fs::FileSystem;
use pg_workspace_new::configuration::{load_configuration, LoadedConfiguration};
use pg_workspace_new::settings::PartialConfigurationExt;
use pg_workspace_new::workspace::{FixFileMode, UpdateSettingsParams};
use pg_workspace_new::{DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) mod check;
pub(crate) mod clean;
pub(crate) mod daemon;
pub(crate) mod init;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
/// PgLsp official CLI. Use it to check the health of your project or run it to check single files.
pub enum PgLspCommand {
    /// Shows the version information and quit.
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    /// Runs everything to the requested files.
    #[bpaf(command)]
    Check {
        /// Writes safe fixes, formatting and import sorting
        #[bpaf(long("write"), switch)]
        write: bool,

        /// Allow to do unsafe fixes, should be used with `--write` or `--fix`
        #[bpaf(long("unsafe"), switch)]
        unsafe_: bool,

        /// Alias for `--write`, writes safe fixes, formatting and import sorting
        #[bpaf(long("fix"), switch, hide_usage)]
        fix: bool,

        #[bpaf(external(partial_configuration), hide_usage, optional)]
        configuration: Option<PartialConfiguration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Use this option when you want to format code piped from `stdin`, and print the output to `stdout`.
        ///
        /// The file doesn't need to exist on disk, what matters is the extension of the file. Based on the extension, we know how to check the code.
        ///
        /// Example: `echo 'let a;' | pg_cli check --stdin-file-path=test.sql`
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        /// When set to true, only the files that have been staged (the ones prepared to be committed)
        /// will be linted. This option should be used when working locally.
        #[bpaf(long("staged"), switch)]
        staged: bool,

        /// When set to true, only the files that have been changed compared to your `defaultBranch`
        /// configuration will be linted. This option should be used in CI environments.
        #[bpaf(long("changed"), switch)]
        changed: bool,

        /// Use this to specify the base branch to compare against when you're using the --changed
        /// flag and the `defaultBranch` is not set in your `pglsp.toml`
        #[bpaf(long("since"), argument("REF"))]
        since: Option<String>,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Starts the daemon server process.
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
            PgLspCommand::Version(cli_options) | PgLspCommand::Check { cli_options, .. } => {
                Some(cli_options)
            }
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
                if matches!(cli_options.reporter, CliReporter::GitHub) {
                    return Some(&ColorsArg::Off);
                }
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
            .is_some_and(|cli_options| cli_options.verbose)
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
            gitignore_matches,
        })?;

        let execution = self.get_execution(cli_options, console, workspace)?;
        Ok((execution, paths))
    }

    /// Computes [Stdin] if the CLI has the necessary information.
    ///
    /// ## Errors
    /// - If the user didn't provide anything via `stdin` but the option `--stdin-file-path` is passed.
    fn get_stdin(&self, console: &mut dyn Console) -> Result<Option<Stdin>, CliDiagnostic> {
        let stdin = if let Some(stdin_file_path) = self.get_stdin_file_path() {
            let input_code = console.read();
            if let Some(input_code) = input_code {
                let path = PathBuf::from(stdin_file_path);
                Some((path, input_code).into())
            } else {
                // we provided the argument without a piped stdin, we bail
                return Err(CliDiagnostic::missing_argument("stdin", Self::COMMAND_NAME));
            }
        } else {
            None
        };

        Ok(stdin)
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

fn get_files_to_process_with_cli_options(
    since: Option<&str>,
    changed: bool,
    staged: bool,
    fs: &DynRef<'_, dyn FileSystem>,
    configuration: &PartialConfiguration,
) -> Result<Option<Vec<OsString>>, CliDiagnostic> {
    if since.is_some() {
        if !changed {
            return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
        }
        if staged {
            return Err(CliDiagnostic::incompatible_arguments("since", "staged"));
        }
    }

    if changed {
        if staged {
            return Err(CliDiagnostic::incompatible_arguments("changed", "staged"));
        }
        Ok(Some(get_changed_files(fs, configuration, since)?))
    } else if staged {
        Ok(Some(get_staged_files(fs)?))
    } else {
        Ok(None)
    }
}

/// Holds the options to determine the fix file mode.
pub(crate) struct FixFileModeOptions {
    write: bool,
    suppress: bool,
    suppression_reason: Option<String>,
    fix: bool,
    unsafe_: bool,
}

/// - [Result]: if the given options are incompatible
/// - [Option]: if no fixes are requested
/// - [FixFileMode]: if safe or unsafe fixes are requested
pub(crate) fn determine_fix_file_mode(
    options: FixFileModeOptions,
    console: &mut dyn Console,
) -> Result<Option<FixFileMode>, CliDiagnostic> {
    let FixFileModeOptions {
        write,
        fix,
        suppress,
        suppression_reason: _,
        unsafe_,
    } = options;

    check_fix_incompatible_arguments(options)?;

    let safe_fixes = write || fix;
    let unsafe_fixes = (write || safe_fixes) && unsafe_;

    if unsafe_fixes {
        Ok(Some(FixFileMode::SafeAndUnsafeFixes))
    } else if safe_fixes {
        Ok(Some(FixFileMode::SafeFixes))
    } else if suppress {
        Ok(Some(FixFileMode::ApplySuppressions))
    } else {
        Ok(None)
    }
}

/// Checks if the fix file options are incompatible.
fn check_fix_incompatible_arguments(options: FixFileModeOptions) -> Result<(), CliDiagnostic> {
    let FixFileModeOptions {
        write,
        suppress,
        suppression_reason,
        fix,
        ..
    } = options;
    if write && fix {
        return Err(CliDiagnostic::incompatible_arguments("--write", "--fix"));
    } else if suppress && write {
        return Err(CliDiagnostic::incompatible_arguments(
            "--suppress",
            "--write",
        ));
    } else if suppress && fix {
        return Err(CliDiagnostic::incompatible_arguments("--suppress", "--fix"));
    } else if !suppress && suppression_reason.is_some() {
        return Err(CliDiagnostic::unexpected_argument(
            "--reason",
            "`--reason` is only valid when `--suppress` is used.",
        ));
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_console::BufferConsole;

    #[test]
    fn incompatible_arguments() {
        {
            let (write, suppress, suppression_reason, fix, unsafe_) =
                (true, false, None, true, false);
            assert!(check_fix_incompatible_arguments(FixFileModeOptions {
                write,
                suppress,
                suppression_reason,
                fix,
                unsafe_
            })
            .is_err());
        }
    }

    #[test]
    fn safe_fixes() {
        let mut console = BufferConsole::default();

        for (write, suppress, suppression_reason, fix, unsafe_) in [
            (true, false, None, false, false), // --write
            (false, false, None, true, false), // --fix
        ] {
            assert_eq!(
                determine_fix_file_mode(
                    FixFileModeOptions {
                        write,
                        suppress,
                        suppression_reason,
                        fix,
                        unsafe_
                    },
                    &mut console
                )
                .unwrap(),
                Some(FixFileMode::SafeFixes)
            );
        }
    }

    #[test]
    fn safe_and_unsafe_fixes() {
        let mut console = BufferConsole::default();

        for (write, suppress, suppression_reason, fix, unsafe_) in [
            (true, false, None, false, true), // --write --unsafe
            (false, false, None, true, true), // --fix --unsafe
        ] {
            assert_eq!(
                determine_fix_file_mode(
                    FixFileModeOptions {
                        write,
                        suppress,
                        suppression_reason,
                        fix,
                        unsafe_
                    },
                    &mut console
                )
                .unwrap(),
                Some(FixFileMode::SafeAndUnsafeFixes)
            );
        }
    }

    #[test]
    fn no_fix() {
        let mut console = BufferConsole::default();

        let (write, suppress, suppression_reason, fix, unsafe_) =
            (false, false, None, false, false);
        assert_eq!(
            determine_fix_file_mode(
                FixFileModeOptions {
                    write,
                    suppress,
                    suppression_reason,
                    fix,
                    unsafe_
                },
                &mut console
            )
            .unwrap(),
            None
        );
    }

    /// Tests that all CLI options adhere to the invariants expected by `bpaf`.
    #[test]
    fn check_options() {
        pg_lsp_command().check_invariants(false);
    }
}
