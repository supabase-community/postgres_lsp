use crate::{CliDiagnostic, CliSession};
use pglt_configuration::PartialConfiguration;
use pglt_console::{ConsoleExt, markup};
use pglt_fs::ConfigName;
use pglt_workspace::configuration::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let fs = &mut session.app.fs;
    create_config(fs, PartialConfiguration::init())?;
    let file_created = ConfigName::pglt_json();
    session.app.console.log(markup! {
"
Welcome to the Postgres Language Tools! Let's get you started...

"<Info><Emphasis>"Files created "</Emphasis></Info>"

  "<Dim>"- "</Dim><Emphasis>{file_created}</Emphasis>"
    Your project configuration.
"
    });
    Ok(())
}
