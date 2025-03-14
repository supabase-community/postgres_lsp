drop table if exists users;

create table
  users (
    name text,
    id serial primary key,
    email varchar(255)
  );

insert into
  users (name, email)
values
  ('Julian', 'j@gmail.com');