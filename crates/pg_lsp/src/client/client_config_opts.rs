use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClientConfigurationOptions {
    #[serde(rename(deserialize = "databaseUrl"))]
    pub(crate) db_connection_string: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::client::client_config_opts::ClientConfigurationOptions;

    #[test]
    fn test_json_parsing() {
        let config = json!({
            "databaseUrl": "cool-shit"
        });

        let parsed: ClientConfigurationOptions = serde_json::from_value(config).unwrap();

        assert_eq!(parsed.db_connection_string, Some("cool-shit".into()));
    }
}
