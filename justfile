HERE := justfile_directory()

build:
    cargo build

test:
    cargo test --all --all-features --all-targets
    cargo test --all --no-default-features --all-targets
    cargo test --doc
    cargo test --all --no-default-features --features "jiff02" --all-targets
    cargo test --all --no-default-features --features "chrono04" --all-targets
    cargo test --all --no-default-features --features "time03" --all-targets

[group('lint')]
lint: clippy machete fmt sort audit

# run clippy in --fix mode
[group('lint')]
clippy:
    # keep this nightly to keep it in sync with CI
    cargo +nightly clippy --fix --allow-dirty --all-features --all-targets

# run cargo-machete in --fix mode
[group('lint')]
machete:
    cargo machete --fix

# run cargo-fmt in --fix mode
[group('lint')]
fmt:
    # this has to be nightly to get import sorting working correctly
    cargo +nightly fmt

# run cargo sort
[group('lint')]
sort:
    cargo sort --no-format -o package,lib,bin,features,dependencies,dev-dependencies,lints

# run security lints
[group('lint')]
audit:
    cargo deny check -c {{ HERE / "deny.toml" }}
