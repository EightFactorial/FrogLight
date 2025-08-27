ALL_FEATURES := "features=async,resolver,resolver-sys,nightly"
NO_FEATURES := "no-default-features --features=libm"

# Generate the changelog
changelog path="CHANGELOG.md":
    git-cliff --output {{path}}

# Run the clippy linter
clippy:
    cargo clippy --workspace --{{ALL_FEATURES}} -- -D warnings
    cargo clippy --workspace --{{NO_FEATURES}} -- -D warnings

# Build the project
build mode="release":
    cargo build --package=froglight --{{mode}} --{{ALL_FEATURES}}
    cargo build --package=froglight --{{mode}} --{{NO_FEATURES}}

# Check all project dependencies
deny:
    cargo deny check all

# Run all workspace tests
test:
    cargo test --workspace --{{ALL_FEATURES}}
    cargo test --workspace --{{NO_FEATURES}}

# Check all files for typos
typos:
    typos

# Update all dependencies
update:
    cargo update --verbose
    @echo '{{CYAN+BOLD}}note{{NORMAL}}: or, if you have `just` installed, run `just inspect <dep>@<ver>`'

# Show the dependency tree for a specific package
inspect package="froglight":
    cargo tree --invert --package={{package}}

# Update and run all checks
pre-commit: (update) (deny) (typos) (clippy) (test)
    @echo '{{GREEN+BOLD}}Success!{{NORMAL}} All checks passed!'
