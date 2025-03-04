_default:
  just --list -u

alias f := format
alias t := test
alias r := ready
alias l := lint

# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-insta taplo-cli
	cargo binstall --git "https://github.com/astral-sh/uv" uv


# Upgrades the tools needed to develop
upgrade-tools:
	cargo install cargo-binstall --force
	cargo binstall cargo-insta taplo-cli --force
	cargo binstall --git "https://github.com/astral-sh/uv" uv --force

# Generates code generated files for the linter
gen-lint:
  cargo run -p xtask_codegen -- analyser
  cargo run -p xtask_codegen -- configuration
  # cargo codegen-migrate
  # just gen-bindings
  cargo run -p rules_check
  just format

# Creates a new lint rule in the given path, with the given name. Name has to be camel case. Group should be lowercase.
new-lintrule group rulename:
  cargo run -p xtask_codegen -- new-lintrule --category=lint --name={{rulename}} --group={{group}}
  just gen-lint
  # just documentation

# Format Rust files and TOML files
format:
	cargo fmt
	taplo format

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

lint-fix:
  cargo clippy --fix
  cargo run -p rules_check

serve-docs:
    uv sync
    uv run mkdocs serve

# When you finished coding, run this command. Note that you should have already committed your changes.
ready:
  git diff --exit-code --quiet
  cargo run -p xtask_codegen -- configuration
  cargo run -p docs_codegen
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
