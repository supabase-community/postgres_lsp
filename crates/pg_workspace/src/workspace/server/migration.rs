use std::path::Path;

#[derive(Debug)]
pub(crate) struct Migration {
    pub(crate) timestamp: u64,
    pub(crate) name: String,
}

/// Get the migration associated with a path, if it is a migration file
pub(crate) fn get_migration(path: &Path, migrations_dir: &Path) -> Option<Migration> {
    // Check if path is a child of the migration directory
    let is_child = path
        .canonicalize()
        .ok()
        .and_then(|canonical_child| {
            migrations_dir
                .canonicalize()
                .ok()
                .map(|canonical_dir| canonical_child.starts_with(&canonical_dir))
        })
        .unwrap_or(false);

    if !is_child {
        return None;
    }

    // we are trying to match patterns used by popular migration tools

    // in the "root" pattern, all files are directly within the migrations directory
    // and their names follow <timestamp>_<name>.sql.
    // this is used by supabase
    let root_migration = path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .and_then(|file_name| {
            let mut parts = file_name.splitn(2, '_');
            let timestamp = parts.next()?.parse().ok()?;
            let name = parts.next()?.to_string();
            Some(Migration { timestamp, name })
        });

    if root_migration.is_some() {
        return root_migration;
    }

    // in the "subdirectory" pattern, each migration is in a subdirectory named <timestamp>_<name>
    // this is used by prisma and drizzle
    path.parent()
        .and_then(|parent| parent.file_name())
        .and_then(|os_str| os_str.to_str())
        .and_then(|dir_name| {
            let mut parts = dir_name.splitn(2, '_');
            let timestamp = parts.next()?.parse().ok()?;
            let name = parts.next()?.to_string();
            Some(Migration { timestamp, name })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        TempDir::new().expect("Failed to create temp dir")
    }

    #[test]
    fn test_get_migration_root_pattern() {
        let temp_dir = setup();
        let migrations_dir = temp_dir.path().to_path_buf();
        let path = migrations_dir.join("1234567890_create_users.sql");
        fs::write(&path, "").unwrap();

        let migration = get_migration(&path, &migrations_dir);

        assert!(migration.is_some());
        let migration = migration.unwrap();
        assert_eq!(migration.timestamp, 1234567890);
        assert_eq!(migration.name, "create_users.sql");
    }

    #[test]
    fn test_get_migration_subdirectory_pattern() {
        let temp_dir = setup();
        let migrations_dir = temp_dir.path().to_path_buf();
        let subdir = migrations_dir.join("1234567890_create_users");
        fs::create_dir(&subdir).unwrap();
        let path = subdir.join("up.sql");
        fs::write(&path, "").unwrap();

        let migration = get_migration(&path, &migrations_dir);

        assert!(migration.is_some());
        let migration = migration.unwrap();
        assert_eq!(migration.timestamp, 1234567890);
        assert_eq!(migration.name, "create_users");
    }

    #[test]
    fn test_get_migration_not_timestamp_in_filename() {
        let migrations_dir = PathBuf::from("/tmp/migrations");
        let path = migrations_dir.join("not_a_migration.sql");

        let migration = get_migration(&path, &migrations_dir);

        assert!(migration.is_none());
    }

    #[test]
    fn test_get_migration_outside_migrations_dir() {
        let migrations_dir = PathBuf::from("/tmp/migrations");
        let path = PathBuf::from("/tmp/other/1234567890_create_users.sql");

        let migration = get_migration(&path, &migrations_dir);

        assert!(migration.is_none());
    }
}
