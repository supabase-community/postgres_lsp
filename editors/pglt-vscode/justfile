# Run Docker Containers first
init:
  psql "postgresql://postgres:example@127.0.0.1:3333/postgres" -f ./seed.sql

connect:
  psql "postgresql://postgres:example@127.0.0.1:3333/postgres"