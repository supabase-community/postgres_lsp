# Linting Migrations

Postgres Language Tools comes with a `check` command that can be integrated into your development workflow to prevent unexpected downtime caused by database migrations and encourage best practices around Postgres schemas and SQL.

To run it, simply point at your migrations directory.

```sh
pglt check supabase/migrations
```

When you are setting it up in an existing project, you might want to ignore all migrations that are already applied. To do so, add `migrations_dir` and `after` to your `pglt.toml` file


```toml
[migrations]
migrations_dir = "supabase/migrations"
after = 1740868021
```

or pass it directly to the command

```
pglt check supabase/migrations --migrations-dir="supabase/migrations" --after=1740868021
```

This will only check migrations after the specified timestamp.

