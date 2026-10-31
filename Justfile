default: fmt clippy

build:
    cargo build --all-targets

test:
    cargo test --all-features --workspace

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings
