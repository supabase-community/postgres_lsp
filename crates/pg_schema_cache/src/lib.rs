//! The schema cache

#![allow(dead_code)]
#![feature(future_join)]

mod functions;
mod versions;
mod schema_cache;
mod schemas;
mod tables;
mod types;

use sqlx::postgres::PgPool;

pub use functions::{Behavior, Function, FunctionArg, FunctionArgs};
pub use schema_cache::SchemaCache;
pub use tables::{ReplicaIdentity, Table};

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
