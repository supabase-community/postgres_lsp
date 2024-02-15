use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

#[derive(Debug, Clone, PartialEq)]
pub enum ReplicaIdentity {
    Default,
    Index,
    Full,
    Nothing,
}

impl Default for ReplicaIdentity {
    fn default() -> Self {
        ReplicaIdentity::Default
    }
}

impl From<String> for ReplicaIdentity {
    fn from(s: String) -> Self {
        match s.as_str() {
            "DEFAULT" => ReplicaIdentity::Default,
            "INDEX" => ReplicaIdentity::Index,
            "FULL" => ReplicaIdentity::Full,
            "NOTHING" => ReplicaIdentity::Nothing,
            _ => panic!("Invalid replica identity"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    id: i64,
    schema: String,
    name: String,
    rls_enabled: bool,
    rls_forced: bool,
    replica_identity: ReplicaIdentity,
    bytes: i64,
    size: String,
    live_rows_estimate: i64,
    dead_rows_estimate: i64,
    comment: Option<String>,
}

impl SchemaCacheItem for Table {
    type Item = Table;

    async fn load(pool: &PgPool) -> Vec<Table> {
        sqlx::query_as!(
            Table,
            r#"SELECT
  c.oid :: int8 AS "id!",
  nc.nspname AS schema,
  c.relname AS name,
  c.relrowsecurity AS rls_enabled,
  c.relforcerowsecurity AS rls_forced,
  CASE
    WHEN c.relreplident = 'd' THEN 'DEFAULT'
    WHEN c.relreplident = 'i' THEN 'INDEX'
    WHEN c.relreplident = 'f' THEN 'FULL'
    ELSE 'NOTHING'
  END AS "replica_identity!",
  pg_total_relation_size(format('%I.%I', nc.nspname, c.relname)) :: int8 AS "bytes!",
  pg_size_pretty(
    pg_total_relation_size(format('%I.%I', nc.nspname, c.relname))
  ) AS "size!",
  pg_stat_get_live_tuples(c.oid) AS "live_rows_estimate!",
  pg_stat_get_dead_tuples(c.oid) AS "dead_rows_estimate!",
  obj_description(c.oid) AS comment
FROM
  pg_namespace nc
  JOIN pg_class c ON nc.oid = c.relnamespace
WHERE
  c.relkind IN ('r', 'p')
  AND NOT pg_is_other_temp_schema(nc.oid)
  AND (
    pg_has_role(c.relowner, 'USAGE')
    OR has_table_privilege(
      c.oid,
      'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
    )
    OR has_any_column_privilege(c.oid, 'SELECT, INSERT, UPDATE, REFERENCES')
  )
group by
  c.oid,
  c.relname,
  c.relrowsecurity,
  c.relforcerowsecurity,
  c.relreplident,
  nc.nspname"#
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }
}
