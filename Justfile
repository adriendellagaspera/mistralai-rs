generate:
    python3 scripts/codegen.py generate

check-generated:
    python3 scripts/codegen.py check

update-spec:
    python3 scripts/update_spec.py

test-codegen:
    python3 -m unittest discover -s scripts -p 'test_*.py'
