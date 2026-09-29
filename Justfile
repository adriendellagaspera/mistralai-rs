generate:
    python3 sdk-build/fern/build.py generate

check-generated:
    python3 sdk-build/fern/build.py check

check-source-evidence:
    python3 sdk-build/official-sdks/update.py check

sync-openapi:
    python3 sdk-build/openapi/update.py

sync-sdk-surface:
    python3 sdk-build/official-sdks/update.py pin-latest
    python3 sdk-build/official-sdks/update.py update
    python3 sdk-build/fern/build.py generate

policy:
    python3 .github/scripts/policy.py

format:
    cargo fmt --all --check

lint:
    cargo clippy --locked --all-features --all-targets -- -D clippy::correctness -D clippy::suspicious

test-tooling:
    python3 -m unittest discover -s sdk-build -p 'test_*.py'
    python3 -m unittest discover -s .github/scripts -p 'test_*.py'

test:
    cargo test --locked --all-features --all-targets

docs:
    cargo doc --locked --all-features --no-deps

validate:
    python3 .github/scripts/policy.py
    python3 sdk-build/fern/build.py check
    python3 sdk-build/official-sdks/update.py check
    cargo fmt --all --check
    cargo check --locked --all-features
    cargo clippy --locked --all-features --all-targets -- -D clippy::correctness -D clippy::suspicious
    cargo test --locked --all-features --all-targets
    cargo doc --locked --all-features --no-deps
