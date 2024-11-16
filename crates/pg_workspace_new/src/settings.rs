use std::{num::NonZeroU64, path::PathBuf, sync::{RwLock, RwLockWriteGuard}};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use biome_deserialize::{Merge, StringSet};

use pg_configuration::{files::FilesConfiguration, ConfigurationDiagnostic, PartialConfiguration};

use crate::{matcher::Matcher, WorkspaceError};

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct Settings {
    /// The database connection string
    pub db_connection: String,
    /// Filesystem settings for the workspace
    pub files: FilesSettings,
}

pub struct SettingsHandleMut<'a> {
    inner: RwLockWriteGuard<'a, Settings>,
}

impl<'a> SettingsHandleMut<'a> {
    pub(crate) fn new(settings: &'a RwLock<Settings>) -> Self {
        Self {
            inner: settings.write().unwrap(),
        }
    }
}

impl<'a> AsMut<Settings> for SettingsHandleMut<'a> {
    fn as_mut(&mut self) -> &mut Settings {
        &mut self.inner
    }
}


impl Settings {
    /// The [PartialConfiguration] is merged into the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: PartialConfiguration,
        working_directory: Option<PathBuf>,
    ) -> Result<(), WorkspaceError> {
        // Filesystem settings
        if let Some(files) = to_file_settings(
            working_directory.clone(),
            configuration.files.map(FilesConfiguration::from),
        )? {
            self.files = files;
        }

        Ok(())
    }
}

fn to_file_settings(
    working_directory: Option<PathBuf>,
    config: Option<FilesConfiguration>,
) -> Result<Option<FilesSettings>, WorkspaceError> {
    let config = if let Some(config) = config {
        Some(config)
    } else {
        None
    };
    let git_ignore = None;
    Ok(if let Some(config) = config {
        Some(FilesSettings {
            max_size: config.max_size,
            git_ignore,
            ignored_files: to_matcher(working_directory.clone(), Some(&config.ignore))?,
            included_files: to_matcher(working_directory, Some(&config.include))?,
        })
    } else {
        None
    })
}

/// Creates a [Matcher] from a [StringSet]
///
/// ## Errors
///
/// It can raise an error if the patterns aren't valid
pub fn to_matcher(
    working_directory: Option<PathBuf>,
    string_set: Option<&StringSet>,
) -> Result<Matcher, WorkspaceError> {
    let mut matcher = Matcher::empty();
    if let Some(working_directory) = working_directory {
        matcher.set_root(working_directory)
    }
    if let Some(string_set) = string_set {
        for pattern in string_set.iter() {
            matcher.add_pattern(pattern).map_err(|err| {
                ConfigurationDiagnostic::new_invalid_ignore_pattern(
                    pattern.to_string(),
                    err.msg.to_string(),
                )
            })?;
        }
    }
    Ok(matcher)
}


/// Filesystem settings for the entire workspace
#[derive(Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: NonZeroU64,

    /// gitignore file patterns
    pub git_ignore: Option<Gitignore>,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,

    /// List of paths/files to matcher
    pub included_files: Matcher,
}

/// Limit the size of files to 1.0 MiB by default
pub(crate) const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
            git_ignore: None,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        }
    }
}


