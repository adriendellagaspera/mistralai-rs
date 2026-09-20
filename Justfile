generate:
    cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- generate

check-generated:
    cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- check
    python3 sdk-build/official-sdks/update.py check

sync-openapi:
    python3 sdk-build/openapi/update.py

sync-sdk-surface:
    python3 sdk-build/official-sdks/update.py pin-latest
    cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- raw
    python3 sdk-build/official-sdks/update.py update
    cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- generate

policy:
    python3 .github/scripts/policy.py

format:
    cargo fmt --all --check
    cargo fmt --manifest-path sdk-build/Cargo.toml --check

lint:
    cargo clippy --locked --all-targets -- -D warnings
    cargo clippy --locked --manifest-path sdk-build/Cargo.toml --all-targets -- -D warnings

test-tooling:
    cargo test --locked --manifest-path sdk-build/Cargo.toml --all-targets
    python3 -m unittest discover -s sdk-build -p 'test_*.py'
    python3 -m unittest discover -s .github/scripts -p 'test_*.py'

test:
    cargo test --locked --all-targets

docs:
    RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

validate:
    python3 .github/scripts/policy.py
    cargo test --locked --manifest-path sdk-build/Cargo.toml --all-targets
    python3 -m unittest discover -s sdk-build -p 'test_*.py'
    python3 -m unittest discover -s .github/scripts -p 'test_*.py'
    cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- check
    python3 sdk-build/official-sdks/update.py check
    cargo fmt --all --check
    cargo fmt --manifest-path sdk-build/Cargo.toml --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo clippy --locked --manifest-path sdk-build/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --all-targets
    RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
