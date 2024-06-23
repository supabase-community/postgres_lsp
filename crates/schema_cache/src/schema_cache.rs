use std::future::join;

use sqlx::postgres::PgPool;

use crate::functions::Function;
use crate::schemas::Schema;
use crate::tables::Table;

#[derive(Debug, Clone, Default)]
pub struct SchemaCache {
    pub schemas: Vec<Schema>,
    pub tables: Vec<Table>,
    pub functions: Vec<Function>,
}

impl SchemaCache {
    pub fn new() -> SchemaCache {
        SchemaCache::default()
    }

    pub async fn load(pool: &PgPool) -> SchemaCache {
        let (schemas, tables, functions) =
            join!(Schema::load(pool), Table::load(pool), Function::load(pool)).await;

        SchemaCache {
            schemas,
            tables,
            functions,
        }
    }

    /// Applies an AST node to the repository
    ///
    /// For example,  alter table add column will add the column to the table if it does not exist
    /// yet
    pub fn mutate(&mut self) {
        unimplemented!();
    }

    pub fn find_table(&self, name: &str, schema: Option<&str>) -> Option<&Table> {
        self.tables
            .iter()
            .find(|t| t.name == name && t.schema == schema.unwrap_or("public"))
    }
}

pub trait SchemaCacheItem {
    type Item;

    async fn load(pool: &PgPool) -> Vec<Self::Item>;
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::SchemaCache;

    #[test]
    fn test_schema_cache() {
        let conn_string = std::env::var("DB_CONNECTION_STRING").unwrap();

        let pool = async_std::task::block_on(PgPool::connect(conn_string.as_str())).unwrap();

        async_std::task::block_on(SchemaCache::load(&pool));

        assert!(true);
    }
}
