//! A simple implementation of feature flags.

use pg_console::fmt::{Display, Formatter};
use pg_console::{markup, DebugDisplay, KeyValuePair};
use std::env;
use std::ops::Deref;
use std::sync::{LazyLock, OnceLock};

/// Returns `true` if this is an unstable build of PgLsp
pub fn is_unstable() -> bool {
    PGLSP_VERSION.deref().is_none()
}

/// The internal version of PgLsp. This is usually supplied during the CI build
pub static PGLSP_VERSION: LazyLock<Option<&str>> = LazyLock::new(|| option_env!("PGLSP_VERSION"));

pub struct PgLspEnv {
    pub pglsp_log_path: PgLspEnvVariable,
    pub pglsp_log_prefix: PgLspEnvVariable,
    pub pglsp_config_path: PgLspEnvVariable,
}

pub static PGLSP_ENV: OnceLock<PgLspEnv> = OnceLock::new();

impl PgLspEnv {
    fn new() -> Self {
        Self {
            pglsp_log_path: PgLspEnvVariable::new(
                "BIOME_LOG_PATH",
                "The directory where the Daemon logs will be saved.",
            ),
            pglsp_log_prefix: PgLspEnvVariable::new(
                "BIOME_LOG_PREFIX_NAME",
                "A prefix that's added to the name of the log. Default: `server.log.`",
            ),
            pglsp_config_path: PgLspEnvVariable::new(
                "BIOME_CONFIG_PATH",
                "A path to the configuration file",
            ),
        }
    }
}

pub struct PgLspEnvVariable {
    /// The name of the environment variable
    name: &'static str,
    /// The description of the variable.
    // This field will be used in the website to automate its generation
    description: &'static str,
}

impl PgLspEnvVariable {
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

pub fn pglsp_env() -> &'static PgLspEnv {
    PGLSP_ENV.get_or_init(PgLspEnv::new)
}

impl Display for PgLspEnv {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self.pglsp_log_path.value() {
            None => {
                KeyValuePair(self.pglsp_log_path.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pglsp_log_path.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        };
        match self.pglsp_log_prefix.value() {
            None => {
                KeyValuePair(self.pglsp_log_prefix.name, markup! { <Dim>"unset"</Dim> })
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pglsp_log_prefix.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        match self.pglsp_config_path.value() {
            None => {
                KeyValuePair(self.pglsp_config_path.name, markup! { <Dim>"unset"</Dim> })
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.pglsp_config_path.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        Ok(())
    }
}
