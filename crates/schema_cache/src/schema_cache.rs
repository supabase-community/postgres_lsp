use std::future::join;

use sqlx::postgres::PgPool;

use crate::schemas::Schemas;

#[derive(Debug, Clone, Default)]
pub struct SchemaCache {
    pub schemas: Schemas,
}

impl SchemaCache {
    pub async fn load(pool: &PgPool) -> SchemaCache {
        let (schemas) = join!(Schemas::load(pool)).await;

        SchemaCache { schemas }
    }

    /// Applies an AST node to the repository
    ///
    /// For example,  alter table add column will add the column to the table if it does not exist
    /// yet
    pub fn mutate(&mut self) {
        unimplemented!();
    }
}

pub trait SchemaCacheItem {
    async fn load(pool: &PgPool) -> Vec<Self>;
}
