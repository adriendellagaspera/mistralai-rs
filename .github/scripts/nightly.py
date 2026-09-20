"""Collect independent nightly checks without masking failures at the final gate.

Each job writes one JSON result file. A failed command is recorded, rather than
terminating the job and hiding unrelated diagnostics. The aggregate gate fails
only after the report and any review PR have been published.
"""

from __future__ import annotations

import argparse
from collections import deque
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import sys
import time

RESULTS = Path(".nightly/results")
STATES = {"PASS", "FAIL", "BLOCKED", "NOT_APPLICABLE"}
BASELINE = {
    "sources": ("openapi", "official_pins"),
    "baseline-generation": ("generation", "official_surface"),
    "baseline-format": ("sdk_format", "builder_format"),
    "baseline-lint": ("sdk_clippy", "builder_clippy"),
    "baseline-tests": ("sdk_tests", "builder_tests"),
    "baseline-docs": ("sdk_doc_tests", "sdk_docs"),
    "baseline-tooling": ("python_tests",),
}
CANDIDATE = {
    "candidate": ("raw", "surface", "generate", "check", "official_check"),
    "candidate-format": ("sdk_format", "builder_format"),
    "candidate-lint": ("sdk_clippy", "builder_clippy"),
    "candidate-tests": ("sdk_tests", "builder_tests"),
    "candidate-docs": ("sdk_doc_tests", "sdk_docs"),
    "candidate-api": ("api_review",),
}


def result_path(job: str) -> Path:
    if not job or "/" in job or ".." in job:
        raise ValueError(f"Invalid job identifier: {job!r}")
    return RESULTS / f"{job}.json"


def load(job: str) -> dict:
    path = result_path(job)
    if path.exists():
        return json.loads(path.read_text())
    return {"job": job, "checks": {}}


def record(job: str, check: str, status: str, detail: str = "", duration: float = 0) -> None:
    if status not in STATES:
        raise ValueError(f"Invalid result: {status}")
    result = load(job)
    result["checks"][check] = {
        "status": status,
        "detail": detail[-3000:],
        "duration_seconds": round(duration, 2),
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }
    path = result_path(job)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(f"nightly: {job}/{check}: {status}: {detail}", flush=True)


def run(job: str, check: str, command: list[str]) -> None:
    if not command:
        raise ValueError("A command is required")
    started = time.monotonic()
    print(f"nightly: running {job}/{check}: {command!r}", flush=True)
    try:
        tail: deque[str] = deque(maxlen=12)
        with subprocess.Popen(
            command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            text=True, errors="replace", bufsize=1,
        ) as process:
            assert process.stdout is not None
            for line in process.stdout:
                print(line, end="", flush=True)
                tail.append(line.rstrip())
            returncode = process.wait()
        status = "PASS" if returncode == 0 else "FAIL"
        detail = " ".join(command) + f" (exit {returncode})"
        if returncode:
            detail += "\n" + "\n".join(tail)
    except OSError as error:
        status, detail = "FAIL", str(error)
    record(job, check, status, detail, time.monotonic() - started)


def checked(job: str, check: str) -> bool:
    return load(job)["checks"].get(check, {}).get("status") == "PASS"


def combined(changed: bool) -> tuple[list[dict], bool]:
    expected = {**BASELINE, **(CANDIDATE if changed else {})}
    rows: list[dict] = []
    healthy = True
    for job, names in expected.items():
        checks = load(job)["checks"]
        for name in names:
            item = checks.get(name, {"status": "BLOCKED", "detail": "Missing result or job artifact"})
            status = item["status"]
            rows.append({"job": job, "check": name, **item})
            healthy &= status in {"PASS", "NOT_APPLICABLE"}
    return rows, healthy


def summary(changed: bool, output: Path, run_url: str) -> None:
    rows, healthy = combined(changed)
    counts = {state: sum(row["status"] == state for row in rows) for state in sorted(STATES)}
    lines = [
        "# Mistral SDK nightly",
        "",
        f"Run: {run_url}",
        f"Candidate source changes: {'yes' if changed else 'no'}",
        f"Overall: {'PASS' if healthy else 'FAIL'} — "
        + ", ".join(f"{state}={count}" for state, count in counts.items()),
        "",
        "| Job | Check | Status | Detail |",
        "| --- | --- | --- | --- |",
    ]
    for row in rows:
        detail = str(row.get("detail", "")).replace("|", r"\|").replace("\n", " ")
        lines.append(f"| {row['job']} | {row['check']} | {row['status']} | {detail} |")
    lines += [
        "",
        "A blocked candidate check does not invalidate passing baseline checks.",
        "Review source pins, generated coverage and the Rust API before merging a candidate.",
        "",
    ]
    output.write_text("\n".join(lines))
    print(output.read_text(), flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="action", required=True)
    command = sub.add_parser("run")
    command.add_argument("job")
    command.add_argument("check")
    command.add_argument("command", nargs=argparse.REMAINDER)
    state = sub.add_parser("record")
    state.add_argument("job")
    state.add_argument("check")
    state.add_argument("status", choices=sorted(STATES))
    state.add_argument("detail")
    ok = sub.add_parser("ok")
    ok.add_argument("job")
    ok.add_argument("check")
    for action in ("summary", "gate"):
        report = sub.add_parser(action)
        report.add_argument("--changed", action="store_true")
        if action == "summary":
            report.add_argument("--output", type=Path, required=True)
            report.add_argument("--run-url", required=True)
    args = parser.parse_args()
    if args.action == "run":
        cmd = args.command[1:] if args.command[:1] == ["--"] else args.command
        run(args.job, args.check, cmd)
    elif args.action == "record":
        record(args.job, args.check, args.status, args.detail)
    elif args.action == "ok":
        return 0 if checked(args.job, args.check) else 1
    elif args.action == "summary":
        summary(args.changed, args.output, args.run_url)
    else:
        _, healthy = combined(args.changed)
        return 0 if healthy else 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
