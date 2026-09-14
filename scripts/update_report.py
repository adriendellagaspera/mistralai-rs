"""Add a bounded, mechanical diff summary after all validation has passed."""

from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)


def operations(spec):
    return set(re.findall(r"^\s+operationId:\s+(.+)$", spec, re.MULTILINE))


def main():
    changed = git("diff", "--name-only").splitlines()
    changed += git("ls-files", "--others", "--exclude-standard").splitlines()
    unexpected = [p for p in changed if p != "codegen.lock"
                  and not p.startswith(("spec/", "src/generated/", "src/sdk/"))]
    if unexpected:
        raise ValueError(f"Unexpected files modified during generation: {unexpected}")
    before = operations(git("show", "HEAD:spec/openapi.yaml"))
    after = operations((ROOT / "spec/openapi.yaml").read_text())
    with (ROOT / "update-report.md").open("a") as report:
        report.write("\nSpec operation changes (whole upstream API):\n")
        for label, values in [("Added", after - before), ("Removed", before - after)]:
            report.write(f"- {label}: {len(values)}\n")
            for value in sorted(values)[:30]:
                report.write(f"  - `{value}`\n")
        report.write("\nChanged operation schemas may still be breaking even with no added/removed operations.\n")
        report.write("\nGenerated/spec diff summary:\n\n```text\n")
        report.write(git("diff", "--stat", "--", "spec", "src/generated", "src/sdk",
                         "codegen.lock"))
        report.write("```\n\nChecks passed in the update workflow: spec hash, generator --check, "
                     "byte-for-byte regeneration, codegen/update tests, cargo fmt, cargo check, "
                     "cargo clippy (-D warnings), unit/integration/example compilation and doctests.\n\n"
                     "No live API calls were made. GITHUB_TOKEN may suppress downstream CI events; "
                     "validation ran here before this PR was created/updated.\n")


if __name__ == "__main__":
    main()
