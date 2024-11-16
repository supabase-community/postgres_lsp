use pg_configuration::ConfigurationDiagnostic;
use pg_console::fmt::Bytes;
use pg_console::markup;
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
    /// Error emitted by the underlying transport layer for a remote Workspace
    TransportError(TransportError),
    /// Emitted when the file is ignored and should not be processed
    FileIgnored(FileIgnored),
    /// Emitted when a file could not be parsed because it's larger than the size limit
    FileTooLarge(FileTooLarge),
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

impl From<TransportError> for WorkspaceError {
    fn from(err: TransportError) -> Self {
        Self::TransportError(err)
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


#[derive(Debug, Serialize, Deserialize)]
/// Error emitted by the underlying transport layer for a remote Workspace
pub enum TransportError {
    /// Error emitted by the transport layer if the connection was lost due to an I/O error
    ChannelClosed,
    /// Error emitted by the transport layer if a request timed out
    Timeout,
    /// Error caused by a serialization or deserialization issue
    SerdeError(String),
    /// Generic error type for RPC errors that can't be deserialized into RomeError
    RPCError(String),
}

impl Display for TransportError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.description(fmt)
    }
}

impl Diagnostic for TransportError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransportError::SerdeError(err) => write!(fmt, "serialization error: {err}"),
            TransportError::ChannelClosed => fmt.write_str(
                "a request to the remote workspace failed because the connection was interrupted",
            ),
            TransportError::Timeout => {
                fmt.write_str("the request to the remote workspace timed out")
            }
            TransportError::RPCError(err) => fmt.write_str(err),
        }
    }

    fn message(&self, fmt: &mut pg_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            TransportError::SerdeError(err) => write!(fmt, "serialization error: {err}"),
            TransportError::ChannelClosed => fmt.write_str(
                "a request to the remote workspace failed because the connection was interrupted",
            ),
            TransportError::Timeout => {
                fmt.write_str("the request to the remote workspace timed out")
            }
            TransportError::RPCError(err) => fmt.write_str(err),
        }
    }
    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::INTERNAL
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

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    message(
        message("The file "{self.path}" was ignored."),
        description = "The file {path} was ignored."
    ),
    severity = Warning,
)]
pub struct FileIgnored {
    #[location(resource)]
    path: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FileTooLarge {
    path: String,
    size: usize,
    limit: usize,
}

impl Diagnostic for FileTooLarge {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/fs"))
    }

    fn message(&self, fmt: &mut pg_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(
            markup!{
                "Size of "{self.path}" is "{Bytes(self.size)}" which exceeds configured maximum of "{Bytes(self.limit)}" for this project.
                The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.
                Use the `files.maxSize` configuration to change the maximum size of files processed."
            }
        )
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "Size of {} is {} which exceeds configured maximum of {} for this project.\n\
               The file size limit exists to prevent us inadvertently slowing down and loading large files that we shouldn't.\n\
               Use the `files.maxSize` configuration to change the maximum size of files processed.",
               self.path, Bytes(self.size), Bytes(self.limit)
        )
    }
}

