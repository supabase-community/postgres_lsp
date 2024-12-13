use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

#[derive(Debug, Clone, Default)]
pub struct Schema {
    id: i64,
    name: String,
    owner: String,
}

impl SchemaCacheItem for Schema {
    type Item = Schema;

    async fn load(pool: &PgPool) -> Result<Vec<Schema>, sqlx::Error> {
        sqlx::query_as!(
            Schema,
            r#"select
  n.oid::int8 as "id!",
  n.nspname as name,
  u.rolname as "owner!"
from
  pg_namespace n,
  pg_roles u
where
  n.nspowner = u.oid
  and (
    pg_has_role(n.nspowner, 'USAGE')
    or has_schema_privilege(n.oid, 'CREATE, USAGE')
  )
  and not pg_catalog.starts_with(n.nspname, 'pg_temp_')
  and not pg_catalog.starts_with(n.nspname, 'pg_toast_temp_')"#
        )
        .fetch_all(pool)
        .await
    }
}
