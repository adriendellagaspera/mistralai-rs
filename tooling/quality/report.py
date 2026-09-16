"""Write a bounded mechanical update summary after validation succeeds."""
from pathlib import Path
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
LOCK = ROOT / "tooling/sources/lock.json"
TAXONOMY = ROOT / "tooling/sources/taxonomy.json"

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)

def from_head(path):
    return json.loads(git("show", f"HEAD:{path}"))

def operations(spec):
    return set(re.findall(r"^\\s+operationId:\\s+(.+)$", spec, re.MULTILINE))

def mapping_delta(before, after):
    before, after = before.get("operations", {}), after.get("operations", {})
    return (sorted(after.keys() - before.keys()), sorted(before.keys() - after.keys()),
            sorted(key for key in before.keys() & after.keys() if before[key] != after[key]))

def main():
    changed = git("diff", "--name-only").splitlines()
    changed += git("ls-files", "--others", "--exclude-standard").splitlines()
    allowed = ("tooling/sources/", "src/generated/", "src/sdk/")
    unexpected = [p for p in changed if not p.startswith(allowed) and p != "update-report.md"]
    if unexpected:
        raise ValueError(f"Unexpected files modified during source update: {unexpected}")

    old_lock = from_head("tooling/sources/lock.json")
    new_lock = json.loads(LOCK.read_text())
    old_taxonomy = from_head("tooling/sources/taxonomy.json")
    new_taxonomy = json.loads(TAXONOMY.read_text())
    added_paths, removed_paths, changed_paths = mapping_delta(old_taxonomy, new_taxonomy)

    old_spec = git("show", "HEAD:tooling/sources/openapi/openapi.yaml")
    new_spec = (ROOT / "tooling/sources/openapi/openapi.yaml").read_text()
    before, after = operations(old_spec), operations(new_spec)

    with (ROOT / "update-report.md").open("w") as report:
        report.write("Update the unofficial Rust SDK from tracked Mistral sources.\
\
")
        for label, repo_key, commit_key in (
            ("OpenAPI", "upstream_repository", "upstream_commit"),
            ("Python SDK", "official_python_sdk_repository", "official_python_sdk_commit"),
            ("TypeScript SDK", "official_typescript_sdk_repository", "official_typescript_sdk_commit"),
        ):
            old, new = old_lock[commit_key], new_lock[commit_key]
            report.write(f"- {label}: `{old}` -> `{new}` ({new_lock[repo_key]})\
")
        report.write("\
OpenAPI operation changes:\
")
        for label, values in (("Added", after - before), ("Removed", before - after)):
            report.write(f"- {label}: {len(values)}\
")
            for value in sorted(values)[:30]:
                report.write(f"  - `{value}`\
")
        report.write("\
Official SDK taxonomy changes:\
")
        for label, values in (("Added mappings", added_paths), ("Removed mappings", removed_paths), ("Changed mappings", changed_paths)):
            report.write(f"- {label}: {len(values)}\
")
            for value in values[:30]:
                report.write(f"  - `{value}`\
")
        report.write(f"- Python/TS alias differences: {len(new_taxonomy.get('cross_sdk_alias_differences', []))}\
")
        report.write(f"- Python/TS divergences requiring review: {len(new_taxonomy.get('cross_sdk_divergences', []))}\
")
        report.write(f"- SDK-only surface digest changed: {old_taxonomy.get('sdk_only_digest') != new_taxonomy.get('sdk_only_digest')}\
")
        report.write("\
Changed source/generated diff summary:\
\
```text\
")
        report.write(git("diff", "--stat", "--", "tooling/sources", "src/generated", "src/sdk"))
        report.write("```\
\
Checks passed: pinned-source verification, raw generation, taxonomy harvesting, byte-for-byte regeneration, tooling tests, cargo fmt/check/clippy/tests/docs, coverage probe and public API review.\
")

if __name__ == "__main__":
    main()
