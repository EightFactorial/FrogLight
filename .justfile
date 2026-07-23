ALL_FEATURES := "all-features"
DEF_FEATURES := "features=bevy,network,std"
NO_FEATURES := "features=libm,once_cell --no-default-features"

EXCLUDE := "exclude=froglight-codegen --exclude=froglight-api"
EXCLUDE_CODEGEN := "exclude=froglight-codegen"

# Generate the changelog
changelog path="CHANGELOG.md":
    git-cliff --output {{ path }}

# Run the clippy linter
clippy:
    cargo clippy --workspace --{{ ALL_FEATURES }} --{{ EXCLUDE_CODEGEN }} -- -D warnings
    cargo clippy --workspace --{{ DEF_FEATURES }} --{{ EXCLUDE_CODEGEN }} -- -D warnings
    cargo clippy --workspace --{{ NO_FEATURES }}  --{{ EXCLUDE_CODEGEN }} -- -D warnings

# Clean up all build artifacts
clean:
    cargo clean --workspace

# Build the project
build mode="release":
    cargo build --workspace --{{ mode }} --{{ ALL_FEATURES }}
    cargo build --workspace --{{ mode }} --{{ DEF_FEATURES }}
    cargo build --workspace --{{ mode }} --{{ NO_FEATURES }}

# Check all project dependencies
deny:
    cargo deny --workspace --exclude=froglight-codegen --features=bevy check all

# Format all code
fmt:
    cargo fmt
    cargo sort-derives

# Run the code generator
generate arg="":
    RUST_LOG=debug cargo run --package=froglight-codegen --release -- {{ arg }}
    @just fmt

# Show the dependency tree for a specific package
inspect package="froglight":
    cargo tree --invert --package={{ package }}

# Run all workspace tests
test:
    cargo nextest run --workspace --release --{{ ALL_FEATURES }} --{{ EXCLUDE }}
    cargo test --doc --workspace --{{ ALL_FEATURES }} --{{ EXCLUDE }}
    cargo nextest run --workspace --release --{{ DEF_FEATURES }} --{{ EXCLUDE }}
    cargo test --doc --workspace --{{ DEF_FEATURES }} --{{ EXCLUDE }}
    cargo nextest run --workspace --release --{{ NO_FEATURES }}  --{{ EXCLUDE }}
    cargo test --doc --workspace --{{ NO_FEATURES }} --{{ EXCLUDE }}

# Build all profiles and check the size of the resulting binaries
buildsize:
    cargo build --example=frogbot --features=bevy,std,nightly,v26_1,v26_2 --profile=dev
    cargo build --example=frogbot --features=bevy,std,nightly,v26_1,v26_2 --profile=release
    cargo build --example=frogbot --features=bevy,std,nightly,v26_1,v26_2 --profile=distribution
    cargo build --example=frogbot --features=bevy,std,nightly,v26_1,v26_2 --profile=distribution-size
    ls -lha target/*/examples/frogbot

# Check all files for typos
typos:
    typos

# Update all dependencies
update:
    cargo update --verbose
    @echo '{{ CYAN + BOLD }}note{{ NORMAL }}: or, if you have `just` installed, run `just inspect <dep>@<ver>`'

# Update and run all checks
pre-commit: typos update generate clippy test
    @echo '{{ GREEN + BOLD }}Success!{{ NORMAL }} All checks passed!'
