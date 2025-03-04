# Linting Migrations

Postgres Language Tools comes with a `check` command that can be integrated into your development workflow to catch problematic schema changes and encourage best practices.

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

Alternatively, pass them directly.

```
pglt check supabase/migrations --migrations-dir="supabase/migrations" --after=1740868021
```

This will only check migrations after the specified timestamp.

For pre-commit hooks and when working locally, use `--staged` to only lint files that have been staged. In CI environments, you most likely want to use `--changed` to only lint files that have been changed compared to your `defaultBranch` configuration. If `defaultBranch` is not set in your `pglt.toml`, use `--since=REF` to specify the base branch to compare against.

