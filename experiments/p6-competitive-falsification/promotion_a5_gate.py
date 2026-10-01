#!/usr/bin/env python3
"""Frozen A5 falsification gate for P6 comparison/tooling promotion.

This is promotion-governance tooling, not ExecSurface runtime semantics.
The exact twelve scientific tests are preregistered in
POST_ALPHA4_PROMOTION_A5_P6_COMPARISON_TOOLING_PROTOCOL.md.
"""

from __future__ import annotations

import hashlib
import json
import unittest
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
P6 = ROOT / "experiments" / "p6-competitive-falsification"
DOCS = ROOT / "docs" / "development"

ALPHA4 = "48e0b9a0707553349e75e97a9dfa096d13f9ab5d"
FROZEN_BLOBS = {
    "p6-a0-manifest.json": "625f5af914f33c9408e28933c031ef183b56f8ae",
    "p6-a1-observation-matrix.json": "6179c4f319be7c7a83d36d20e035abb13f3d792c",
    "p6-a2-adversarial-matrix.json": "49e67fe84e61b257ee71230206215f1aa7dd87ab",
    "workload.sh": "c5986f1acd8bc79a3945e613acc128d5c8a9d168",
}
FROZEN_CLOSEOUT_BLOB = "f87a8309cb7131643bcafe57ef3753d89eada693"


def load_json(name: str) -> dict[str, Any]:
    return json.loads((P6 / name).read_text(encoding="utf-8"))


def git_blob_sha(path: Path) -> str:
    payload = path.read_bytes()
    header = f"blob {len(payload)}\0".encode()
    return hashlib.sha1(header + payload).hexdigest()  # noqa: S324 - Git object identity is SHA-1 by design.


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


@dataclass(frozen=True)
class ComparisonFact:
    origin: str
    comparator: str
    native_verdict: str | None = None
    factual_scope: str = "proposition_only"


def comparison_can_raise_semantic_authority(_: ComparisonFact) -> bool:
    """P6 comparison facts are never semantic-authority sources."""
    return False


def comparison_can_replace_product_correctness(_: ComparisonFact) -> bool:
    """Comparator overlap/success never substitutes for product correctness."""
    return False


def qualifies_for_p8_external_validation(origin: str, evidence_class: str) -> bool:
    allowed = {
        "ZERO_ASSISTANCE_REPRODUCTION",
        "REPRODUCTION_FAILURE",
        "EXTERNAL_REAL_WORKLOAD_REPORT",
        "ARCHITECTURE_CRITICISM",
        "COUNTEREXAMPLE",
        "NO_FIT_OR_REDUNDANCY",
        "INTEROPERABILITY_GUIDANCE",
    }
    return origin == "external_independent" and evidence_class in allowed


def comparison_supports_claim(claim: str) -> bool:
    """Only bounded factual-overlap statements are derivable from P6 itself."""
    return claim == "factual_overlap"


class A5P6ToolingFalsification(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.a0 = load_json("p6-a0-manifest.json")
        cls.a1 = load_json("p6-a1-observation-matrix.json")
        cls.a2 = load_json("p6-a2-adversarial-matrix.json")
        cls.closeout = (DOCS / "P6_CLOSEOUT_DECISION.md").read_text(encoding="utf-8")

    def test_a5_01_aggregate_score_and_overall_winner_remain_forbidden(self) -> None:
        for record in (self.a0, self.a1, self.a2):
            self.assertIs(record.get("aggregate_score_forbidden"), True)
        self.assertIsNone(self.a1.get("overall_winner"))
        self.assertIsNone(self.a2.get("overall_winner"))
        self.assertIn("does not rank products", self.closeout)
        self.assertIn("does not establish:\n- an overall winner", self.closeout)

    def test_a5_02_foreign_native_verdict_cannot_become_execsurface_pass(self) -> None:
        sensor = self.a1["systems"]["cicd_sensor"]
        self.assertEqual(sensor["native_predicate_result"], "passed")
        self.assertIs(sensor["native_result_not_mapped_to_execsurface_pass"], True)
        self.assertIsNone(self.a2["cicd_sensor"]["C3_common_verdict"])

    def test_a5_03_not_testable_and_incomplete_are_not_collapsed(self) -> None:
        tetragon = self.a1["systems"]["tetragon"]
        self.assertEqual(tetragon["CP1"], "NOT_TESTABLE_YET")
        self.assertEqual(tetragon["CP2"], "NOT_TESTABLE_YET")
        self.assertEqual(tetragon["CP3"], "NOT_TESTABLE_YET")
        network = self.a1["systems"]["execsurface_alpha4"]["scenarios"]["S2_NETWORK_DESTINATION_EXPANSION"]
        self.assertEqual(network["native_result"], "ERROR_INCOMPLETE")
        self.assertEqual(network["CP2"], "INCOMPLETE")
        self.assertNotEqual(network["native_result"].lower(), "pass")

    def test_a5_04_comparison_fact_cannot_raise_semantic_authority(self) -> None:
        for comparator in ("Harden-Runner", "cicd-sensor", "Tetragon", "Falco", "Tracee", "ExecSurface"):
            fact = ComparisonFact(origin="p6_internal", comparator=comparator, native_verdict="success")
            self.assertFalse(comparison_can_raise_semantic_authority(fact))

    def test_a5_05_comparison_cannot_substitute_for_product_correctness(self) -> None:
        positive = ComparisonFact(origin="p6_internal", comparator="cicd-sensor", native_verdict="passed")
        negative = ComparisonFact(origin="p6_internal", comparator="ExecSurface", native_verdict="ERROR_INCOMPLETE")
        self.assertFalse(comparison_can_replace_product_correctness(positive))
        self.assertFalse(comparison_can_replace_product_correctness(negative))
        self.assertEqual(self.a2["execsurface"]["false_pass_count"], 0)

    def test_a5_06_internal_p6_cannot_satisfy_p8_external_validation(self) -> None:
        for evidence_class in (
            "ZERO_ASSISTANCE_REPRODUCTION",
            "EXTERNAL_REAL_WORKLOAD_REPORT",
            "ARCHITECTURE_CRITICISM",
            "COUNTEREXAMPLE",
            "INTEROPERABILITY_GUIDANCE",
        ):
            self.assertFalse(qualifies_for_p8_external_validation("p6_internal", evidence_class))
        self.assertTrue(qualifies_for_p8_external_validation("external_independent", "COUNTEREXAMPLE"))

    def test_a5_07_praise_overlap_or_listing_cannot_manufacture_adoption_or_endorsement(self) -> None:
        self.assertTrue(comparison_supports_claim("factual_overlap"))
        for claim in ("adoption", "endorsement", "production_readiness", "superiority", "external_validation"):
            self.assertFalse(comparison_supports_claim(claim))

    def test_a5_08_negative_evidence_locators_remain_retained(self) -> None:
        negative = self.a1["historical_negative"]
        self.assertEqual(negative["run_id"], 36770551463)
        self.assertIn("ERROR/INCOMPLETE", negative["product_evidence_retained"])
        for marker in (
            "36770551463",
            "ExecSurface alpha.4 S2 incompleteness",
            "Tetragon live `NOT_TESTABLE_YET` status",
            "cicd-sensor unsigned copied-predicate mutability observation",
            "decision not to create a misleading performance comparison",
        ):
            self.assertIn(marker, self.closeout)

    def test_a5_09_invalid_performance_ranking_remains_blocked(self) -> None:
        self.assertIn("Decision: `NO_VALID_PERFORMANCE_COMPARISON`", self.closeout)
        self.assertIn("No product speed ranking is authorized.", self.closeout)
        self.assertNotIn("overall performance winner", self.closeout.lower())

    def test_a5_10_frozen_comparator_and_workload_identities_match(self) -> None:
        for filename, expected in FROZEN_BLOBS.items():
            self.assertEqual(git_blob_sha(P6 / filename), expected, filename)
        self.assertEqual(git_blob_sha(DOCS / "P6_CLOSEOUT_DECISION.md"), FROZEN_CLOSEOUT_BLOB)
        self.assertEqual(self.a0["execsurface"]["alpha4_source"], ALPHA4)
        expected_pins = {
            "harden_runner": "e14015d583714f6e62063499dc959a02595150a1",
            "cicd_sensor": "1f031a106e23edda1eb496b0bae51fb12e85d62d",
            "tetragon": "666efe6f91e3605ad58683ad226d759d9cf970ca",
            "falco": "e12b1d43e47a2903c07e14479e034d74d523ab9d",
            "tracee": "2f9dc40c20b17c2ba27f6d92b25e62790bd48a62",
        }
        for name, pin in expected_pins.items():
            self.assertEqual(self.a0["comparators"][name]["source_commit"], pin)

    def test_a5_11_factual_normalization_is_deterministic_under_record_reordering(self) -> None:
        original = self.a1
        reordered = {key: original[key] for key in reversed(list(original.keys()))}
        systems = original["systems"]
        reordered["systems"] = {key: systems[key] for key in reversed(list(systems.keys()))}
        self.assertEqual(canonical(original), canonical(reordered))

    def test_a5_12_competitor_name_or_success_cannot_confer_authority(self) -> None:
        for name in self.a0["comparators"]:
            fact = ComparisonFact(origin="p6_internal", comparator=name, native_verdict="success")
            self.assertFalse(comparison_can_raise_semantic_authority(fact))
        self.assertIn("use backend/product name as semantic authority", (DOCS / "P6_COMPETITIVE_FALSIFICATION_PROTOCOL.md").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main(verbosity=2)
