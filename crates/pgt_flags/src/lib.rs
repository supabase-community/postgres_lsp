//! A simple implementation of feature flags.

use pgt_console::fmt::{Display, Formatter};
use pgt_console::{DebugDisplay, KeyValuePair, markup};
use std::env;
use std::ops::Deref;
use std::sync::{LazyLock, OnceLock};

/// Returns `true` if this is an unstable build of PgLT
pub fn is_unstable() -> bool {
    PGLT_VERSION.deref().is_none()
}

/// The internal version of PgLT. This is usually supplied during the CI build
pub static PGLT_VERSION: LazyLock<Option<&str>> = LazyLock::new(|| option_env!("PGLT_VERSION"));

pub struct PgLTEnv {
    pub pgt_log_path: PgLTEnvVariable,
    pub pgt_log_prefix: PgLTEnvVariable,
    pub pgt_config_path: PgLTEnvVariable,
}

pub static PGLT_ENV: OnceLock<PgLTEnv> = OnceLock::new();

impl PgLTEnv {
    fn new() -> Self {
        Self {
            pgt_log_path: PgLTEnvVariable::new(
                "PGLT_LOG_PATH",
                "The directory where the Daemon logs will be saved.",
            ),
            pgt_log_prefix: PgLTEnvVariable::new(
                "PGLT_LOG_PREFIX_NAME",
                "A prefix that's added to the name of the log. Default: `server.log.`",
            ),
            pgt_config_path: PgLTEnvVariable::new(
                "PGLT_CONFIG_PATH",
                "A path to the configuration file",
            ),
        }
    }
}

pub struct PgLTEnvVariable {
    /// The name of the environment variable
    name: &'static str,
    /// The description of the variable.
    // This field will be used in the website to automate its generation
    description: &'static str,
}

impl PgLTEnvVariable {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description }
    }

    /// It attempts to read the value of the variable
    pub fn value(&self) -> Option<String> {
        env::var(self.name).ok()
    }

    /// It returns the description of the variable
    pub fn description(&self) -> &'static str {
        self.description
    }

    /// It returns the name of the variable.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub fn pgt_env() -> &'static PgLTEnv {
    PGLT_ENV.get_or_init(PgLTEnv::new)
}

impl Display for PgLTEnv {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self.pgt_log_path.value() {
            None => {
                KeyValuePair(self.pgt_log_path.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pgt_log_path.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        };
        match self.pgt_log_prefix.value() {
            None => {
                KeyValuePair(self.pgt_log_prefix.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pgt_log_prefix.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        };

        match self.pgt_config_path.value() {
            None => {
                KeyValuePair(self.pgt_config_path.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pgt_config_path.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        Ok(())
    }
}
