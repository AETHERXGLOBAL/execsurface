#!/usr/bin/env python3
"""Mutation red team for ExecSurface experimental typed evidence v1."""
from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path

from p8_a3_typed_evidence_reference_consumer import ContractError, validate_evidence


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def first_effect(value):
    require(bool(value["effects"]), "fixture has no typed effect")
    return value["effects"][0]


def first_raw_write(value):
    for event in value["raw_observation"]["events"]:
        if event.get("event_type") == "file_descriptor_access" and event.get("operation") == "write":
            return event
    raise SystemExit("fixture has no raw fd write")


def run_mutation(name, base, mutate):
    candidate = copy.deepcopy(base)
    mutate(candidate)
    try:
        validate_evidence(candidate)
    except ContractError:
        print(f"REJECTED {name}")
        return
    raise SystemExit(f"mutation unexpectedly accepted: {name}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence", type=Path)
    args = parser.parse_args()

    base = json.loads(args.evidence.read_text(encoding="utf-8"))
    validate_evidence(base)

    mutations = [
        ("schema-version", lambda x: x.__setitem__("schema_version", 2)),
        ("report-kind", lambda x: x.__setitem__("report_kind", "other")),
        ("stability-promotion", lambda x: x.__setitem__("stability", "stable")),
        ("backend-profile", lambda x: x["backend"].__setitem__("profile", "trusted-super-backend")),
        ("health-state", lambda x: x["collection_health"].__setitem__("state", "incomplete_loss")),
        ("warning-code", lambda x: x["collection_health"].__setitem__("warning_codes", ["fabricated"])),
        ("raw-complete", lambda x: x["raw_observation"].__setitem__("complete", False)),
        ("raw-backend", lambda x: x["raw_observation"]["backend"].__setitem__("name", "other-backend")),
        ("effect-target", lambda x: first_effect(x).__setitem__("target", "/tmp/substituted")),
        ("effect-actor", lambda x: first_effect(x).__setitem__("actor", "/bin/substituted")),
        ("effect-proposition", lambda x: first_effect(x).__setitem__("proposition", "file_contents_proven")),
        (
            "effect-guarantee",
            lambda x: first_effect(x)["guarantees"].__setitem__(
                "temporal_bindings", ["pre_operation_intent"]
            ),
        ),
        (
            "drop-raw-write",
            lambda x: x["raw_observation"].__setitem__(
                "events",
                [
                    event
                    for event in x["raw_observation"]["events"]
                    if event is not first_raw_write(x)
                ],
            ),
        ),
        (
            "inject-typed-effect",
            lambda x: x["effects"].append(
                {
                    "proposition": "file_fd_write_effect_observed",
                    "actor": "/bin/injected",
                    "target": "/tmp/injected",
                    "guarantees": copy.deepcopy(first_effect(x)["guarantees"]),
                }
            ),
        ),
        ("remove-typed-effect", lambda x: x["effects"].pop(0)),
        ("repeat-typed-effect", lambda x: x["effects"].append(copy.deepcopy(first_effect(x)))),
        ("verdict-field", lambda x: x.__setitem__("verdict", "PASS")),
        ("byte-count-field", lambda x: first_effect(x).__setitem__("byte_count", 6)),
        ("state-root-field", lambda x: first_effect(x).__setitem__("afterRoot", "sha256:00")),
        ("custody-field", lambda x: x.__setitem__("custody", "independent")),
        ("missing-nonclaim", lambda x: x["does_not_assert"].remove("custody")),
        (
            "unsupported-capability-drift",
            lambda x: x["unsupported_capabilities"].append("fd_read_write_effect"),
        ),
        ("limitations-drift", lambda x: x["limitations"].append("fabricated limitation")),
        (
            "raw-warning-injection",
            lambda x: x["raw_observation"]["warnings"].append(
                {"code": "fabricated", "tid": None, "message": "mutation"}
            ),
        ),
        (
            "raw-write-path-substitution",
            lambda x: first_raw_write(x).__setitem__("path", "/tmp/raw-substituted"),
        ),
        (
            "raw-exec-substitution",
            lambda x: next(
                event
                for event in x["raw_observation"]["events"]
                if event.get("event_type") == "process_exec"
            ).__setitem__("path", "/bin/raw-substituted"),
        ),
        (
            "duplicate-sequence",
            lambda x: x["raw_observation"]["events"][1].__setitem__(
                "sequence", x["raw_observation"]["events"][0]["sequence"]
            ),
        ),
    ]

    for name, mutate in mutations:
        run_mutation(name, base, mutate)

    print(json.dumps({"decision": "MUTATION_REDTEAM_PASS", "rejected": len(mutations)}, sort_keys=True))


if __name__ == "__main__":
    main()
