use std::str::FromStr;

use biome_deserialize_macros::{Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// The configuration of the filesystem
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize, Default)]
#[partial(derive(Bpaf, Clone, Eq, PartialEq, Merge))]
#[partial(serde(rename_all = "snake_case", default, deny_unknown_fields))]
pub struct MigrationsConfiguration {
    /// The directory where the migration files are stored
    #[partial(bpaf(hide))]
    pub migration_dir: Option<String>,

    /// The pattern used to store migration files
    #[partial(bpaf(hide))]
    pub migration_pattern: Option<MigrationPattern>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Merge)]
pub enum MigrationPattern {
    /// The migration files are stored in the root of the migration directory
    Root,
    /// The migration files are stored in a subdirectory of the migration directory
    Subdirectory,
}

impl FromStr for MigrationPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "root" => Ok(Self::Root),
            "subdirectory" => Ok(Self::Subdirectory),
            _ => Err(format!("Invalid migration pattern: {}", s)),
        }
    }
}
