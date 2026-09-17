generate:
    python3 tooling/pipeline/build.py generate

check-generated:
    python3 tooling/pipeline/build.py check
    python3 tooling/sources/taxonomy_inventory.py check
    cargo fetch --locked
    python3 tooling/pipeline/build.py probe

sync-openapi:
    python3 tooling/sources/openapi.py

sync-sdk-surface:
    python3 tooling/sources/taxonomy.py pin-latest
    python3 tooling/pipeline/build.py raw
    python3 tooling/sources/taxonomy_inventory.py update
    python3 tooling/pipeline/build.py generate

policy:
    python3 tooling/quality/agent_contract.py

format:
    cargo fmt --all --check

lint:
    cargo clippy --locked --all-targets -- -D warnings

test-tooling:
    python3 tooling/tests/run_pinned.py

test:
    cargo test --locked --all-targets

docs:
    RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

validate:
    bash tooling/quality/validate.sh
