"""Bounded proposition-local evaluator for ExecSurface P6 promotion testing.

This module is internal validation tooling. It deliberately exposes no product score,
ranking, winner, superiority, release, or semantic-authority API.
"""

from __future__ import annotations

from dataclasses import dataclass, asdict
from typing import Iterable, Mapping, Sequence


ALLOWED_CLASSIFICATIONS = frozenset(
    {
        "OBSERVED_SUPPORTED",
        "OBSERVED_PARTIAL",
        "NOT_OBSERVED",
        "NOT_APPLICABLE",
        "NOT_TESTABLE",
        "AMBIGUOUS",
        "INCOMPLETE",
        "LOST",
        "FIXTURE_OR_INFRA_FAILURE",
    }
)

FROZEN_SOURCE_PINS = {
    "Harden-Runner": "e14015d583714f6e62063499dc959a02595150a1",
    "cicd-sensor": "1f031a106e23edda1eb496b0bae51fb12e85d62d",
    "Tetragon": "666efe6f91e3605ad58683ad226d759d9cf970ca",
    "Falco": "e12b1d43e47a2903c07e14479e034d74d523ab9d",
    "Tracee": "2f9dc40c20b17c2ba27f6d92b25e62790bd48a62",
}

REQUIRED_NEGATIVE_EVIDENCE = frozenset(
    {
        "P6-A1-HARNESS-STOP-36770551463",
        "EXECSURFACE-S2-ERROR-INCOMPLETE",
        "TETRAGON-NOT-TESTABLE-YET",
        "CICD-SENSOR-UNSIGNED-COPY-MUTABILITY",
        "P6-A3-NATURAL-RUN-DNS-DIGEST-VARIATION",
        "P6-A4-NO-VALID-PERFORMANCE-COMPARISON",
    }
)

FORBIDDEN_INPUT_KEYS = frozenset(
    {
        "score",
        "rank",
        "tier",
        "winner",
        "global_winner",
        "better",
        "worse",
        "stronger",
        "weaker",
        "superiority",
        "product_correct",
        "release_ready",
        "semantic_authority",
        "proof_admissible",
        "external_validation",
        "adoption",
        "endorsement",
        "independent_reproduction",
        "production_ready",
    }
)

FORBIDDEN_CLAIM_KEYS = frozenset(
    {
        "winner",
        "ranking",
        "score",
        "superiority",
        "semantic_authority",
        "proof_admissible",
        "release_ready",
        "main_merge_authorized",
        "tag_movement_authorized",
        "default_v3_authorized",
        "external_validation",
        "p8_closed",
        "adoption",
        "endorsement",
        "independent_reproduction",
        "production_ready",
        "backend_equivalent",
    }
)


class PolicyError(ValueError):
    """Raised when comparison input attempts to cross the frozen A5 boundary."""


@dataclass(frozen=True)
class FactualRecord:
    product: str
    proposition: str
    scenario: str
    dimension: str
    semantic_contract: str
    classification: str
    source_sha: str
    source_status: str
    note: str = ""


def _reject_forbidden_keys(record: Mapping[str, object]) -> None:
    forbidden = sorted(FORBIDDEN_INPUT_KEYS.intersection(record))
    if forbidden:
        raise PolicyError(f"forbidden comparison fields: {', '.join(forbidden)}")


def normalize_record(record: Mapping[str, object]) -> FactualRecord:
    """Validate one proposition-local factual record without deriving a verdict."""

    _reject_forbidden_keys(record)
    required = {
        "product",
        "proposition",
        "scenario",
        "dimension",
        "semantic_contract",
        "classification",
        "source_sha",
    }
    missing = sorted(required.difference(record))
    if missing:
        raise PolicyError(f"missing required fields: {', '.join(missing)}")

    classification = str(record["classification"])
    if classification not in ALLOWED_CLASSIFICATIONS:
        raise PolicyError(f"unsupported classification: {classification}")

    product = str(record["product"])
    source_sha = str(record["source_sha"])
    expected = FROZEN_SOURCE_PINS.get(product)
    source_status = "UNPINNED_COMPARATOR"
    if expected is not None:
        source_status = "CURRENT_FROZEN_SOURCE" if source_sha == expected else "STALE_SOURCE"

    return FactualRecord(
        product=product,
        proposition=str(record["proposition"]),
        scenario=str(record["scenario"]),
        dimension=str(record["dimension"]),
        semantic_contract=str(record["semantic_contract"]),
        classification=classification,
        source_sha=source_sha,
        source_status=source_status,
        note=str(record.get("note", "")),
    )


def build_factual_matrix(
    rows: Iterable[Mapping[str, object]],
    *,
    negative_evidence_ids: Iterable[str],
) -> dict[str, object]:
    """Build deterministic factual output while retaining mandatory negative evidence."""

    retained = frozenset(negative_evidence_ids)
    missing_negative = sorted(REQUIRED_NEGATIVE_EVIDENCE.difference(retained))
    if missing_negative:
        raise PolicyError(
            "required negative evidence missing: " + ", ".join(missing_negative)
        )

    normalized = [normalize_record(row) for row in rows]
    normalized.sort(
        key=lambda row: (
            row.product,
            row.proposition,
            row.scenario,
            row.dimension,
            row.semantic_contract,
            row.classification,
            row.source_sha,
            row.note,
        )
    )

    return {
        "records": [asdict(row) for row in normalized],
        "negative_evidence_ids": sorted(retained),
        "semantic_authority_from_comparison": False,
        "proof_admission_from_comparison": False,
        "global_product_verdict": None,
        "performance_ranking": None,
        "external_validation": False,
        "p8_closed": False,
        "release_authorized": False,
        "main_merge_authorized": False,
        "tag_movement_authorized": False,
    }


def proposition_relation(left: FactualRecord, right: FactualRecord) -> str:
    """Return only comparability, never an ordering or equivalence verdict."""

    if left.source_status != "CURRENT_FROZEN_SOURCE" or right.source_status != "CURRENT_FROZEN_SOURCE":
        return "INCOMPARABLE_SOURCE_IDENTITY"
    if left.proposition != right.proposition:
        return "INCOMPARABLE_PROPOSITION"
    if left.dimension != right.dimension:
        return "INCOMPARABLE_DIMENSION"
    if left.semantic_contract != right.semantic_contract:
        return "INCOMPARABLE_SEMANTIC_CONTRACT"
    return "FACTS_SHARE_DECLARED_COMPARISON_AXIS"


def validate_claims(claims: Mapping[str, object]) -> None:
    """Reject any downstream attempt to turn P6/A5 facts into forbidden conclusions."""

    forbidden = [
        key
        for key in sorted(FORBIDDEN_CLAIM_KEYS)
        if key in claims and claims[key] not in (False, None, "", 0)
    ]
    if forbidden:
        raise PolicyError(f"forbidden derived claims: {', '.join(forbidden)}")


def require_complete_matrix_scope(
    records: Sequence[FactualRecord], required_axes: Iterable[tuple[str, str]]
) -> None:
    """Fail closed when a requested comparison scope is only partially represented."""

    present = {(record.product, record.proposition) for record in records}
    missing = sorted(set(required_axes).difference(present))
    if missing:
        encoded = ", ".join(f"{product}/{proposition}" for product, proposition in missing)
        raise PolicyError(f"partial matrix cannot support requested scope: {encoded}")
