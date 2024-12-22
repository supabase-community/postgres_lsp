_default:
  just --list -u

alias f := format
alias t := test
# alias r := ready
# alias l := lint
# alias qt := test-quick

# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-insta taplo-cli

# Upgrades the tools needed to develop
upgrade-tools:
	cargo install cargo-binstall --force
	cargo binstall cargo-insta taplo-cli --force

# Generate all files across crates and tools. You rarely want to use it locally.
gen-all:
  cargo run -p xtask_codegen -- all
  # cargo codegen-configuration
#   cargo codegen-migrate
#   just gen-bindings
#   just format

# Generates TypeScript types and JSON schema of the configuration
# gen-bindings:
#   cargo codegen-schema
#   cargo codegen-bindings

# Generates code generated files for the linter
gen-lint:
  cargo run -p xtask_codegen -- analyser
  cargo run -p xtask_codegen -- configuration
  # cargo codegen-migrate
  # just gen-bindings
  # cargo run -p rules_check
  just format

# Generates the linter documentation and Rust documentation
# documentation:
#   RUSTDOCFLAGS='-D warnings' cargo documentation

# Creates a new lint rule in the given path, with the given name. Name has to be camel case. Group should be lowercase.
new-lintrule group rulename:
  cargo run -p xtask_codegen -- new-lintrule --category=lint --name={{rulename}} --group={{group}}
  just gen-lint
  # just documentation

# Creates a new lint rule in the given path, with the given name. Name has to be camel case.
# new-assistrule rulename:
#   cargo run -p xtask_codegen -- new-lintrule --kind=js --category=assist --name={{rulename}}
#   just gen-lint
#   just documentation

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

# Tests a lint rule. The name of the rule needs to be camel case
# test-lintrule name:
#   just _touch crates/biome_js_analyze/tests/spec_tests.rs
#   just _touch crates/biome_json_analyze/tests/spec_tests.rs
#   just _touch crates/biome_css_analyze/tests/spec_tests.rs
#   just _touch crates/biome_graphql_analyze/tests/spec_tests.rs
#   cargo test -p biome_js_analyze -- {{snakecase(name)}} --show-output
#   cargo test -p biome_json_analyze -- {{snakecase(name)}} --show-output
#   cargo test -p biome_css_analyze -- {{snakecase(name)}} --show-output
#   cargo test -p biome_graphql_analyze -- {{snakecase(name)}} --show-output

# Tests a lint rule. The name of the rule needs to be camel case
# test-transformation name:
#   just _touch crates/biome_js_transform/tests/spec_tests.rs
#   cargo test -p biome_js_transform -- {{snakecase(name)}} --show-output

# Run the quick_test for the given package.
# test-quick package:
#   cargo test -p {{package}} --test quick_test -- quick_test --nocapture --ignored


# Alias for `cargo clippy`, it runs clippy on the whole codebase
lint:
  cargo clippy

lint-fix:
  cargo clippy --fix

# When you finished coding, run this command to run the same commands in the CI.
# ready:
#   git diff --exit-code --quiet
#   just gen-all
#   just documentation
#   #just format # format is already run in `just gen-all`
#   just lint
#   just test
#   just test-doc
#   git diff --exit-code --quiet

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}

# Creates a new changeset for the final changelog
# new-changeset:
#     knope document-change

# Dry-run of the release
# dry-run-release *args='':
#     knope release --dry-run {{args}}

clear-branches:
    git branch --merged | egrep -v "(^\\*|main)" | xargs git branch -d

reset-git:
    git checkout main && git pull && pnpm run clear-branches
