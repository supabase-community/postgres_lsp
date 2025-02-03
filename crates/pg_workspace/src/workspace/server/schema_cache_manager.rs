use std::sync::{RwLock, RwLockReadGuard};

use pg_schema_cache::SchemaCache;
use sqlx::PgPool;

use crate::WorkspaceError;

use super::async_helper::run_async;

#[derive(Debug)]
pub(crate) struct SchemaCacheHandle<'a> {
    inner: RwLockReadGuard<'a, SchemaCache>,
}

impl<'a> SchemaCacheHandle<'a> {
    pub(crate) fn new(cache: &'a RwLock<SchemaCache>) -> Self {
        Self {
            inner: cache.read().unwrap(),
        }
    }

    pub(crate) fn wrap(inner: RwLockReadGuard<'a, SchemaCache>) -> Self {
        Self { inner }
    }
}

impl AsRef<SchemaCache> for SchemaCacheHandle<'_> {
    fn as_ref(&self) -> &SchemaCache {
        &self.inner
    }
}

#[derive(Default)]
pub struct SchemaCacheManager {
    cache: RwLock<SchemaCache>,
}

impl SchemaCacheManager {
    pub fn load(&self, pool: PgPool) -> Result<SchemaCacheHandle, WorkspaceError> {
        let cache_lock = self.cache.read().unwrap();

        if cache_lock.has_already_cached_connection(&pool) {
            Ok(SchemaCacheHandle::wrap(cache_lock))
        } else {
            let maybe_refreshed = run_async(async move { SchemaCache::load(&pool).await })?;
            let refreshed = maybe_refreshed?;

            let mut cache = self.cache.write().unwrap();
            *cache = refreshed;

            Ok(SchemaCacheHandle::new(&self.cache))
        }
    }
}
