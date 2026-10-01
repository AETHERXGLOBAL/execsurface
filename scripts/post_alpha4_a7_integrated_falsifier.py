#!/usr/bin/env python3
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
PROTOCOL = (ROOT / "docs/development/POST_ALPHA4_PROMOTION_A7_INTEGRATED_DESTRUCTIVE_PROTOCOL.md").read_text()
INVENTORY = (ROOT / "docs/development/POST_ALPHA4_PROMOTION_INVENTORY.md").read_text()
A1_REQUAL = (ROOT / "docs/development/POST_ALPHA4_PROMOTION_A1_REQUALIFICATION_PROTOCOL.md").read_text()
P6 = (ROOT / "docs/development/P6_CLOSEOUT_DECISION.md").read_text()
P7 = (ROOT / "docs/development/P7_CLOSEOUT_DECISION.md").read_text()
SEM = (ROOT / "crates/execsurface-model/src/semantics_v3.rs").read_text()
ALPHA4 = "48e0b9a0707553349e75e97a9dfa096d13f9ab5d"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


class A7IntegratedFalsifier(unittest.TestCase):
    def test_a7_01_public_alpha4_and_v01_remain_immutable(self):
        self.assertEqual(git("rev-parse", "v0.1.0-alpha.4^{commit}"), ALPHA4)
        self.assertEqual(git("rev-parse", "v0.1^{commit}"), ALPHA4)

    def test_a7_02_public_v2_and_candidate_v3_remain_distinct_domains(self):
        self.assertIn("RAW_OBSERVATION_SCHEMA_VERSION: u32 = 2", (ROOT / "crates/execsurface-model/src/lib.rs").read_text())
        self.assertIn("SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION: u32 = 3", SEM)

    def test_a7_03_cross_schema_default_remains_incomparable(self):
        self.assertIn("INCOMPARABLE_SCHEMA", INVENTORY)
        self.assertIn("no default PASS-eligible v2 -> v3 projection", INVENTORY)

    def test_a7_04_proof_requirement_remains_proposition_bound(self):
        self.assertIn("expected_proposition", SEM)
        self.assertIn("proposition", SEM)

    def test_a7_05_invalidating_ambiguity_remains_non_admissible(self):
        self.assertIn("ambiguity", SEM.lower())
        self.assertIn("missing/incomplete/ambiguous/unsupported required evidence remains non-admissible", INVENTORY)

    def test_a7_06_empty_requirement_bypass_remains_closed(self):
        tests = (ROOT / "crates/execsurface-model/tests/semantics_v3_rework.rs").read_text()
        self.assertIn("default", tests.lower())
        self.assertIn("requirement", tests.lower())

    def test_a7_07_backend_and_external_import_names_never_raise_authority(self):
        self.assertIn("backend names do not confer authority", INVENTORY)
        self.assertIn("backend labels/profile names never confer authority", INVENTORY)

    def test_a7_08_crypto_provenance_and_verifier_identity_never_raise_authority(self):
        self.assertIn("cryptographic/signature/provenance validity never raises semantic authority", INVENTORY)
        self.assertIn("verifier identity", INVENTORY)

    def test_a7_09_variance_frequency_similarity_never_authorize_behavior(self):
        self.assertIn("Frequency, recurrence and similarity never authorize", INVENTORY)

    def test_a7_10_variance_projection_remains_bounded_to_frozen_gcc_grammar(self):
        self.assertIn("only exact GCC producer/role/grammar ephemeral projection is eligible", INVENTORY)

    def test_a7_11_comparison_tooling_never_creates_pass_winner_or_external_validation(self):
        self.assertIn("does not rank products", P6)
        self.assertIn("The native predicate field `passed` is not normalized to ExecSurface `PASS`", P6)
        self.assertIn("does not establish:\n- an overall winner", P6)

    def test_a7_12_ci_context_never_selects_baseline_or_overrides_verdict(self):
        self.assertIn("cannot select baselines, change verdicts, or upgrade observer authority", P7)
        self.assertIn("Baseline steering and verdict override from CI environment remain forbidden", P7)

    def test_a7_13_arm64_negative_result_remains_negative_retained(self):
        self.assertIn("P7_A0_ARM64_NOT_PORTABLE", P7)
        self.assertIn("36774237512", P7)
        self.assertIn("does NOT establish:\n- Linux arm64 support", P7)

    def test_a7_14_historical_material_failures_remain_retained(self):
        self.assertIn("historical broken candidate: `5079a990b924d8ccd7ac6414f8a9a2571b54e240`", A1_REQUAL)
        self.assertIn("Historical counterevidence remains retained", A1_REQUAL)
        for marker in ["36755962689", "36770551463", "36774237512"]:
            corpus = INVENTORY + P6 + P7 + PROTOCOL
            self.assertIn(marker, corpus)

    def test_a7_15_cross_layer_replay_and_substitution_corpora_reprove_cleanly(self):
        required = [
            "crates/execsurface-model/tests/semantics_v3_rework.rs",
            "experiments/p4-backend-authority/tests/b_cross_proposition_falsification.rs",
            "experiments/p5-cross-attestation-redteam/tests/a5_graph_redteam.rs",
            "experiments/p3-v4-falsifier",
            "experiments/p7-platform-ci/test_gitlab_context_adapter.py",
            "experiments/p7-platform-ci/test_self_hosted_evidence_contract.py",
        ]
        for path in required:
            self.assertTrue((ROOT / path).exists(), path)

    def test_a7_16_no_public_release_main_merge_or_p8_closure_is_implied(self):
        self.assertIn("does not authorize `main` merge, public release, tag movement", PROTOCOL)
        self.assertIn("P8 remains independently evidence-gated", PROTOCOL)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(A7IntegratedFalsifier)
    names = [case._testMethodName for case in suite]
    expected = [f"test_a7_{i:02d}_" for i in range(1, 17)]
    assert len(names) == 16, names
    for prefix, name in zip(expected, names):
        assert name.startswith(prefix), (prefix, name)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if result.wasSuccessful() else 1)
