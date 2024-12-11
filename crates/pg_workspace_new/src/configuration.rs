use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use pg_configuration::{
    ConfigurationDiagnostic, ConfigurationPathHint, ConfigurationPayload, PartialConfiguration,
};
use pg_fs::{AutoSearchResult, ConfigName, FileSystem, OpenOptions};

use crate::{DynRef, WorkspaceError};

/// Information regarding the configuration that was found.
///
/// This contains the expanded configuration including default values where no
/// configuration was present.
#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    /// If present, the path of the directory where it was found
    pub directory_path: Option<PathBuf>,
    /// If present, the path of the file where it was found
    pub file_path: Option<PathBuf>,
    /// The Deserialized configuration
    pub configuration: PartialConfiguration,
}

impl LoadedConfiguration {
    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Path> {
        self.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }
}

impl From<Option<ConfigurationPayload>> for LoadedConfiguration {
    fn from(value: Option<ConfigurationPayload>) -> Self {
        let Some(value) = value else {
            return LoadedConfiguration::default();
        };

        let ConfigurationPayload {
            configuration_file_path,
            deserialized: partial_configuration,
            ..
        } = value;

        LoadedConfiguration {
            configuration: partial_configuration,
            directory_path: configuration_file_path.parent().map(PathBuf::from),
            file_path: Some(configuration_file_path),
        }
    }
}

/// Load the partial configuration for this session of the CLI.
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationPathHint,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    Ok(LoadedConfiguration::from(config))
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [path hint](ConfigurationPathHint) should be provided.
fn load_config(
    file_system: &DynRef<'_, dyn FileSystem>,
    base_path: ConfigurationPathHint,
) -> LoadConfig {
    // This path is used for configuration resolution from external packages.
    let external_resolution_base_path = match base_path {
        // Path hint from LSP is always the workspace root
        // we use it as the resolution base path.
        ConfigurationPathHint::FromLsp(ref path) => path.clone(),
        ConfigurationPathHint::FromWorkspace(ref path) => path.clone(),
        // Path hint from user means the command is invoked from the CLI
        // So we use the working directory (CWD) as the resolution base path
        ConfigurationPathHint::FromUser(_) | ConfigurationPathHint::None => file_system
            .working_directory()
            .map_or(PathBuf::new(), |working_directory| working_directory),
    };

    // If the configuration path hint is from user and is a file path,
    // we'll load it directly
    if let ConfigurationPathHint::FromUser(ref config_file_path) = base_path {
        if file_system.path_is_file(config_file_path) {
            let content = file_system.read_file_from_path(config_file_path)?;

            let deserialized = toml::from_str::<PartialConfiguration>(&content)
                .map_err(ConfigurationDiagnostic::new_deserialization_error)?;

            return Ok(Some(ConfigurationPayload {
                deserialized,
                configuration_file_path: PathBuf::from(config_file_path),
                external_resolution_base_path,
            }));
        }
    }

    // If the configuration path hint is not a file path
    // we'll auto search for the configuration file
    let should_error = base_path.is_from_user();
    let configuration_directory = match base_path {
        ConfigurationPathHint::FromLsp(path) => path,
        ConfigurationPathHint::FromUser(path) => path,
        ConfigurationPathHint::FromWorkspace(path) => path,
        ConfigurationPathHint::None => file_system.working_directory().unwrap_or_default(),
    };

    // We first search for `pgtoml.json`
    if let Some(auto_search_result) = file_system.auto_search(
        &configuration_directory,
        ConfigName::file_names().as_slice(),
        should_error,
    )? {
        let AutoSearchResult { content, file_path } = auto_search_result;

        let deserialized = toml::from_str::<PartialConfiguration>(&content)
            .map_err(ConfigurationDiagnostic::new_deserialization_error)?;

        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: file_path,
            external_resolution_base_path,
        }))
    } else {
        Ok(None)
    }
}

/// Creates a new configuration on file system
///
/// ## Errors
///
/// It fails if:
/// - the configuration file already exists
/// - the program doesn't have the write rights
pub fn create_config(
    fs: &mut DynRef<dyn FileSystem>,
    configuration: PartialConfiguration,
) -> Result<(), WorkspaceError> {
    let path = PathBuf::from(ConfigName::pglsp_toml());

    if fs.path_exists(&path) {
        return Err(ConfigurationDiagnostic::new_already_exists().into());
    }

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            ConfigurationDiagnostic::new_already_exists().into()
        } else {
            WorkspaceError::cant_read_file(format!("{}", path.display()))
        }
    })?;

    let contents = toml::ser::to_string_pretty(&configuration)
        .map_err(|_| ConfigurationDiagnostic::new_serialization_error())?;

    config_file
        .set_content(contents.as_bytes())
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path.display())))?;

    Ok(())
}
