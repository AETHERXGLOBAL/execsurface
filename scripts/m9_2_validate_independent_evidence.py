#!/usr/bin/env python3
"""Validate whether one M9.2 record is admissible as independent evidence.

This validator answers only the provenance/intake question. It deliberately
accepts independently produced failures because negative external evidence is
first-class. It does NOT decide whether a record is countable toward the
positive M9.2 adoption close gate.

Use `scripts/m9_validate_independent_adoption.py --countable` for one
successful countable adoption record and `--close-gate` for the aggregate
positive-close requirements.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

from m9_validate_evidence import EvidenceError, require, validate_record


def validate_independent_record(record: dict) -> None:
    # First inherit every canonical M9 semantic/privacy/authority check.
    validate_record(record)

    require(
        record["independence_class"] == "INDEPENDENT_USER",
        "M9.2 admissible independent evidence must be INDEPENDENT_USER",
    )

    provenance = record["provenance"]
    project = record["project"]
    workflow = record["workflow"]
    outcome = record["outcome"]

    # Re-state the independence boundary here deliberately so a future change to
    # the generic validator cannot silently weaken M9.2.
    require(provenance["initiated_by"] == "THIRD_PARTY", "M9.2 requires third-party initiation")
    require(provenance["executed_by"] == "THIRD_PARTY", "M9.2 requires third-party execution")
    require(
        provenance["aether_x_material_involvement"] is False,
        "M9.2 independent evidence cannot have material AETHER X execution involvement",
    )
    require(project["modified_by_aether_x"] is False, "M9.2 workload cannot be modified by AETHER X")

    attestation = provenance["third_party_attestation"]
    require(isinstance(attestation, str) and attestation.strip(), "M9.2 requires third-party attestation")

    external_reference = record.get("external_reference")
    require(
        isinstance(external_reference, str) and external_reference.strip(),
        "M9.2 requires a public or consented external reference",
    )

    # A successful command alone is not adoption evidence. Require an actual
    # install/baseline/rerun attempt. PASS is intentionally not required here,
    # because independently produced failures must remain admissible evidence.
    require(outcome["installation"] != "NOT_RUN", "M9.2 requires an installation attempt")
    require(workflow["baseline_attempted"] is True, "M9.2 requires baseline creation to be attempted")
    require(outcome["baseline"] != "NOT_RUN", "M9.2 baseline outcome cannot be NOT_RUN")
    require(workflow["rerun_check_attempted"] is True, "M9.2 requires unchanged rerun/check to be attempted")
    require(outcome["rerun_check"] != "NOT_RUN", "M9.2 rerun/check outcome cannot be NOT_RUN")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("record", type=Path)
    args = parser.parse_args()

    try:
        raw = args.record.read_bytes()
        record = json.loads(raw)
        require(isinstance(record, dict), "top-level JSON must be an object")
        validate_independent_record(record)
    except (OSError, json.JSONDecodeError, EvidenceError, KeyError, TypeError, ValueError) as exc:
        print(f"M9_2_INDEPENDENT_EVIDENCE_INVALID: {exc}", file=sys.stderr)
        return 1

    print(
        "M9_2_INDEPENDENT_EVIDENCE_ADMISSIBLE "
        f"id={record['evidence_id']} sha256={hashlib.sha256(raw).hexdigest()}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
