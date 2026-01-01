ALL_FEATURES := "all-features"
DEF_FEATURES := "features=bevy,std"
NO_FEATURES := "features=libm --no-default-features"

# Generate the changelog
changelog path="CHANGELOG.md":
    git-cliff --output {{ path }}

# Run the clippy linter
clippy:
    cargo sort-derives
    cargo clippy --workspace --{{ ALL_FEATURES }} -- -D warnings
    cargo clippy --workspace --{{ DEF_FEATURES }} -- -D warnings
    cargo clippy --workspace --{{ NO_FEATURES }} -- -D warnings

# Clean up all build artifacts
clean:
    cargo clean

# Build the project
build mode="release":
    cargo build --workspace --{{ mode }} --{{ ALL_FEATURES }}
    cargo build --workspace --{{ mode }} --{{ DEF_FEATURES }}
    cargo build --workspace --{{ mode }} --{{ NO_FEATURES }}

# Check all project dependencies
deny:
    cargo deny check all

# Run all workspace tests
test:
    cargo nextest run --workspace --release --{{ ALL_FEATURES }}
    cargo test --doc --workspace --{{ ALL_FEATURES }}
    cargo nextest run --workspace --release --{{ DEF_FEATURES }}
    cargo test --doc --workspace --{{ DEF_FEATURES }}
    cargo nextest run --workspace --release --{{ NO_FEATURES }}
    cargo test --doc --workspace --{{ NO_FEATURES }}

# Check all files for typos
typos:
    typos

# Update all dependencies
update:
    cargo update --verbose
    @echo '{{ CYAN + BOLD }}note{{ NORMAL }}: or, if you have `just` installed, run `just inspect <dep>@<ver>`'

# Show the dependency tree for a specific package
inspect package="froglight":
    cargo tree --invert --package={{ package }}

# Update and run all checks
pre-commit: clean update deny typos clippy test
    @echo '{{ GREEN + BOLD }}Success!{{ NORMAL }} All checks passed!'
