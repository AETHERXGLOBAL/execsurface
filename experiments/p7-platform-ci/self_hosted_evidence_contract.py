"""Research-only self-hosted CI package/evidence contract for P7-A2."""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass
from typing import Mapping, Sequence

ALPHA4_SOURCE = "48e0b9a0707553349e75e97a9dfa096d13f9ab5d"
ALPHA4_VERSION = "0.1.0-alpha.4"
SCHEMA = "execsurface.self-hosted-ci-package.v1"
EVIDENCE_SCHEMA = "execsurface.self-hosted-ci-evidence.v1"
CONTEXT_AUTHORITY = "context_only"

_HEX64 = re.compile(r"^[0-9a-fA-F]{64}$")
_TOKEN = re.compile(r"^[A-Za-z0-9_.:/-]{1,128}$")
_RUNNER_ID = re.compile(r"^[A-Za-z0-9_.:-]{1,128}$")
_ALLOWED_VERDICTS = {"PASS", "REVIEW", "BLOCK", "ERROR"}
_ALLOWED_AUTHORITY = {"none", "attempt_only", "derived", "direct"}
_ALLOWED_COMPLETENESS = {"complete", "incomplete", "lost", "ambiguous", "unsupported"}


class ContractError(ValueError):
    pass


@dataclass(frozen=True)
class PackageContract:
    record: dict[str, object]
    canonical_json: str
    package_digest: str

    def output(self) -> dict[str, object]:
        out = dict(self.record)
        out["package_digest"] = self.package_digest
        return out


@dataclass(frozen=True)
class EvidenceEnvelope:
    record: dict[str, object]
    canonical_json: str
    envelope_digest: str

    def output(self) -> dict[str, object]:
        out = dict(self.record)
        out["envelope_digest"] = self.envelope_digest
        return out


def _text(value: object, field: str) -> str:
    if not isinstance(value, str) or not value:
        raise ContractError(f"missing_or_invalid:{field}")
    if value != value.strip() or any(ord(ch) < 0x20 or ord(ch) == 0x7F for ch in value):
        raise ContractError(f"invalid_controls:{field}")
    return value


def _runner_token(value: object, field: str) -> str:
    value = _text(value, field)
    if not _RUNNER_ID.fullmatch(value):
        raise ContractError(f"invalid_token:{field}")
    return value


def _platform_token(value: object, field: str) -> str:
    value = _text(value, field).lower()
    if not _TOKEN.fullmatch(value):
        raise ContractError(f"invalid_token:{field}")
    return value


def _labels(values: object) -> list[str]:
    if not isinstance(values, Sequence) or isinstance(values, (str, bytes)):
        raise ContractError("invalid_labels")
    canonical: set[str] = set()
    for item in values:
        label = _platform_token(item, "label")
        canonical.add(label)
    return sorted(canonical)


def _digest(record: dict[str, object]) -> tuple[str, str]:
    canonical = json.dumps(record, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return canonical, hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def _baseline_reference(value: str | None) -> str | None:
    if value is None:
        return None
    value = _text(value, "baseline_reference").lower()
    if not _HEX64.fullmatch(value):
        raise ContractError("invalid_baseline_reference")
    return value


def build_package_contract(
    env: Mapping[str, object],
    *,
    baseline_reference: str | None = None,
) -> PackageContract:
    provider = _runner_token(env.get("provider"), "provider")
    runner_id = _runner_token(env.get("runner_id"), "runner_id")
    os_name = _platform_token(env.get("os"), "os")
    arch = _platform_token(env.get("arch"), "arch")
    labels = _labels(env.get("labels"))

    eligible = os_name == "linux" and arch in {"x86_64", "amd64"}
    ineligible_reason = None if eligible else "public_alpha4_platform_not_supported"

    record: dict[str, object] = {
        "schema": SCHEMA,
        "execsurface_source": ALPHA4_SOURCE,
        "execsurface_version": ALPHA4_VERSION,
        "runner": {
            "provider": provider,
            "runner_id": runner_id,
            "os": os_name,
            "arch": arch,
            "labels": labels,
            "authority": CONTEXT_AUTHORITY,
        },
        "package_eligible": eligible,
        "ineligible_reason": ineligible_reason,
        "baseline_reference": _baseline_reference(baseline_reference),
        "may_infer_baseline": False,
        "may_change_verdict": False,
        "may_upgrade_observer_authority": False,
        "may_upgrade_completeness": False,
    }
    canonical, digest = _digest(record)
    return PackageContract(record, canonical, digest)


def bind_verification_evidence(
    package: PackageContract,
    *,
    verdict: str,
    evidence_digest: str,
    observer_authority: str,
    observer_completeness: str,
) -> EvidenceEnvelope:
    if verdict not in _ALLOWED_VERDICTS:
        raise ContractError("invalid_verdict")
    evidence_digest = _text(evidence_digest, "evidence_digest").lower()
    if not _HEX64.fullmatch(evidence_digest):
        raise ContractError("invalid_evidence_digest")
    if observer_authority not in _ALLOWED_AUTHORITY:
        raise ContractError("invalid_observer_authority")
    if observer_completeness not in _ALLOWED_COMPLETENESS:
        raise ContractError("invalid_observer_completeness")
    if observer_completeness != "complete" and verdict == "PASS":
        raise ContractError("noncomplete_pass_forbidden")

    record: dict[str, object] = {
        "schema": EVIDENCE_SCHEMA,
        "package_digest": package.package_digest,
        "package_eligible": package.record["package_eligible"],
        "verdict": verdict,
        "evidence_digest": evidence_digest,
        "observer_authority": observer_authority,
        "observer_completeness": observer_completeness,
        "runner_authority": CONTEXT_AUTHORITY,
        "authority_upgraded_by_runner": False,
        "completeness_upgraded_by_runner": False,
    }
    canonical, digest = _digest(record)
    return EvidenceEnvelope(record, canonical, digest)
