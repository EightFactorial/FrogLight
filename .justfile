#!/usr/bin/env just --justfile

# Alias for `run`
default: (run-profile "dev")

# ---- Build Recipes ----

# Compile development build
alias build := build-profile
# Compile development build
alias build-dev := build-profile
# Compile release build
build-release: (build-profile "release")

# Compile build with specified profile
[private]
build-profile profile="dev" args="":
  cargo build --profile {{profile}} --features mimalloc {{args}}

# Clean build artifacts
clean: (fetch-tools) (tools "clean")
  cargo clean

# ---- Run Recipes ----

# Run development build
alias run := run-profile
# Run development build
alias run-dev := run-profile
# Run release build
run-release: (run-profile "release")

# Run build with specified profile
[private]
run-profile profile="dev" args="":
  cargo run --profile {{profile}} --features mimalloc {{args}}

# ---- Test Recipes ----

# Run all tests and all tool tests
all-tests: (update) (deny) (fmt) (test) (graph) (tools "all-tests")

# Run all tests and doc-tests
test: (nextest) (doc-test) 

# Run all tests
nextest: (fetch-nextest)
  cargo nextest run --workspace

# Get number of threads
threads := `nproc --all`

# Run all doc-tests
# Uses at most 4 threads
doc-test: 
  cargo test --doc --workspace -- --test-threads=$(( {{threads}} > 4 ? 4 : {{threads}} ))

# ---- Tool Recipes ----

# Run cargo deny
deny:
  cargo deny check

# Run cargo update
update:
  cargo update

# Run clippy
clippy:
  cargo clippy --workspace

# Run cargo fmt
fmt:
  cargo fmt --all

# Run `just` in `tools/`
tools arg0="" arg1="" arg2="" arg3="" arg4="": (fetch-tools)
  @just --justfile tools/.justfile {{arg0}} {{arg1}} {{arg2}} {{arg3}} {{arg4}}

# Generate froglight-app graphs
graph:
  RUST_LOG=info cargo run --package=froglight-app --example=system-graph --features default

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

