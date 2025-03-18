select
  version(),
  current_setting('server_version_num') :: int8 AS version_num,
  (
    select
      count(*) :: int8 AS active_connections
    FROM
      pg_stat_activity
  ) AS active_connections,
  current_setting('max_connections') :: int8 AS max_connections;