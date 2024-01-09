mod adapters;
mod database;
mod formatter;

pub use database::DatabaseConfig;
pub use formatter::FormatterConfig;

pub struct UserConfig {
    pub database: DatabaseConfig,
    pub formatter: FormatterConfig,
}

impl UserConfig {
    pub fn load() -> Self {
        adapters::load_user_config()
    }
}
