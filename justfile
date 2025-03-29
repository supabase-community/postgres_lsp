_default:
  just --list -u

alias f := format
alias r := ready
alias l := lint
alias t := test

# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-insta taplo-cli wasm-pack wasm-tools
	cargo binstall --git "https://github.com/astral-sh/uv" uv
	bun install

# Upgrades the tools needed to develop
upgrade-tools:
	cargo install cargo-binstall --force
	cargo binstall cargo-insta taplo-cli --force
	cargo binstall --git "https://github.com/astral-sh/uv" uv --force
	bun install

# Generates code generated files for the linter
gen-lint:
  cargo run -p xtask_codegen -- analyser
  cargo run -p xtask_codegen -- configuration
  cargo run -p xtask_codegen -- bindings
  cargo run -p rules_check
  cargo run -p docs_codegen
  just format

# Creates a new lint rule in the given path, with the given name. Name has to be camel case. Group should be lowercase.
new-lintrule group rulename:
  cargo run -p xtask_codegen -- new-lintrule --category=lint --name={{rulename}} --group={{group}}
  just gen-lint

# Format Rust, JS and TOML files
format:
	cargo fmt
	taplo format
	bun biome format --write

[unix]
_touch file:
  touch {{file}}

[windows]
_touch file:
  (gci {{file}}).LastWriteTime = Get-Date

# Run tests of all crates
test:
	cargo test run --no-fail-fast

# Run tests for the crate passed as argument e.g. just test-create pg_cli
test-crate name:
	cargo test run -p {{name}} --no-fail-fast

# Run doc tests
test-doc:
	cargo test --doc

# Alias for `cargo clippy`, it runs clippy on the whole codebase
lint:
  cargo clippy
  cargo run -p rules_check
  bun biome lint

lint-fix:
  cargo clippy --fix
  cargo run -p rules_check
  bun biome lint --write

serve-docs:
    uv sync
    uv run mkdocs serve

# When you finished coding, run this command. Note that you should have already committed your changes.
ready:
  git diff --exit-code --quiet
  cargo run -p xtask_codegen -- configuration
  cargo run -p docs_codegen
  cargo run -p xtask_codegen -- bindings
  just lint-fix
  just format
  git diff --exit-code --quiet

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}

# Prints the treesitter tree of the given SQL file
tree-print file:
	cargo run --bin tree_print -- -f {{file}}

clear-branches:
    git branch --merged | egrep -v "(^\\*|main)" | xargs git branch -d

reset-git:
    git checkout main
    git pull
    just clear-branches

merge-main:
    git fetch origin main:main
    git merge main


# Make sure to set your PGT_LOG_PATH in your shell profile.
show-logs:
    tail -f $(ls $PGT_LOG_PATH/server.log.* | sort -t- -k2,2 -k3,3 -k4,4 | tail -n 1)
