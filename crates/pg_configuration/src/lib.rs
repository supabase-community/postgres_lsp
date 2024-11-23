//! This module contains the configuration of `pg.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.

pub mod files;
pub mod diagnostics;
pub mod vcs;

pub use crate::diagnostics::ConfigurationDiagnostic;

use std::path::PathBuf;

use crate::vcs::{partial_vcs_configuration, PartialVcsConfiguration, VcsConfiguration};
use bpaf::Bpaf;
use files::{partial_files_configuration, FilesConfiguration, PartialFilesConfiguration};
use serde::{Deserialize, Serialize};
use biome_deserialize_macros::Partial;
use vcs::VcsClientKind;

pub const VERSION: &str = match option_env!("PGLSP_VERSION") {
    Some(version) => version,
    None => "0.0.0",
};

/// The configuration that is contained inside the configuration file.
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Eq, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "snake_case"))]
pub struct Configuration {
    /// The configuration of the VCS integration
    #[partial(type, bpaf(external(partial_vcs_configuration), optional, hide_usage))]
    pub vcs: VcsConfiguration,

    /// The configuration of the filesystem
    #[partial(
        type,
        bpaf(external(partial_files_configuration), optional, hide_usage)
    )]
    pub files: FilesConfiguration,
}


impl PartialConfiguration {
    /// Returns the initial configuration.
    pub fn init() -> Self {
        Self {
            files: Some(PartialFilesConfiguration {
                ignore: Some(Default::default()),
                ..Default::default()
            }),
            vcs: Some(PartialVcsConfiguration {
                enabled: Some(false),
                client_kind: Some(VcsClientKind::Git),
                use_ignore_file: Some(false),
                ..Default::default()
            }),
        }
    }
}


pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: PartialConfiguration,
    /// The path of where the configuration file that was found. This contains the file name.
    pub configuration_file_path: PathBuf,
    /// The base path where the external configuration in a package should be resolved from
    pub external_resolution_base_path: PathBuf,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum ConfigurationPathHint {
    /// The default mode, not having a configuration file is not an error.
    /// The path will be filled with the working directory if it is not filled at the time of usage.
    #[default]
    None,

    /// Very similar to [ConfigurationPathHint::None]. However, the path provided by this variant
    /// will be used as **working directory**, which means that all globs defined in the configuration
    /// will use **this path** as base path.
    FromWorkspace(PathBuf),

    /// The configuration path provided by the LSP, not having a configuration file is not an error.
    /// The path will always be a directory path.
    FromLsp(PathBuf),
    /// The configuration path provided by the user, not having a configuration file is an error.
    /// The path can either be a directory path or a file path.
    /// Throws any kind of I/O errors.
    FromUser(PathBuf),
}

impl ConfigurationPathHint {
    pub const fn is_from_user(&self) -> bool {
        matches!(self, Self::FromUser(_))
    }
    pub const fn is_from_lsp(&self) -> bool {
        matches!(self, Self::FromLsp(_))
    }
}

