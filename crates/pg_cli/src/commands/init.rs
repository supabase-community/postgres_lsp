use crate::{CliDiagnostic, CliSession};
use pg_configuration::PartialConfiguration;
use pg_console::{markup, ConsoleExt};
use pg_fs::ConfigName;
use pg_workspace_new::configuration::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let fs = &mut session.app.fs;
    create_config(fs, PartialConfiguration::init())?;
    let file_created = ConfigName::pglsp_toml();
    session.app.console.log(markup! {
"
Welcome to the Postgres Language Server! Let's get you started...

"<Info><Emphasis>"Files created "</Emphasis></Info>"

  "<Dim>"- "</Dim><Emphasis>{file_created}</Emphasis>"
    Your project configuration.
"
    });
    Ok(())
}
