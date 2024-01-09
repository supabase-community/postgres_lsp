use crate::DatabaseConfig;

use super::{Adapter, Config};

pub struct EnvAdapter {}

impl EnvAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Adapter for EnvAdapter {
    fn is_available(&self) -> bool {
        todo!()
    }

    fn load_configs(&self) -> Vec<Config> {
        vec![Config::Database(DatabaseConfig {
            connection_string: Some("env://".to_string()),
        })]
    }
}
