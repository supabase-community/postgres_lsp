with
  available_tables as (
    select
      c.relname as table_name,
      c.oid as table_oid,
      c.relkind as class_kind,
      n.nspname as schema_name
    from
      pg_catalog.pg_class c
      left join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where
      -- r: normal tables
      -- v: views
      -- m: materialized views
      -- f: foreign tables
      -- p: partitioned tables
      c.relkind in ('r', 'v', 'm', 'f', 'p')
  )
select
  atts.attname as name,
  ts.table_name,
  ts.table_oid,
  ts.class_kind,
  ts.schema_name,
  atts.attnum,
  atts.atttypid as type_id,
  not atts.attnotnull as is_nullable,
  nullif(
    information_schema._pg_char_max_length (atts.atttypid, atts.atttypmod),
    -1
  ) as varchar_length
from
  pg_catalog.pg_attribute atts
  left join available_tables ts on atts.attrelid = ts.table_oid
where
  -- system columns, such as `cmax` or `tableoid`, have negative `attnum`s
  atts.attnum >= 0;