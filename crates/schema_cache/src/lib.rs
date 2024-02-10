//! The schema cache

#![feature(future_join)]

mod schema_cache;
mod schemas;
mod tables;

use schema_cache::SchemaCache;
use sqlx::postgres::PgPool;

#[derive(Debug, Clone)]
struct SchemaCacheManager {
    pool: PgPool,
    pub cache: SchemaCache,
}

impl SchemaCacheManager {
    pub async fn init(pool: PgPool) -> Self {
        SchemaCacheManager {
            cache: SchemaCache::load(&pool).await,
            pool,
        }
    }

    pub async fn reload_cache(&mut self) {
        self.cache = SchemaCache::load(&self.pool).await;
    }
}
