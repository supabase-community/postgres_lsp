create table
  unknown_users (id serial primary key, address text, email text);

drop table unknown_users;

select
  *
from
  unknown_users;