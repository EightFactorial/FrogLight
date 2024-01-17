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
  cargo run --profile {{profile}}

# ---- Tool Recipes ----

# Run `just` in `tools/`
tools args="": (fetch-tools)
  @just --justfile tools/.justfile {{args}}

# ---- Test Recipes ----

# Run all tests and doc-tests
all-tests: (tests) (doc-tests) (tools "all-tests")

# Run all tests
tests: (fetch-nextest)
  cargo nextest run --workspace

# Run all doc-tests
doc-tests:
  cargo test --doc --workspace

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

