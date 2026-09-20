"""Fail closed on SDK operation identity or coverage regressions.

This guard is independent of the raw generator and SDK compiler.
Coverage progress may reduce the approved rejection set but cannot turn an
accepted operation back into a rejection or add an unreviewed override.
"""

from __future__ import annotations

from collections import Counter
from collections.abc import Iterable, Mapping


def validate_coverage(
    report: Mapping[str, dict],
    baseline: dict,
    source_operation_ids: Iterable[str],
) -> dict[str, int]:
    if baseline.get("schema_version") != 1:
        raise ValueError("unsupported SDK coverage baseline schema")
    source_ids = set(source_operation_ids)
    actual_ids = set(report)
    if len(source_ids) != baseline["total_operations"] or actual_ids != source_ids:
        missing = sorted(source_ids - actual_ids)
        extra = sorted(actual_ids - source_ids)
        raise ValueError(
            "SDK operation inventory drift: "
            f"source={len(source_ids)}, report={len(actual_ids)}, "
            f"expected={baseline['total_operations']}; missing={missing}; extra={extra}"
        )

    allowed_rejected = baseline["previously_rejected_operations"]
    allowed_overridden = baseline["approved_overridden_operations"]
    if (
        len(set(allowed_rejected)) != len(allowed_rejected)
        or len(set(allowed_overridden)) != len(allowed_overridden)
        or set(allowed_rejected) & set(allowed_overridden)
        or not set(allowed_rejected + allowed_overridden) <= source_ids
    ):
        raise ValueError("SDK coverage baseline has invalid or unknown operation identities")
    counts = Counter(item.get("status") for item in report.values())
    if set(counts) - {"derived", "overridden", "rejected"}:
        raise ValueError(f"unknown SDK derivation statuses: {dict(counts)}")
    rejected = {op for op, item in report.items() if item["status"] == "rejected"}
    overridden = {op for op, item in report.items() if item["status"] == "overridden"}
    new_rejections = sorted(rejected - set(allowed_rejected))
    new_overrides = sorted(overridden - set(allowed_overridden))
    if new_rejections or new_overrides:
        raise ValueError(
            f"SDK coverage regression: new_rejections={new_rejections}; "
            f"unreviewed_overrides={new_overrides}"
        )
    return dict(sorted(counts.items()))
