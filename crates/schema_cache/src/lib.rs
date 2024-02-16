//! The schema cache

#![feature(future_join)]

mod schema_cache;
mod schemas;
mod tables;

use sqlx::postgres::PgPool;

pub use schema_cache::SchemaCache;

#[derive(Debug, Clone)]
struct SchemaCacheManager {
    pub cache: SchemaCache,
}

impl SchemaCacheManager {
    pub async fn init(pool: &PgPool) -> Self {
        SchemaCacheManager {
            cache: SchemaCache::load(pool).await,
        }
    }

    pub async fn reload_cache(&mut self, pool: &PgPool) {
        self.cache = SchemaCache::load(pool).await;
    }
}
