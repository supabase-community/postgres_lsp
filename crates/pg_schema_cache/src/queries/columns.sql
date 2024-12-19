select
  tbl.schemaname as schema,
  tbl.tablename as table,
  tbl.quoted_name,
  tbl.is_table,
  json_agg(a) as columns
from
  (
    select
      n.nspname as schemaname,
      c.relname as tablename,
      (
        quote_ident(n.nspname) || '.' || quote_ident(c.relname)
      ) as quoted_name,
      true as is_table
    from
      pg_catalog.pg_class c
      join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where
      c.relkind = 'r'
      and n.nspname != 'pg_toast'
      and n.nspname not like 'pg_temp_%'
      and n.nspname not like 'pg_toast_temp_%'
      and has_schema_privilege(n.oid, 'USAGE') = true
      and has_table_privilege(
        quote_ident(n.nspname) || '.' || quote_ident(c.relname),
        'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
      ) = true
    union
    all
    select
      n.nspname as schemaname,
      c.relname as tablename,
      (
        quote_ident(n.nspname) || '.' || quote_ident(c.relname)
      ) as quoted_name,
      false as is_table
    from
      pg_catalog.pg_class c
      join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where
      c.relkind in ('v', 'm')
      and n.nspname != 'pg_toast'
      and n.nspname not like 'pg_temp_%'
      and n.nspname not like 'pg_toast_temp_%'
      and has_schema_privilege(n.oid, 'USAGE') = true
      and has_table_privilege(
        quote_ident(n.nspname) || '.' || quote_ident(c.relname),
        'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'
      ) = true
  ) as tbl
  left join (
    select
      attrelid,
      attname,
      format_type(atttypid, atttypmod) as data_type,
      attnum,
      attisdropped
    from
      pg_attribute
  ) as a on (
    a.attrelid = tbl.quoted_name :: regclass
    and a.attnum > 0
    and not a.attisdropped
    and has_column_privilege(
      tbl.quoted_name,
      a.attname,
      'SELECT, INSERT, UPDATE, REFERENCES'
    )
  )
group by
  schemaname,
  tablename,
  quoted_name,
  is_table;