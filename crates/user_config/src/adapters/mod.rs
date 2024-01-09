pub mod env;
pub mod supabase;

use env::EnvAdapter;
use supabase::SupabaseAdapter;

use crate::{DatabaseConfig, FormatterConfig, UserConfig};

// Enum to represent different configuration types
pub enum Config {
    Database(DatabaseConfig),
    Formatter(FormatterConfig),
}

pub trait Adapter {
    /// Check if the adapter is available
    fn is_available(&self) -> bool;

    /// Load all configs available with this adapter
    fn load_configs(&self) -> Vec<Config>;
}

fn load_config_from_adapters(adapters: &[Box<dyn Adapter>]) -> UserConfig {
    let mut database_config: Option<DatabaseConfig> = None;
    let mut formatter_config: Option<FormatterConfig> = None;

    for adapter in adapters {
        if adapter.is_available() {
            for config in adapter.load_configs() {
                match config {
                    Config::Database(db_config) if database_config.is_none() => {
                        database_config = Some(db_config);
                    }
                    Config::Formatter(fmt_config) if formatter_config.is_none() => {
                        formatter_config = Some(fmt_config);
                    }
                    _ => {}
                }
            }
        }
    }

    UserConfig {
        database: database_config.unwrap(),
        formatter: formatter_config.unwrap(),
    }
}

pub fn load_user_config() -> UserConfig {
    let adapters: Vec<Box<dyn Adapter>> = vec![
        Box::new(SupabaseAdapter::new()),
        Box::new(EnvAdapter::new()),
    ];

    load_config_from_adapters(&adapters)
}
