use std::future::join;

use sqlx::postgres::PgPool;

use crate::schemas::Schema;
use crate::tables::Table;

#[derive(Debug, Clone, Default)]
pub struct SchemaCache {
    pub schemas: Vec<Schema>,
    pub tables: Vec<Table>,
}

impl SchemaCache {
    pub async fn load(pool: &PgPool) -> SchemaCache {
        let (schemas, tables) = join!(Schema::load(pool), Table::load(pool)).await;

        SchemaCache { schemas, tables }
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
    type Item;

    async fn load(pool: &PgPool) -> Vec<Self::Item>;
}
