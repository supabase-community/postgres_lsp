use crate::DatabaseConfig;

use super::{Adapter, Config};

pub struct SupabaseAdapter {}

impl SupabaseAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Adapter for SupabaseAdapter {
    fn is_available(&self) -> bool {
        todo!()
    }

    fn load_configs(&self) -> Vec<Config> {
        vec![Config::Database(DatabaseConfig {
            connection_string: Some("supabase://".to_string()),
        })]
    }
}
