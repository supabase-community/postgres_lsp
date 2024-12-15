use serde::Deserialize;
use sqlx::types::JsonValue;
use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

#[derive(Debug, Clone, Default)]
pub struct TypeAttributes {
    attrs: Vec<PostgresTypeAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PostgresTypeAttribute {
    name: String,
    type_id: i64,
}

impl From<Option<JsonValue>> for TypeAttributes {
    fn from(s: Option<JsonValue>) -> Self {
        let values: Vec<PostgresTypeAttribute> =
            serde_json::from_value(s.unwrap_or(JsonValue::Array(vec![]))).unwrap();
        TypeAttributes { attrs: values }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Enums {
    pub values: Vec<String>,
}

impl From<Option<JsonValue>> for Enums {
    fn from(s: Option<JsonValue>) -> Self {
        let values: Vec<String> =
            serde_json::from_value(s.unwrap_or(JsonValue::Array(vec![]))).unwrap();
        Enums { values }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PostgresType {
    pub id: i64,
    pub name: String,
    pub schema: String,
    pub format: String,
    pub enums: Enums,
    pub attributes: TypeAttributes,
    pub comment: Option<String>,
}

impl SchemaCacheItem for PostgresType {
    type Item = PostgresType;

    async fn load(pool: &PgPool) -> Result<Vec<PostgresType>, sqlx::Error> {
        sqlx::query_as!(
            PostgresType,
            r#"select
  t.oid::int8 as "id!",
  t.typname as name,
  n.nspname as "schema!",
  format_type (t.oid, null) as "format!",
  coalesce(t_enums.enums, '[]') as enums,
  coalesce(t_attributes.attributes, '[]') as attributes,
  obj_description (t.oid, 'pg_type') as comment
from
  pg_type t
  left join pg_namespace n on n.oid = t.typnamespace
  left join (
    select
      enumtypid,
      jsonb_agg(enumlabel order by enumsortorder) as enums
    from
      pg_enum
    group by
      enumtypid
  ) as t_enums on t_enums.enumtypid = t.oid
  left join (
    select
      oid,
      jsonb_agg(
        jsonb_build_object('name', a.attname, 'type_id', a.atttypid::int8)
        order by a.attnum asc
      ) as attributes
    from
      pg_class c
      join pg_attribute a on a.attrelid = c.oid
    where
      c.relkind = 'c' and not a.attisdropped
    group by
      c.oid
  ) as t_attributes on t_attributes.oid = t.typrelid
where
  (
    t.typrelid = 0
    or (
      select
        c.relkind = 'c'
      from
        pg_class c
      where
        c.oid = t.typrelid
    )
  )"#
        )
        .fetch_all(pool)
        .await
    }
}
