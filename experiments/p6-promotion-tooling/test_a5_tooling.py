import json
import unittest

from evaluator import (
    FROZEN_SOURCE_PINS,
    REQUIRED_NEGATIVE_EVIDENCE,
    PolicyError,
    build_factual_matrix,
    normalize_record,
    proposition_relation,
    require_complete_matrix_scope,
    validate_claims,
)


def row(
    product="Harden-Runner",
    proposition="CP1_PROCESS_EXECUTION_IDENTITY",
    classification="OBSERVED_SUPPORTED",
    source_sha=None,
    dimension="actor_identity",
    semantic_contract="observation-v1",
    **extra,
):
    value = {
        "product": product,
        "proposition": proposition,
        "scenario": "S1",
        "dimension": dimension,
        "semantic_contract": semantic_contract,
        "classification": classification,
        "source_sha": source_sha or FROZEN_SOURCE_PINS.get(product, "internal-source"),
        "note": "bounded factual observation",
    }
    value.update(extra)
    return value


class A5PromotionToolingAttacks(unittest.TestCase):
    def test_a5_01_product_name_cannot_confer_semantic_authority(self):
        with self.assertRaises(PolicyError):
            normalize_record(row(semantic_authority=True))

    def test_a5_02_comparison_record_cannot_be_proof_admissible(self):
        with self.assertRaises(PolicyError):
            normalize_record(row(proof_admissible=True))
        matrix = build_factual_matrix(
            [row()], negative_evidence_ids=REQUIRED_NEGATIVE_EVIDENCE
        )
        self.assertFalse(matrix["proof_admission_from_comparison"])
        self.assertFalse(matrix["semantic_authority_from_comparison"])

    def test_a5_03_incomplete_and_not_testable_remain_explicit(self):
        matrix = build_factual_matrix(
            [
                row(classification="INCOMPLETE"),
                row(
                    product="Tetragon",
                    proposition="CP2_OUTBOUND_NETWORK_DESTINATION",
                    classification="NOT_TESTABLE",
                ),
            ],
            negative_evidence_ids=REQUIRED_NEGATIVE_EVIDENCE,
        )
        classes = [record["classification"] for record in matrix["records"]]
        self.assertEqual(classes, ["INCOMPLETE", "NOT_TESTABLE"])
        self.assertNotIn("PASS", classes)

    def test_a5_04_ranking_score_and_winner_inputs_are_rejected(self):
        for forbidden in ("score", "rank", "tier", "winner", "global_winner"):
            with self.subTest(forbidden=forbidden):
                with self.assertRaises(PolicyError):
                    normalize_record(row(**{forbidden: 1}))

    def test_a5_05_partial_matrix_cannot_support_global_scope(self):
        record = normalize_record(row())
        with self.assertRaises(PolicyError):
            require_complete_matrix_scope(
                [record],
                {
                    ("Harden-Runner", "CP1_PROCESS_EXECUTION_IDENTITY"),
                    ("cicd-sensor", "CP1_PROCESS_EXECUTION_IDENTITY"),
                },
            )

    def test_a5_06_mismatched_comparison_axes_are_incomparable(self):
        left = normalize_record(row())
        different_prop = normalize_record(
            row(proposition="CP2_OUTBOUND_NETWORK_DESTINATION")
        )
        different_dimension = normalize_record(row(dimension="completeness"))
        different_contract = normalize_record(row(semantic_contract="attempt-vs-success-v2"))
        self.assertEqual(
            proposition_relation(left, different_prop), "INCOMPARABLE_PROPOSITION"
        )
        self.assertEqual(
            proposition_relation(left, different_dimension), "INCOMPARABLE_DIMENSION"
        )
        self.assertEqual(
            proposition_relation(left, different_contract),
            "INCOMPARABLE_SEMANTIC_CONTRACT",
        )

    def test_a5_07_source_pin_mismatch_is_explicit_stale_source(self):
        stale = normalize_record(row(source_sha="0" * 40))
        current = normalize_record(row())
        self.assertEqual(stale.source_status, "STALE_SOURCE")
        self.assertEqual(
            proposition_relation(stale, current), "INCOMPARABLE_SOURCE_IDENTITY"
        )

    def test_a5_08_required_negative_evidence_cannot_be_suppressed(self):
        incomplete_ledger = set(REQUIRED_NEGATIVE_EVIDENCE)
        incomplete_ledger.remove("EXECSURFACE-S2-ERROR-INCOMPLETE")
        with self.assertRaises(PolicyError):
            build_factual_matrix([row()], negative_evidence_ids=incomplete_ledger)

    def test_a5_09_input_and_product_order_are_deterministic(self):
        rows = [
            row(),
            row(
                product="cicd-sensor",
                proposition="CP3_FILE_MUTATION",
                classification="OBSERVED_PARTIAL",
            ),
            row(
                product="Tetragon",
                proposition="CP2_OUTBOUND_NETWORK_DESTINATION",
                classification="NOT_TESTABLE",
            ),
        ]
        first = build_factual_matrix(
            rows, negative_evidence_ids=REQUIRED_NEGATIVE_EVIDENCE
        )
        second = build_factual_matrix(
            list(reversed(rows)), negative_evidence_ids=reversed(sorted(REQUIRED_NEGATIVE_EVIDENCE))
        )
        self.assertEqual(
            json.dumps(first, sort_keys=True, separators=(",", ":")),
            json.dumps(second, sort_keys=True, separators=(",", ":")),
        )

    def test_a5_10_internal_evidence_cannot_claim_p8_or_external_validation(self):
        for claims in (
            {"external_validation": True},
            {"p8_closed": True},
            {"adoption": True},
            {"endorsement": True},
            {"independent_reproduction": True},
            {"production_ready": True},
        ):
            with self.subTest(claims=claims):
                with self.assertRaises(PolicyError):
                    validate_claims(claims)

    def test_a5_11_overlap_cannot_become_backend_equivalence(self):
        left = normalize_record(row())
        right = normalize_record(
            row(
                product="cicd-sensor",
                source_sha=FROZEN_SOURCE_PINS["cicd-sensor"],
            )
        )
        self.assertEqual(
            proposition_relation(left, right), "FACTS_SHARE_DECLARED_COMPARISON_AXIS"
        )
        with self.assertRaises(PolicyError):
            validate_claims({"backend_equivalent": True})

    def test_a5_12_comparison_success_cannot_authorize_release_or_product_correctness(self):
        matrix = build_factual_matrix(
            [row()], negative_evidence_ids=REQUIRED_NEGATIVE_EVIDENCE
        )
        self.assertIsNone(matrix["global_product_verdict"])
        self.assertIsNone(matrix["performance_ranking"])
        self.assertFalse(matrix["release_authorized"])
        self.assertFalse(matrix["main_merge_authorized"])
        self.assertFalse(matrix["tag_movement_authorized"])
        for claims in (
            {"release_ready": True},
            {"main_merge_authorized": True},
            {"tag_movement_authorized": True},
            {"default_v3_authorized": True},
        ):
            with self.subTest(claims=claims):
                with self.assertRaises(PolicyError):
                    validate_claims(claims)


if __name__ == "__main__":
    unittest.main(verbosity=2)
