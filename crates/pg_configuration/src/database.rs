use biome_deserialize_macros::{Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// The configuration of the database connection.
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Eq, PartialEq, Merge))]
#[partial(serde(rename_all = "snake_case", default, deny_unknown_fields))]
pub struct DatabaseConfiguration {
    /// The host of the database.
    #[partial(bpaf(long("host")))]
    pub host: String,

    /// The port of the database.
    #[partial(bpaf(long("port")))]
    pub port: u16,

    /// The username to connect to the database.
    #[partial(bpaf(long("username")))]
    pub username: String,

    /// The password to connect to the database.
    #[partial(bpaf(long("password")))]
    pub password: String,

    /// The name of the database.
    #[partial(bpaf(long("database")))]
    pub database: String,
}

impl Default for DatabaseConfiguration {
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

impl DatabaseConfiguration {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
