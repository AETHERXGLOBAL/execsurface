#!/usr/bin/env python3
"""Validate core M9 evidence semantics without mutating evidence.

This validator intentionally re-checks high-value invariants that are easy to
misrepresent with structurally valid JSON: independence provenance, privacy,
ptrace authority, sample counts, derived performance statistics, and the
predeclared M6.5 reopen trigger.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import statistics
import sys
from pathlib import Path
from typing import Any

ALLOWED_CLASSES = {
    "INDEPENDENT_USER",
    "ZERO_CONTACT_EXTERNAL_REPRO",
    "COLLABORATIVE_EXTERNAL",
    "INTERNAL_FIXTURE",
}
ALLOWED_STATUSES = {"PROVED", "COMPUTATIONAL_EVIDENCE", "PARTIAL", "OPEN", "KILLED"}


class EvidenceError(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise EvidenceError(message)


def close(a: float, b: float, *, rel: float = 1e-9, abs_: float = 1e-6) -> bool:
    return math.isclose(a, b, rel_tol=rel, abs_tol=abs_)


def require_keys(obj: dict[str, Any], keys: set[str], where: str) -> None:
    missing = sorted(keys - obj.keys())
    require(not missing, f"{where}: missing required keys: {missing}")


def validate_record(record: dict[str, Any]) -> None:
    require_keys(
        record,
        {
            "schema_version",
            "evidence_id",
            "timestamp_utc",
            "independence_class",
            "claim_status",
            "provenance",
            "project",
            "execsurface",
            "host",
            "workflow",
            "outcome",
            "privacy",
            "artifacts",
            "failures_exclusions",
            "reproduction",
        },
        "record",
    )

    require(record["schema_version"] == "m9-evidence-v1", "unsupported schema_version")
    require(str(record["evidence_id"]).startswith("M9-"), "evidence_id must start with M9-")
    require(record["independence_class"] in ALLOWED_CLASSES, "invalid independence_class")
    require(record["claim_status"] in ALLOWED_STATUSES, "invalid claim_status")

    provenance = record["provenance"]
    project = record["project"]
    require(isinstance(provenance, dict), "provenance must be an object")
    require(isinstance(project, dict), "project must be an object")
    require_keys(
        provenance,
        {"initiated_by", "executed_by", "aether_x_material_involvement", "third_party_attestation"},
        "provenance",
    )
    require_keys(project, {"name", "repository", "revision", "modified_by_aether_x"}, "project")

    klass = record["independence_class"]
    if klass == "INDEPENDENT_USER":
        require(provenance["initiated_by"] == "THIRD_PARTY", "INDEPENDENT_USER must be third-party initiated")
        require(provenance["executed_by"] == "THIRD_PARTY", "INDEPENDENT_USER must be third-party executed")
        require(provenance["aether_x_material_involvement"] is False, "INDEPENDENT_USER cannot have material AETHER X involvement")
        require(project["modified_by_aether_x"] is False, "INDEPENDENT_USER project/workload cannot be modified by AETHER X")
        attestation = provenance["third_party_attestation"]
        require(isinstance(attestation, str) and attestation.strip(), "INDEPENDENT_USER requires non-empty third-party attestation")
    elif klass == "ZERO_CONTACT_EXTERNAL_REPRO":
        require(provenance["initiated_by"] == "AETHER_X", "ZERO_CONTACT_EXTERNAL_REPRO must be AETHER X initiated")
        require(provenance["executed_by"] == "AETHER_X", "ZERO_CONTACT_EXTERNAL_REPRO must be AETHER X executed")
        require(provenance["aether_x_material_involvement"] is True, "ZERO_CONTACT_EXTERNAL_REPRO must disclose AETHER X involvement")
        require(project["modified_by_aether_x"] is False, "ZERO_CONTACT_EXTERNAL_REPRO must use unmodified external workload")
        require(provenance["third_party_attestation"] is None, "zero-contact evidence cannot invent a third-party attestation")
    elif klass == "INTERNAL_FIXTURE":
        require(provenance["initiated_by"] == "AETHER_X", "INTERNAL_FIXTURE must be AETHER X initiated")
        require(provenance["executed_by"] == "AETHER_X", "INTERNAL_FIXTURE must be AETHER X executed")
        require(provenance["aether_x_material_involvement"] is True, "INTERNAL_FIXTURE must disclose AETHER X involvement")

    execsurface = record["execsurface"]
    require(execsurface.get("backend") == "ptrace", "M9 accepted evidence must retain ptrace public authority")

    privacy = record["privacy"]
    require(privacy.get("prohibited_sensitive_payload_collected") is False, "record contains prohibited sensitive payload evidence")
    require(privacy.get("metadata_only_confirmed") is True, "metadata-only privacy confirmation required")

    artifacts = record["artifacts"]
    require(isinstance(artifacts, list) and artifacts, "at least one artifact is required")
    for index, artifact in enumerate(artifacts):
        digest = artifact.get("sha256")
        require(isinstance(digest, str) and len(digest) == 64, f"artifact[{index}] sha256 must be 64 hex chars")
        try:
            bytes.fromhex(digest)
        except ValueError as exc:
            raise EvidenceError(f"artifact[{index}] sha256 is not hexadecimal") from exc

    for index, failure in enumerate(record["failures_exclusions"]):
        require(failure.get("retained") is True, f"failures_exclusions[{index}] must be retained")
        require(str(failure.get("description", "")).strip() != "", f"failures_exclusions[{index}] needs description")

    performance = record.get("performance")
    if performance is not None:
        direct = performance["measured_direct_ms"]
        ptrace = performance["measured_ptrace_ms"]
        warm_direct = performance["warmup_direct_ms"]
        warm_ptrace = performance["warmup_ptrace_ms"]
        require(len(warm_direct) == 3 and len(warm_ptrace) == 3, "performance requires exactly 3 warmups per mode")
        require(len(direct) == 15 and len(ptrace) == 15, "performance requires exactly 15 measured samples per mode")
        require(all(float(x) >= 0 for x in warm_direct + warm_ptrace + direct + ptrace), "timings must be non-negative")

        d = float(statistics.median(direct))
        p = float(statistics.median(ptrace))
        overhead = p - d
        ratio = p / d if d > 0 else math.inf
        trigger = ((d >= 100.0 and ratio > 2.0) or overhead > 500.0)

        require(close(float(performance["median_direct_ms"]), d), "median_direct_ms does not match raw samples")
        require(close(float(performance["median_ptrace_ms"]), p), "median_ptrace_ms does not match raw samples")
        require(close(float(performance["median_absolute_overhead_ms"]), overhead), "median_absolute_overhead_ms mismatch")
        if math.isfinite(ratio):
            require(close(float(performance["slowdown_ratio"]), ratio), "slowdown_ratio mismatch")
        require(bool(performance["m65_trigger_crossed"]) is trigger, "m65_trigger_crossed does not match predeclared rule")
        require(float(performance["timeout_seconds"]) > 0, "timeout_seconds must be positive")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("record", type=Path)
    args = parser.parse_args()

    try:
        raw = args.record.read_bytes()
        record = json.loads(raw)
        require(isinstance(record, dict), "top-level JSON must be an object")
        validate_record(record)
    except (OSError, json.JSONDecodeError, EvidenceError, KeyError, TypeError, ValueError) as exc:
        print(f"M9_EVIDENCE_INVALID: {exc}", file=sys.stderr)
        return 1

    print(f"M9_EVIDENCE_VALID id={record['evidence_id']} sha256={hashlib.sha256(raw).hexdigest()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
