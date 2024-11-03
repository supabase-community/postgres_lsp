use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct Options {
    pub db_connection_string: Option<String>,
}
