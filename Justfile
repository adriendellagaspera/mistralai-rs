generate:
    python3 sdk-build/build.py generate

check-generated:
    python3 sdk-build/build.py check
    python3 sdk-build/official-sdks/update.py check

sync-openapi:
    python3 sdk-build/openapi/update.py

sync-sdk-surface:
    python3 sdk-build/official-sdks/update.py pin-latest
    python3 sdk-build/build.py raw
    python3 sdk-build/official-sdks/update.py update
    python3 sdk-build/build.py generate

policy:
    python3 .github/scripts/policy.py

format:
    cargo fmt --all --check

lint:
    cargo clippy --locked --all-targets -- -D warnings

test-tooling:
    python3 -m unittest discover -s sdk-build -p 'test_*.py'
    python3 -m unittest discover -s .github/scripts -p 'test_*.py'

test:
    cargo test --locked --all-targets

docs:
    RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

validate:
    python3 sdk-build/policy.py
    python3 -m unittest discover -s sdk-build -p 'test_*.py'
    python3 -m unittest discover -s .github/scripts -p 'test_*.py'
    python3 sdk-build/build.py check
    python3 sdk-build/official-sdks/update.py check
    cargo fmt --all --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked --all-targets
    RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
