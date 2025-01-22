use crate::cli_options::CliOptions;
use crate::{CliDiagnostic, Execution, TraversalMode};
use pg_configuration::PartialConfiguration;
use pg_console::Console;
use pg_fs::FileSystem;
use pg_workspace::{configuration::LoadedConfiguration, DynRef, Workspace, WorkspaceError};
use std::ffi::OsString;

use super::{get_files_to_process_with_cli_options, CommandRunner};

pub(crate) struct CheckCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) configuration: Option<PartialConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

impl CommandRunner for CheckCommandPayload {
    const COMMAND_NAME: &'static str = "check";

    fn merge_configuration(
        &mut self,
        loaded_configuration: LoadedConfiguration,
        fs: &DynRef<'_, dyn FileSystem>,
        console: &mut dyn Console,
    ) -> Result<PartialConfiguration, WorkspaceError> {
        let LoadedConfiguration { configuration, .. } = loaded_configuration;

        Ok(configuration)
    }

    fn get_files_to_process(
        &self,
        fs: &DynRef<'_, dyn FileSystem>,
        configuration: &PartialConfiguration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        // update this to find migration files
        let paths = get_files_to_process_with_cli_options(
            self.since.as_deref(),
            self.changed,
            self.staged,
            fs,
            configuration,
        )?
        .unwrap_or(self.paths.clone());

        Ok(paths)
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Execution, CliDiagnostic> {
        Ok(Execution::new(TraversalMode::Check {
            stdin: self.get_stdin(console)?,
            vcs_targeted: (self.staged, self.changed).into(),
        })
        .set_report(cli_options))
    }
}
