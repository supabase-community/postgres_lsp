use pg_configuration::ConfigurationDiagnostic;
use pg_diagnostics::{
    category, Advices, Category, Diagnostic, DiagnosticTags, Location, LogCategory,
    MessageAndDescription, Severity, Visit,
};
use pg_fs::FileSystemDiagnostic;
use std::process::{ExitCode, Termination};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use serde::{Deserialize, Serialize};


/// Generic errors thrown during operations
#[derive(Deserialize, Diagnostic, Serialize)]
pub enum WorkspaceError {
    /// Error thrown when validating the configuration. Once deserialized, further checks have to be done.
    Configuration(ConfigurationDiagnostic),
    /// Error when trying to access the database
    DatabaseConnectionError(DatabaseConnectionError),
    /// Diagnostics emitted when querying the file system
    FileSystem(FileSystemDiagnostic),
    /// Thrown when we can't read a generic directory
    CantReadDirectory(CantReadDirectory),
    /// Thrown when we can't read a generic file
    CantReadFile(CantReadFile),
    /// The file does not exist in the [crate::Workspace]
    NotFound(NotFound),
}

impl WorkspaceError {
    pub fn cant_read_file(path: String) -> Self {
        Self::CantReadFile(CantReadFile { path })
    }

    pub fn not_found() -> Self {
        Self::NotFound(NotFound)
    }
}

impl Error for WorkspaceError {}

impl Debug for WorkspaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for WorkspaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Diagnostic::description(self, f)
    }
}

impl Termination for WorkspaceError {
    fn report(self) -> ExitCode {
        ExitCode::FAILURE
    }
}

impl From<FileSystemDiagnostic> for WorkspaceError {
    fn from(err: FileSystemDiagnostic) -> Self {
        Self::FileSystem(err)
    }
}

impl From<ConfigurationDiagnostic> for WorkspaceError {
    fn from(err: ConfigurationDiagnostic) -> Self {
        Self::Configuration(err)
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "database/connection",
    message = "Error when trying to access the database",
)]
pub struct DatabaseConnectionError;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message = "The file does not exist in the workspace.",
    tags(INTERNAL)
)]
pub struct NotFound;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("We couldn't read the following directory, maybe for permissions reasons or it doesn't exist: "{self.path}),
        description = "We couldn't read the following directory, maybe for permissions reasons or it doesn't exist: {path}"
    )
)]
pub struct CantReadDirectory {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("We couldn't read the following file, maybe for permissions reasons or it doesn't exist: "{self.path}),
        description = "We couldn't read the following file, maybe for permissions reasons or it doesn't exist: {path}"
    )
)]
pub struct CantReadFile {
    #[location(resource)]
    path: String,
}
