use pg_console::fmt::Display;
use pg_console::{markup, MarkupBuf};
use pg_diagnostics::{Advices, Diagnostic, Error, LogCategory, MessageAndDescription, Visit};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Series of errors that can be thrown while computing the configuration.
#[derive(Deserialize, Diagnostic, Serialize)]
pub enum ConfigurationDiagnostic {
    /// Thrown when the program can't serialize the configuration, while saving it
    SerializationError(SerializationError),

    /// Error thrown when de-serialising the configuration from file
    DeserializationError(DeserializationError),

    /// Thrown when trying to **create** a new configuration file, but it exists already
    ConfigAlreadyExists(ConfigAlreadyExists),

    /// When something is wrong with the configuration
    InvalidConfiguration(InvalidConfiguration),

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(InvalidIgnorePattern),
}

impl ConfigurationDiagnostic {
    pub fn new_deserialization_error(error: toml::de::Error) -> Self {
        Self::DeserializationError(DeserializationError {
            message: error.message().to_string(),
        })
    }

    pub fn new_serialization_error() -> Self {
        Self::SerializationError(SerializationError)
    }

    pub fn new_invalid_ignore_pattern(
        pattern: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidIgnorePattern(InvalidIgnorePattern {
            message: format!(
                "Couldn't parse the pattern \"{}\". Reason: {}",
                pattern.into(),
                reason.into()
            ),
            file_path: None,
        })
    }

    pub fn new_invalid_ignore_pattern_with_path(
        pattern: impl Into<String>,
        reason: impl Into<String>,
        file_path: Option<impl Into<String>>,
    ) -> Self {
        Self::InvalidIgnorePattern(InvalidIgnorePattern {
            message: format!(
                "Couldn't parse the pattern \"{}\". Reason: {}",
                pattern.into(),
                reason.into()
            ),
            file_path: file_path.map(|f| f.into()),
        })
    }

    pub fn new_already_exists() -> Self {
        Self::ConfigAlreadyExists(ConfigAlreadyExists {})
    }

    pub fn invalid_configuration(message: impl Display) -> Self {
        Self::InvalidConfiguration(InvalidConfiguration {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
        })
    }
}

impl Debug for ConfigurationDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for ConfigurationDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigurationAdvices {
    messages: Vec<MarkupBuf>,
}

impl Advices for ConfigurationAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for message in &self.messages {
            visitor.record_log(LogCategory::Info, message)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "Failed to deserialize",
    category = "configuration",
    severity = Error
)]
pub struct DeserializationError {
    #[message]
    #[description]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "Failed to serialize",
    category = "configuration",
    severity = Error
)]
pub struct SerializationError;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "It seems that a configuration file already exists",
    category = "configuration",
    severity = Error
)]
pub struct ConfigAlreadyExists {}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct InvalidIgnorePattern {
    #[message]
    #[description]
    pub message: String,

    #[location(resource)]
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
	category = "configuration",
	severity = Error,
)]
pub struct InvalidConfiguration {
    #[message]
    #[description]
    message: MessageAndDescription,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct CantResolve {
    #[message]
    #[description]
    message: MessageAndDescription,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}
