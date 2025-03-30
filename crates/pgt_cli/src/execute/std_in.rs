//! In here, there are the operations that run via standard input
//!
use crate::{CliDiagnostic, CliSession};
use pgt_console::{ConsoleExt, markup};

pub(crate) fn run(session: CliSession, content: &str) -> Result<(), CliDiagnostic> {
    let console = &mut *session.app.console;

    console.append(markup! {{content}});
    Ok(())
}
