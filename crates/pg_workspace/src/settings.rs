use biome_deserialize::StringSet;
use pg_diagnostics::Category;
use std::{
    borrow::Cow,
    num::NonZeroU64,
    path::{Path, PathBuf},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use pg_configuration::{
    database::PartialDatabaseConfiguration,
    diagnostics::InvalidIgnorePattern,
    files::{FilesConfiguration, MigrationPattern},
    migrations::{MigrationsConfiguration, PartialMigrationsConfiguration},
    ConfigurationDiagnostic, LinterConfiguration, PartialConfiguration,
};
use pg_fs::FileSystem;

use crate::{matcher::Matcher, DynRef, WorkspaceError};

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct Settings {
    /// Filesystem settings for the workspace
    pub files: FilesSettings,

    /// Database settings for the workspace
    pub db: DatabaseSettings,

    /// Linter settings applied to all files in the workspace
    pub linter: LinterSettings,

    /// Migrations settings
    pub migrations: Option<Migrations>,
}

#[derive(Debug)]
pub struct SettingsHandleMut<'a> {
    inner: RwLockWriteGuard<'a, Settings>,
}

/// Handle object holding a temporary lock on the settings
#[derive(Debug)]
pub struct SettingsHandle<'a> {
    inner: RwLockReadGuard<'a, Settings>,
}

impl<'a> SettingsHandle<'a> {
    pub(crate) fn new(settings: &'a RwLock<Settings>) -> Self {
        Self {
            inner: settings.read().unwrap(),
        }
    }
}

impl AsRef<Settings> for SettingsHandle<'_> {
    fn as_ref(&self) -> &Settings {
        &self.inner
    }
}

impl<'a> SettingsHandleMut<'a> {
    pub(crate) fn new(settings: &'a RwLock<Settings>) -> Self {
        Self {
            inner: settings.write().unwrap(),
        }
    }
}

impl AsMut<Settings> for SettingsHandleMut<'_> {
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
        vcs_path: Option<PathBuf>,
        gitignore_matches: &[String],
    ) -> Result<(), WorkspaceError> {
        // Filesystem settings
        if let Some(files) = to_file_settings(
            working_directory.clone(),
            configuration.files.map(FilesConfiguration::from),
            vcs_path,
            gitignore_matches,
        )? {
            self.files = files;
        }

        // db settings
        if let Some(db) = configuration.db {
            self.db = db.into()
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter =
                to_linter_settings(working_directory.clone(), LinterConfiguration::from(linter))?;
        }

        // TODO migrations

        Ok(())
    }

    /// Retrieves the settings of the linter
    pub fn linter(&self) -> &LinterSettings {
        &self.linter
    }

    /// Returns linter rules.
    pub fn as_linter_rules(&self) -> Option<Cow<pg_configuration::analyser::linter::Rules>> {
        self.linter.rules.as_ref().map(Cow::Borrowed)
    }

    /// It retrieves the severity based on the `code` of the rule and the current configuration.
    ///
    /// The code of the has the following pattern: `{group}/{rule_name}`.
    ///
    /// It returns [None] if the `code` doesn't match any rule.
    pub fn get_severity_from_rule_code(&self, code: &Category) -> Option<pg_diagnostics::Severity> {
        let rules = self.linter.rules.as_ref();
        if let Some(rules) = rules {
            rules.get_severity_from_code(code)
        } else {
            None
        }
    }
}

fn to_linter_settings(
    working_directory: Option<PathBuf>,
    conf: LinterConfiguration,
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled,
        rules: Some(conf.rules),
        ignored_files: to_matcher(working_directory.clone(), Some(&conf.ignore))?,
        included_files: to_matcher(working_directory.clone(), Some(&conf.include))?,
    })
}

fn to_file_settings(
    working_directory: Option<PathBuf>,
    config: Option<FilesConfiguration>,
    vcs_config_path: Option<PathBuf>,
    gitignore_matches: &[String],
) -> Result<Option<FilesSettings>, WorkspaceError> {
    let config = if let Some(config) = config {
        Some(config)
    } else if vcs_config_path.is_some() {
        Some(FilesConfiguration::default())
    } else {
        None
    };
    let git_ignore = if let Some(vcs_config_path) = vcs_config_path {
        Some(to_git_ignore(vcs_config_path, gitignore_matches)?)
    } else {
        None
    };
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

fn to_git_ignore(path: PathBuf, matches: &[String]) -> Result<Gitignore, WorkspaceError> {
    let mut gitignore_builder = GitignoreBuilder::new(path.clone());

    for the_match in matches {
        gitignore_builder
            .add_line(Some(path.clone()), the_match)
            .map_err(|err| {
                ConfigurationDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
                    message: err.to_string(),
                    file_path: path.to_str().map(|s| s.to_string()),
                })
            })?;
    }
    let gitignore = gitignore_builder.build().map_err(|err| {
        ConfigurationDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
            message: err.to_string(),
            file_path: path.to_str().map(|s| s.to_string()),
        })
    })?;
    Ok(gitignore)
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

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of rules
    pub rules: Option<pg_configuration::analyser::linter::Rules>,

    /// List of ignored paths/files to match
    pub ignored_files: Matcher,

    /// List of included paths/files to match
    pub included_files: Matcher,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(pg_configuration::analyser::linter::Rules::default()),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        }
    }
}

/// Database settings for the entire workspace
#[derive(Debug)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "postgres".to_string(),
        }
    }
}

impl DatabaseSettings {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl From<PartialDatabaseConfiguration> for DatabaseSettings {
    fn from(value: PartialDatabaseConfiguration) -> Self {
        let d = DatabaseSettings::default();
        Self {
            host: value.host.unwrap_or(d.host),
            port: value.port.unwrap_or(d.port),
            username: value.username.unwrap_or(d.username),
            password: value.password.unwrap_or(d.password),
            database: value.database.unwrap_or(d.database),
        }
    }
}

/// Filesystem settings for the entire workspace
#[derive(Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: NonZeroU64,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,

    /// List of paths/files to matcher
    pub included_files: Matcher,

    /// gitignore file patterns
    pub git_ignore: Option<Gitignore>,
}

/// Migration settings
#[derive(Debug)]
pub(crate) struct Migrations {
    path: PathBuf,
    pattern: MigrationPattern,
}

pub(crate) struct Migration {
    timestamp: u64,
    name: String,
}

impl Migrations {
    fn get_migration(&self, path: &Path) -> Option<Migration> {
        // check if path is a child of the migration directory
        match path.canonicalize() {
            Ok(canonical_child) => match self.path.canonicalize() {
                Ok(canonical_dir) => canonical_child.starts_with(&canonical_dir),
                Err(_) => return None,
            },
            Err(_) => return None,
        };

        match self.pattern {
            // supabase style migrations/0001_create_table.sql
            MigrationPattern::Root => {
                let file_name = path.file_name()?.to_str()?;
                let timestamp = file_name.split('_').next()?;
                let name = file_name
                    .split('_')
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join("_");
                let timestamp = timestamp.parse().ok()?;
                Some(Migration { timestamp, name })
            }
            // drizzle / prisma style migrations/0001_create_table/migration.sql
            MigrationPattern::Subdirectory => {
                let relative_path = path.strip_prefix(&self.path).ok()?;
                let components: Vec<_> = relative_path.components().collect();
                if components.len() != 2 {
                    return None;
                }
                let dir_name = components[0].as_os_str().to_str()?;
                let parts: Vec<&str> = dir_name.splitn(2, '_').collect();
                if parts.len() != 2 {
                    return None;
                }
                let timestamp = parts[0].parse().ok()?;
                let name = parts[1].to_string();
                Some(Migration { timestamp, name })
            }
        }
    }
}

impl From<MigrationsConfiguration> for Migrations {
    fn from(value: MigrationsConfiguration) -> Self {
        Self {
            path: value.migration_dir,
            pattern: value.migration_pattern,
        }
    }
}

/// Limit the size of files to 1.0 MiB by default
pub(crate) const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
            git_ignore: None,
        }
    }
}

pub trait PartialConfigurationExt {
    fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError>;
}

impl PartialConfigurationExt for PartialConfiguration {
    /// This function checks if the VCS integration is enabled, and if so, it will attempts to resolve the
    /// VCS root directory and the `.gitignore` file.
    ///
    /// ## Returns
    ///
    /// A tuple with VCS root folder and the contents of the `.gitignore` file
    fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError> {
        let Some(vcs) = &self.vcs else {
            return Ok((None, vec![]));
        };
        if vcs.is_enabled() {
            let vcs_base_path = match (vcs_base_path, &vcs.root) {
                (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
                (None, Some(root)) => PathBuf::from(root),
                (Some(vcs_base_path), None) => PathBuf::from(vcs_base_path),
                (None, None) => return Err(WorkspaceError::vcs_disabled()),
            };
            if let Some(client_kind) = &vcs.client_kind {
                if !vcs.ignore_file_disabled() {
                    let result = file_system
                        .auto_search(&vcs_base_path, &[client_kind.ignore_file()], false)
                        .map_err(WorkspaceError::from)?;

                    if let Some(result) = result {
                        return Ok((
                            result.file_path.parent().map(PathBuf::from),
                            result
                                .content
                                .lines()
                                .map(String::from)
                                .collect::<Vec<String>>(),
                        ));
                    }
                }
            }
        }
        Ok((None, vec![]))
    }
}
