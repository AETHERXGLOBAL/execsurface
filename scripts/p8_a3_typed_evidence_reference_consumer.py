#!/usr/bin/env python3
"""Fail-closed reference consumer for ExecSurface experimental typed evidence v1.

This intentionally does not import ExecSurface code. It re-derives the bounded
meaning from the evidence artifact itself.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

TOP_KEYS = {
    "schema_version",
    "report_kind",
    "stability",
    "backend",
    "collection_health",
    "unsupported_capabilities",
    "effects",
    "limitations",
    "does_not_assert",
    "raw_observation",
}
BACKEND_KEYS = {
    "profile",
    "implementation_version",
    "platform",
    "architecture",
    "privacy_profile",
}
HEALTH_KEYS = {"state", "warning_codes"}
RAW_TOP_KEYS = {"schema_version", "backend", "complete", "outcome", "events", "warnings"}
RAW_BACKEND_KEYS = {"name", "platform", "architecture", "capabilities", "limitations"}
RAW_OUTCOME_KEYS = {"exit_code", "signal"}
RAW_WARNING_KEYS = {"code", "tid", "message"}
EFFECT_KEYS = {"proposition", "actor", "target", "guarantees"}
GUARANTEE_KEYS = {
    "observation_points",
    "identity_bases",
    "temporal_bindings",
    "causal_bindings",
}
EXPECTED_UNSUPPORTED = {"causal_executable_chain", "process_exit"}
EXPECTED_NONCLAIMS = {
    "policy_verdict",
    "vulnerability_free",
    "exact_transferred_byte_count",
    "file_contents",
    "before_after_state_roots",
    "custody",
    "trusted_time",
    "signature_as_behavioral_authority",
    "causal_source_code_provenance",
}
FORBIDDEN_KEYS = {
    "verdict",
    "pass_eligible",
    "bytes_transferred",
    "byte_count",
    "beforeRoot",
    "afterRoot",
    "preStateDigest",
    "postStateDigest",
    "authority",
    "signature",
    "custody",
}
KNOWN_HEALTH = {
    "complete",
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_capability",
    "incomplete_ambiguity",
    "error",
}


class ContractError(ValueError):
    pass


def require(condition: bool, code: str) -> None:
    if not condition:
        raise ContractError(code)


def require_object(value: Any, code: str) -> dict[str, Any]:
    require(isinstance(value, dict), code)
    return value


def require_string_list(value: Any, code: str) -> list[str]:
    require(isinstance(value, list), code)
    require(all(isinstance(item, str) and item for item in value), code)
    return value


def walk_keys(value: Any):
    if isinstance(value, dict):
        for key, nested in value.items():
            yield key
            yield from walk_keys(nested)
    elif isinstance(value, list):
        for nested in value:
            yield from walk_keys(nested)


def validate_schema_contract(schema: dict[str, Any]) -> None:
    require(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "schema-draft")
    require(schema.get("additionalProperties") is False, "schema-top-additional-properties")
    require(set(schema.get("required", [])) == TOP_KEYS, "schema-required-drift")
    props = require_object(schema.get("properties"), "schema-properties")
    require(props["schema_version"].get("const") == 1, "schema-version-contract")
    require(props["report_kind"].get("const") == "typed_observation_evidence", "schema-report-kind")
    require(props["stability"].get("const") == "experimental", "schema-stability")
    require(props["backend"]["properties"]["profile"].get("const") == "linux-ptrace-metadata-v2", "schema-backend-profile")
    require(props["backend"]["properties"]["privacy_profile"].get("const") == "metadata-only-v1", "schema-privacy-profile")
    require(schema["$defs"]["fd_write_effect"]["properties"]["proposition"].get("const") == "file_fd_write_effect_observed", "schema-effect-proposition")


def expected_health(raw_complete: bool, warning_codes: list[str]) -> str:
    if raw_complete and not warning_codes:
        return "complete"
    if "event_limit_exceeded" in warning_codes:
        return "incomplete_limit"
    if "shared_fd_table_ambiguity" in warning_codes:
        return "incomplete_ambiguity"
    return "incomplete_capability"


def expected_write_effects(events: list[dict[str, Any]]) -> list[tuple[str, str]]:
    sequences = []
    for event in events:
        require(isinstance(event, dict), "raw-event-object")
        require(isinstance(event.get("sequence"), int) and not isinstance(event.get("sequence"), bool) and event["sequence"] > 0, "raw-event-sequence")
        require(isinstance(event.get("tid"), int) and not isinstance(event.get("tid"), bool), "raw-event-tid")
        require(isinstance(event.get("event_type"), str), "raw-event-type")
        sequences.append(event["sequence"])
    require(sequences == sorted(sequences), "raw-event-order")
    require(len(sequences) == len(set(sequences)), "raw-event-sequence-duplicate")

    result: list[tuple[str, str]] = []
    for effect in events:
        if not (
            effect.get("event_type") == "file_descriptor_access"
            and effect.get("operation") == "write"
        ):
            continue
        path = effect.get("path")
        require(isinstance(path, str) and path, "raw-write-path")
        prior = [
            event
            for event in events
            if event.get("tid") == effect["tid"]
            and event.get("event_type") == "process_exec"
            and isinstance(event.get("path"), str)
            and event["sequence"] < effect["sequence"]
        ]
        require(bool(prior), "raw-write-process-binding-missing")
        actor = max(prior, key=lambda item: item["sequence"])["path"]
        result.append((actor, path))
    return result


def validate_evidence(evidence: dict[str, Any]) -> dict[str, Any]:
    require_object(evidence, "evidence-object")
    require(set(evidence) == TOP_KEYS, "top-level-shape")
    require(evidence["schema_version"] == 1, "schema-version")
    require(evidence["report_kind"] == "typed_observation_evidence", "report-kind")
    require(evidence["stability"] == "experimental", "stability")

    present_forbidden = set(walk_keys(evidence)) & FORBIDDEN_KEYS
    require(not present_forbidden, "forbidden-claim-field:" + ",".join(sorted(present_forbidden)))

    backend = require_object(evidence["backend"], "backend-object")
    require(set(backend) == BACKEND_KEYS, "backend-shape")
    require(backend["profile"] == "linux-ptrace-metadata-v2", "backend-profile")
    require(isinstance(backend["implementation_version"], str) and backend["implementation_version"], "implementation-version")
    require(backend["platform"] == "linux", "backend-platform")
    require(backend["architecture"] == "x86_64", "backend-architecture")
    require(backend["privacy_profile"] == "metadata-only-v1", "privacy-profile")

    unsupported = require_string_list(evidence["unsupported_capabilities"], "unsupported-capabilities")
    require(unsupported == sorted(set(unsupported)), "unsupported-order-or-duplicate")
    require(set(unsupported) == EXPECTED_UNSUPPORTED, "unsupported-profile-drift")

    raw = require_object(evidence["raw_observation"], "raw-object")
    require(set(raw) == RAW_TOP_KEYS, "raw-top-shape")
    require(raw["schema_version"] == 2, "raw-schema-version")
    require(isinstance(raw["complete"], bool), "raw-complete-type")

    raw_backend = require_object(raw["backend"], "raw-backend-object")
    require(set(raw_backend) == RAW_BACKEND_KEYS, "raw-backend-shape")
    require(raw_backend["name"] == backend["profile"], "raw-backend-profile-binding")
    require(raw_backend["platform"] == backend["platform"], "raw-backend-platform-binding")
    require(raw_backend["architecture"] == backend["architecture"], "raw-backend-architecture-binding")
    raw_caps = require_string_list(raw_backend["capabilities"], "raw-capabilities")
    require(len(raw_caps) == len(set(raw_caps)), "raw-capability-duplicate")
    raw_limits = require_string_list(raw_backend["limitations"], "raw-limitations")

    outcome = require_object(raw["outcome"], "raw-outcome-object")
    require(set(outcome) == RAW_OUTCOME_KEYS, "raw-outcome-shape")
    for key in ("exit_code", "signal"):
        value = outcome[key]
        require(value is None or (isinstance(value, int) and not isinstance(value, bool)), "raw-outcome-type")

    warnings = raw["warnings"]
    require(isinstance(warnings, list), "raw-warnings")
    raw_warning_codes = []
    for warning in warnings:
        warning = require_object(warning, "raw-warning-object")
        require(set(warning) == RAW_WARNING_KEYS, "raw-warning-shape")
        require(isinstance(warning["code"], str) and warning["code"], "raw-warning-code")
        require(warning["tid"] is None or (isinstance(warning["tid"], int) and not isinstance(warning["tid"], bool)), "raw-warning-tid")
        require(isinstance(warning["message"], str), "raw-warning-message")
        raw_warning_codes.append(warning["code"])
    raw_warning_codes = sorted(set(raw_warning_codes))

    health = require_object(evidence["collection_health"], "health-object")
    require(set(health) == HEALTH_KEYS, "health-shape")
    require(health["state"] in KNOWN_HEALTH, "unknown-health-state")
    typed_warning_codes = require_string_list(health["warning_codes"], "typed-warning-codes")
    require(typed_warning_codes == sorted(set(typed_warning_codes)), "typed-warning-order-or-duplicate")
    require(typed_warning_codes == raw_warning_codes, "warning-code-binding")
    require(health["state"] == expected_health(raw["complete"], raw_warning_codes), "health-raw-binding")

    limitations = require_string_list(evidence["limitations"], "limitations")
    require(limitations == sorted(set(limitations)), "limitation-order-or-duplicate")
    require(limitations == sorted(set(raw_limits)), "limitation-raw-binding")

    nonclaims = require_string_list(evidence["does_not_assert"], "does-not-assert")
    require(len(nonclaims) == len(set(nonclaims)), "does-not-assert-duplicate")
    require(set(nonclaims) == EXPECTED_NONCLAIMS, "does-not-assert-boundary")

    events = raw["events"]
    require(isinstance(events, list), "raw-events")
    expected_effects = expected_write_effects(events)

    typed_effects = evidence["effects"]
    require(isinstance(typed_effects, list), "typed-effects")
    actual_effects: list[tuple[str, str]] = []
    for effect in typed_effects:
        effect = require_object(effect, "typed-effect-object")
        require(set(effect) == EFFECT_KEYS, "typed-effect-shape")
        require(effect["proposition"] == "file_fd_write_effect_observed", "typed-effect-proposition")
        require(isinstance(effect["actor"], str) and effect["actor"], "typed-effect-actor")
        require(isinstance(effect["target"], str) and effect["target"], "typed-effect-target")
        guarantees = require_object(effect["guarantees"], "typed-guarantees-object")
        require(set(guarantees) == GUARANTEE_KEYS, "typed-guarantees-shape")
        require(
            guarantees["observation_points"]
            == ["derived_runtime_fd_state", "syscall_result_post_operation"],
            "typed-observation-points",
        )
        require(
            guarantees["identity_bases"] == ["runtime_fd_path_correlated"],
            "typed-identity-basis",
        )
        require(
            guarantees["temporal_bindings"] == ["successful_operation_result"],
            "typed-temporal-binding",
        )
        require(
            guarantees["causal_bindings"] == ["state_machine_correlated"],
            "typed-causal-binding",
        )
        actual_effects.append((effect["actor"], effect["target"]))

    require(actual_effects == expected_effects, "typed-raw-effect-binding")

    return {
        "decision": "EXPERIMENTAL_TYPED_EVIDENCE_V1_ACCEPTED_BOUNDED",
        "collection_health": health["state"],
        "effect_count": len(actual_effects),
        "raw_event_count": len(events),
        "implementation_version": backend["implementation_version"],
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("--schema", type=Path)
    args = parser.parse_args()

    try:
        evidence = json.loads(args.evidence.read_text(encoding="utf-8"))
        if args.schema is not None:
            schema = json.loads(args.schema.read_text(encoding="utf-8"))
            validate_schema_contract(schema)
        summary = validate_evidence(evidence)
    except (OSError, json.JSONDecodeError, ContractError, KeyError, TypeError) as error:
        print(f"REJECT: {error}", file=sys.stderr)
        raise SystemExit(2) from error

    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    import sys
    main()
