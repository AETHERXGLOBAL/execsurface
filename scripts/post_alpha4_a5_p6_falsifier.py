#!/usr/bin/env python3
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = (ROOT / "docs/development/P6_CLOSEOUT_DECISION.md").read_text()
PROTOCOL = (ROOT / "docs/development/POST_ALPHA4_PROMOTION_A5_P6_COMPARISON_PROTOCOL.md").read_text()
MANIFEST = (ROOT / "experiments/p6-competitive-falsification/p6-a0-manifest.json").read_text()
OBS = (ROOT / "experiments/p6-competitive-falsification/p6-a1-observation-matrix.json").read_text()
ADV = (ROOT / "experiments/p6-competitive-falsification/p6-a2-adversarial-matrix.json").read_text()

ALPHA4 = "48e0b9a0707553349e75e97a9dfa096d13f9ab5d"
WORKLOAD_BLOB = "c5986f1acd8bc79a3945e613acc128d5c8a9d168"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


class A5P6ComparisonFalsifier(unittest.TestCase):
    def test_a5_01_closeout_decision_is_bounded_factual(self):
        self.assertIn("P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED", CLOSEOUT)
        self.assertIn("factual and bounded", CLOSEOUT)

    def test_a5_02_overall_winner_and_ranking_remain_disclaimed(self):
        self.assertIn("does not rank products", CLOSEOUT)
        self.assertIn("does not establish:\n- an overall winner", CLOSEOUT)
        self.assertIn("- performance ranking;", CLOSEOUT)
        self.assertIn("No product speed ranking is authorized.", CLOSEOUT)

    def test_a5_03_native_comparator_pass_is_not_execsurface_pass(self):
        self.assertIn("The native predicate field `passed` is not normalized to ExecSurface `PASS`.", CLOSEOUT)

    def test_a5_04_not_testable_yet_remains_coverage_gap(self):
        self.assertIn("`NOT_TESTABLE_YET`", CLOSEOUT)
        self.assertIn("bounded coverage gap, not a negative product result", CLOSEOUT)

    def test_a5_05_incomplete_execsurface_network_evidence_remains_retained(self):
        self.assertIn("S2 network scenario fails closed as `ERROR/INCOMPLETE`", CLOSEOUT)
        self.assertIn("ExecSurface alpha.4 S2 incompleteness", CLOSEOUT)

    def test_a5_06_historical_harness_failure_remains_retained(self):
        self.assertIn("36770551463", CLOSEOUT)
        self.assertIn("harness incorrectly stopped", CLOSEOUT)

    def test_a5_07_performance_noncomparison_boundary_remains_binding(self):
        self.assertIn("Decision: `NO_VALID_PERFORMANCE_COMPARISON`", CLOSEOUT)
        self.assertIn("would conflate different lifecycle, output, telemetry and completeness work", CLOSEOUT)

    def test_a5_08_workload_and_reproducibility_pins_remain_frozen(self):
        self.assertIn(WORKLOAD_BLOB, CLOSEOUT)
        self.assertEqual(git("hash-object", "experiments/p6-competitive-falsification/workload.sh"), WORKLOAD_BLOB)
        self.assertIn("unchanged comparator pins and acceptance criteria", CLOSEOUT)

    def test_a5_09_negative_evidence_set_remains_present(self):
        required = [
            "36770551463",
            "ExecSurface alpha.4 S2 incompleteness",
            "Tetragon live `NOT_TESTABLE_YET` status",
            "cicd-sensor unsigned copied-predicate mutability observation",
            "natural DNS/run-identity/artifact-digest variation",
            "decision not to create a misleading performance comparison",
        ]
        for item in required:
            self.assertIn(item, CLOSEOUT)

    def test_a5_10_comparison_cannot_manufacture_semantic_authority(self):
        self.assertIn("comparison/falsification tooling", PROTOCOL)
        self.assertIn("cannot substitute for Semantics-v3/P4/P5 correctness evidence", PROTOCOL)
        self.assertIn("No composite score and no partial-pass promotion.", PROTOCOL)
        self.assertTrue(MANIFEST.strip().startswith("{"))
        self.assertTrue(OBS.strip().startswith("{"))
        self.assertTrue(ADV.strip().startswith("{"))

    def test_a5_11_comparison_cannot_manufacture_external_validation(self):
        self.assertIn("cannot satisfy P8 external-independence requirements", PROTOCOL)
        self.assertIn("external-validation/adoption claims", PROTOCOL)

    def test_a5_12_public_anchors_and_product_scope_remain_unchanged(self):
        self.assertEqual(git("rev-parse", "v0.1.0-alpha.4^{commit}"), ALPHA4)
        self.assertEqual(git("rev-parse", "v0.1^{commit}"), ALPHA4)
        changed = git("diff", "--name-only", "3bb0749f08046cbe26de0d9117defc6c93466fa8..HEAD").splitlines()
        forbidden = (
            "crates/execsurface-cli/",
            "crates/execsurface-diff/",
            "crates/execsurface-normalize/",
            "crates/execsurface-observe/",
            "crates/execsurface-policy/",
        )
        self.assertFalse(any(path.startswith(forbidden) for path in changed), changed)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(A5P6ComparisonFalsifier)
    names = [case._testMethodName for case in suite]
    expected = [f"test_a5_{i:02d}_" for i in range(1, 13)]
    assert len(names) == 12, names
    for prefix, name in zip(expected, names):
        assert name.startswith(prefix), (prefix, name)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if result.wasSuccessful() else 1)
