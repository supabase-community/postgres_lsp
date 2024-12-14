select
  c.oid :: int8 as "id!",
  nc.nspname as schema,
  c.relname as name,
  c.relrowsecurity as rls_enabled,
  c.relforcerowsecurity as rls_forced,
  case
    when c.relreplident = 'd' then 'default'
    when c.relreplident = 'i' then 'index'
    when c.relreplident = 'f' then 'full'
    else 'nothing'
  end as "replica_identity!",
  pg_total_relation_size(format('%i.%i', nc.nspname, c.relname)) :: int8 as "bytes!",
  pg_size_pretty(
    pg_total_relation_size(format('%i.%i', nc.nspname, c.relname))
  ) as "size!",
  pg_stat_get_live_tuples(c.oid) as "live_rows_estimate!",
  pg_stat_get_dead_tuples(c.oid) as "dead_rows_estimate!",
  obj_description(c.oid) as comment
from
  pg_namespace nc
  join pg_class c on nc.oid = c.relnamespace
where
  c.relkind in ('r', 'p')
  and not pg_is_other_temp_schema(nc.oid)
  and (
    pg_has_role(c.relowner, 'usage')
    or has_table_privilege(
      c.oid,
      'select, insert, update, delete, truncate, references, trigger'
    )
    or has_any_column_privilege(c.oid, 'select, insert, update, references')
  )
group by
  c.oid,
  c.relname,
  c.relrowsecurity,
  c.relforcerowsecurity,
  c.relreplident,
  nc.nspname;