use std::{io::ErrorKind, iter::FusedIterator, path::{Path, PathBuf}};

use pg_configuration::{ConfigurationPathHint, ConfigurationPayload, PartialConfiguration, ConfigurationDiagnostic};
use pg_diagnostics::{Error, Severity};
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
    /// All diagnostics that were emitted during parsing and deserialization
    pub diagnostics: Vec<Error>,
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

    /// Whether the are errors emitted. Error are [Severity::Error] or greater.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It return an iterator over the diagnostics emitted during the resolution of the configuration file
    pub fn as_diagnostics_iter(&self) -> ConfigurationDiagnosticsIter {
        ConfigurationDiagnosticsIter::new(self.diagnostics.as_slice())
    }
}


pub struct ConfigurationDiagnosticsIter<'a> {
    errors: &'a [Error],
    len: usize,
    index: usize,
}

impl<'a> ConfigurationDiagnosticsIter<'a> {
    fn new(errors: &'a [Error]) -> Self {
        Self {
            len: errors.len(),
            index: 0,
            errors,
        }
    }
}

impl<'a> Iterator for ConfigurationDiagnosticsIter<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == self.index {
            return None;
        }

        let item = self.errors.get(self.index);
        self.index += 1;
        item
    }
}

impl FusedIterator for ConfigurationDiagnosticsIter<'_> {}

impl LoadedConfiguration {
    fn try_from_payload(
        value: Option<ConfigurationPayload>,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Self, WorkspaceError> {
        let Some(value) = value else {
            return Ok(LoadedConfiguration::default());
        };

        let ConfigurationPayload {
            external_resolution_base_path,
            configuration_file_path,
            deserialized,
        } = value;
        let (partial_configuration, mut diagnostics) = deserialized.consume();

        Ok(Self {
            configuration: match partial_configuration {
                Some(mut partial_configuration) => {
                    partial_configuration.apply_extends(
                        fs,
                        &configuration_file_path,
                        &external_resolution_base_path,
                        &mut diagnostics,
                    )?;
                    partial_configuration
                }
                None => PartialConfiguration::default(),
            },
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| {
                    diagnostic.with_file_path(configuration_file_path.display().to_string())
                })
                .collect(),
            directory_path: configuration_file_path.parent().map(PathBuf::from),
            file_path: Some(configuration_file_path),
        })
    }
}

// TODO: implement toml serialization and deserialization and merge with default supabase config

/// Load the partial configuration for this session of the CLI.
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationPathHint,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    LoadedConfiguration::try_from_payload(config, fs)
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [path hint](ConfigurationPathHint) should be provided.
///
/// - If the path hint is a path to a file that is provided by the user, the function will try to load that file or error.
///     The name doesn't have to be `biome.json` or `biome.jsonc`. And if it doesn't end with `.json`, Biome will try to
///     deserialize it as a `.jsonc` file.
///
/// - If the path hint is a path to a directory which is provided by the user, the function will try to find a `biome.json`
///     or `biome.jsonc` file in order in that directory. And If it cannot find one, it will error.
///
/// - Otherwise, the function will try to traverse upwards the file system until it finds a `biome.json` or `biome.jsonc`
///     file, or there aren't directories anymore. In this case, the function will not error but return an `Ok(None)`, which
///     means Biome will use the default configuration.
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
            let parser_options = match config_file_path.extension().map(OsStr::as_encoded_bytes) {
                Some(b"json") => JsonParserOptions::default(),
                _ => JsonParserOptions::default()
                    .with_allow_comments()
                    .with_allow_trailing_commas(),
            };
            let deserialized =
                deserialize_from_json_str::<PartialConfiguration>(&content, parser_options, "");
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

    // We first search for `biome.json` or `biome.jsonc` files
    if let Some(auto_search_result) = match file_system.auto_search(
        &configuration_directory,
        ConfigName::file_names().as_slice(),
        should_error,
    ) {
        Ok(Some(auto_search_result)) => Some(auto_search_result),
        // We then search for the deprecated `rome.json` file
        // if neither `biome.json` nor `biome.jsonc` is found
        // TODO: The following arms should be removed in v2.0.0
        Ok(None) => file_system.auto_search(
            &configuration_directory,
            [file_system.deprecated_config_name()].as_slice(),
            should_error,
        )?,
        Err(error) => file_system
            .auto_search(
                &configuration_directory,
                [file_system.deprecated_config_name()].as_slice(),
                should_error,
            )
            // Map the error so users won't see error messages
            // that contains `rome.json`
            .map_err(|_| error)?,
    } {
        let AutoSearchResult { content, file_path } = auto_search_result;

        let parser_options = match file_path.extension().map(OsStr::as_encoded_bytes) {
            Some(b"json") => JsonParserOptions::default(),
            _ => JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
        };

        let deserialized =
            deserialize_from_json_str::<PartialConfiguration>(&content, parser_options, "");

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
    mut configuration: PartialConfiguration,
    emit_jsonc: bool,
) -> Result<(), WorkspaceError> {
    let toml_path = PathBuf::from(ConfigName::pg_toml());

    if fs.path_exists(&toml_path) {
        return Err(ConfigurationDiagnostic::new_already_exists().into());
    }

    let path = if emit_jsonc { jsonc_path } else { json_path };

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            ConfigurationDiagnostic::new_already_exists().into()
        } else {
            WorkspaceError::cant_read_file(format!("{}", path.display()))
        }
    })?;

    let contents = serde_json::to_string_pretty(&configuration)
        .map_err(|_| ConfigurationDiagnostic::new_serialization_error())?;

    let parsed = parse_json(&contents, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())?
            .print()
            .expect("valid format document");

    config_file
        .set_content(formatted.as_code().as_bytes())
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path.display())))?;

    Ok(())
}

