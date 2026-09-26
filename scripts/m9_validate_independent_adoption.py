#!/usr/bin/env python3
"""Validate M9.2 independent-adoption evidence without fabricating adoption.

This module layers M9.2 provenance/countability rules on the frozen M9 evidence
validator. `--self-test` uses in-memory synthetic records only; synthetic data is
never an adoption record and is never written to the evidence ledger.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import sys
from pathlib import Path
from typing import Any

from m9_validate_evidence import EvidenceError, require, validate_record

COUNTABLE_STATUSES = {"PROVED", "COMPUTATIONAL_EVIDENCE"}
DRIFT_VERDICTS = {"REVIEW", "BLOCK"}


def validate_independent_record(record: dict[str, Any], *, countable: bool) -> None:
    validate_record(record)

    require(
        record["independence_class"] == "INDEPENDENT_USER",
        "M9.2 independent validator accepts only INDEPENDENT_USER records",
    )

    external_reference = record.get("external_reference")
    require(
        isinstance(external_reference, str) and external_reference.strip(),
        "INDEPENDENT_USER requires a public or explicitly consented external_reference",
    )

    provenance = record["provenance"]
    require(provenance["initiated_by"] == "THIRD_PARTY", "record must be third-party initiated")
    require(provenance["executed_by"] == "THIRD_PARTY", "record must be third-party executed")
    require(
        provenance["aether_x_material_involvement"] is False,
        "material AETHER X involvement makes the record non-independent",
    )
    require(
        isinstance(provenance["third_party_attestation"], str)
        and provenance["third_party_attestation"].strip(),
        "independent record requires a non-empty third-party attestation",
    )
    require(record["project"]["modified_by_aether_x"] is False, "AETHER X-modified workload is not independent")

    if not countable:
        return

    require(
        record["claim_status"] in COUNTABLE_STATUSES,
        "countable adoption requires PROVED or COMPUTATIONAL_EVIDENCE status",
    )

    outcome = record["outcome"]
    for field in ("installation", "doctor", "baseline", "rerun_check"):
        require(outcome[field] == "PASS", f"countable adoption requires outcome.{field}=PASS")
    require(outcome["comparable"] is True, "countable adoption requires comparable=true")
    require(outcome["complete"] is True, "countable adoption requires complete=true")
    require(outcome["operational_error"] is None, "countable adoption cannot have unresolved operational_error")
    require(
        outcome.get("suspected_false_negative", False) is False,
        "countable adoption cannot contain an unresolved suspected false negative",
    )


def has_countable_drift(record: dict[str, Any]) -> bool:
    workflow = record["workflow"]
    outcome = record["outcome"]
    verdict = str(outcome.get("verdict") or "").upper()
    return (
        workflow["drift_case_attempted"] is True
        and outcome["drift_case"] == "PASS"
        and verdict in DRIFT_VERDICTS
        and outcome["comparable"] is True
        and outcome["complete"] is True
    )


def validate_close_gate(records: list[dict[str, Any]]) -> None:
    require(len(records) >= 2, "M9.2 close gate requires at least two independent records")
    for record in records:
        validate_independent_record(record, countable=True)

    repositories = {str(record["project"]["repository"]).strip() for record in records}
    references = {str(record["external_reference"]).strip() for record in records}
    require(len(repositories) >= 2, "M9.2 close gate requires at least two distinct external project repositories")
    require(len(references) >= 2, "M9.2 close gate requires at least two distinct external references")
    require(any(has_countable_drift(record) for record in records), "M9.2 close gate requires at least one comparable complete REVIEW/BLOCK drift case")


def _synthetic_record(evidence_id: str, repository: str, reference: str, attestation: str) -> dict[str, Any]:
    return {
        "schema_version": "m9-evidence-v1",
        "evidence_id": evidence_id,
        "timestamp_utc": "2026-09-27T00:00:00Z",
        "independence_class": "INDEPENDENT_USER",
        "claim_status": "COMPUTATIONAL_EVIDENCE",
        "external_reference": reference,
        "provenance": {
            "initiated_by": "THIRD_PARTY",
            "executed_by": "THIRD_PARTY",
            "aether_x_material_involvement": False,
            "third_party_attestation": attestation,
        },
        "project": {
            "name": "synthetic-validator-only",
            "repository": repository,
            "revision": "0123456789abcdef",
            "modified_by_aether_x": False,
        },
        "execsurface": {
            "version": "0.1.0-alpha.2",
            "source_sha": "0123456789abcdef",
            "install_path": "cargo-install-pinned",
            "backend": "ptrace",
        },
        "host": {
            "os": "Linux",
            "kernel": "synthetic",
            "architecture": "x86_64",
            "cpu_description": "synthetic-validator-only",
            "runner_image": None,
        },
        "workflow": {
            "command": "/bin/bash -lc 'true'",
            "baseline_attempted": True,
            "rerun_check_attempted": True,
            "drift_case_attempted": False,
            "notes": "in-memory validator self-test only; never evidence",
        },
        "outcome": {
            "installation": "PASS",
            "doctor": "PASS",
            "baseline": "PASS",
            "rerun_check": "PASS",
            "drift_case": "NOT_RUN",
            "verdict": "PASS",
            "comparable": True,
            "complete": True,
            "operational_error": None,
            "false_positive_reported": False,
            "suspected_false_negative": False,
            "usability_friction": None,
        },
        "privacy": {
            "metadata_only_confirmed": True,
            "prohibited_sensitive_payload_collected": False,
            "notes": "synthetic-validator-only",
        },
        "performance": None,
        "artifacts": [
            {
                "name": "synthetic-validator-only",
                "sha256": "0" * 64,
                "location": "synthetic://validator-only",
            }
        ],
        "failures_exclusions": [],
        "reproduction": {
            "instructions": "synthetic validator self-test only",
            "expected_scope": "never count as evidence",
        },
    }


def self_test() -> None:
    first = _synthetic_record(
        "M9-SYNTHETIC-VALIDATOR-1",
        "https://example.invalid/third-party-one/project-a",
        "https://example.invalid/third-party-one/evidence/1",
        "synthetic attestation one",
    )
    second = _synthetic_record(
        "M9-SYNTHETIC-VALIDATOR-2",
        "https://example.invalid/third-party-two/project-b",
        "https://example.invalid/third-party-two/evidence/2",
        "synthetic attestation two",
    )
    first["workflow"]["drift_case_attempted"] = True
    first["outcome"]["drift_case"] = "PASS"
    first["outcome"]["verdict"] = "REVIEW"

    validate_close_gate([first, second])

    bad = copy.deepcopy(first)
    bad["provenance"]["aether_x_material_involvement"] = True
    try:
        validate_independent_record(bad, countable=True)
    except EvidenceError:
        pass
    else:
        raise EvidenceError("self-test expected material AETHER X involvement to fail")

    same_repo = copy.deepcopy(second)
    same_repo["project"]["repository"] = first["project"]["repository"]
    try:
        validate_close_gate([first, same_repo])
    except EvidenceError:
        pass
    else:
        raise EvidenceError("self-test expected duplicate repository close gate to fail")

    print("M9_2_VALIDATOR_SELF_TEST_PASS")


def load_record(path: Path) -> tuple[dict[str, Any], str]:
    raw = path.read_bytes()
    record = json.loads(raw)
    require(isinstance(record, dict), f"{path}: top-level JSON must be an object")
    return record, hashlib.sha256(raw).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("records", nargs="*", type=Path)
    parser.add_argument("--countable", action="store_true", help="require successful-adoption fields")
    parser.add_argument("--close-gate", action="store_true", help="validate aggregate M9.2 close gate")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    try:
        if args.self_test:
            self_test()
            return 0

        require(bool(args.records), "provide at least one evidence record")
        loaded = [load_record(path) for path in args.records]
        records = [record for record, _ in loaded]

        if args.close_gate:
            validate_close_gate(records)
            for path, (record, digest) in zip(args.records, loaded):
                print(f"M9_INDEPENDENT_EVIDENCE_COUNTABLE id={record['evidence_id']} sha256={digest} path={path}")
            print(f"M9_2_CLOSE_GATE_AUTOMATED_PASS records={len(records)}")
            print("M9_2_CLOSE_GATE_REQUIRES_HUMAN_PROVENANCE_REVIEW=true")
            return 0

        for path, (record, digest) in zip(args.records, loaded):
            validate_independent_record(record, countable=args.countable)
            print(f"M9_INDEPENDENT_EVIDENCE_VALID id={record['evidence_id']} sha256={digest} path={path}")
        return 0
    except (OSError, json.JSONDecodeError, EvidenceError, KeyError, TypeError, ValueError) as exc:
        print(f"M9_INDEPENDENT_EVIDENCE_INVALID: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
