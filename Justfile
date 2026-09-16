generate:
    python3 tooling/pipeline/build.py generate

check-generated:
    python3 tooling/pipeline/build.py check

sync-openapi:
    python3 tooling/sources/openapi.py

sync-sdk-surface:
    python3 tooling/sources/taxonomy.py pin-latest
    python3 tooling/pipeline/build.py raw
    python3 tooling/sources/taxonomy_inventory.py update
    python3 tooling/pipeline/build.py generate

test-tooling:
    python3 tooling/tests/run.py

validate:
    bash tooling/quality/validate.sh
