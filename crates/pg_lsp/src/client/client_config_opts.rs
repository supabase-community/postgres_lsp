use serde::Deserialize;

// TODO: Check that the Opts are correct (existed in server.rs)
#[derive(Deserialize)]
pub struct ClientConfigurationOptions {
    pub db_connection_string: Option<String>
} 