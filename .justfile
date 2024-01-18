#!/usr/bin/env just --justfile

# Alias for `run`
default: (run-profile)

# ---- Build Recipes ----

# Compile development build
alias build := build-profile
# Compile development build
alias build-dev := build-profile
# Compile release build
build-release: (build-profile "release")

# Compile build with specified profile
[private]
build-profile profile="dev":
  cargo build --profile {{profile}}

# ---- Run Recipes ----

# Run development build
alias run := run-profile
# Run development build
alias run-dev := run-profile
# Run release build
run-release: (run-profile "release")

# Run build with specified profile
[private]
run-profile profile="dev":
  cargo run --profile {{profile}} --features logging

# ---- Test Recipes ----


# Run all tests and all tool tests
all-tests: (test) (tools "all-tests")

# Run all tests and doc-tests
test: (deny) (fmt) (nextest) (doc-test) 

# Run cargo deny
deny:
  cargo deny check

# Run cargo fmt
fmt:
  cargo fmt --all

# Run clippy
clippy:
  cargo clippy --workspace

# Run all tests
nextest: (fetch-nextest)
  cargo nextest run --workspace

# Run all doc-tests
doc-test:
  cargo test --doc --workspace

# ---- Tool Recipes ----

# Run `just` in `tools/`
tools args="": (fetch-tools)
  @just --justfile tools/.justfile {{args}}

# ---- Fetch Recipes ----

# Fetch `froglight-tools` submodule if not present
[private]
fetch-tools:
  @if [ ! -f tools/.justfile ]; then git submodule update; fi

# Fetch `nextest` if not present
[private]
fetch-nextest:
  @-cargo nextest --version > /dev/null 2>&1
  @if [ $? -ne 0 ]; then cargo install nextest; fi

