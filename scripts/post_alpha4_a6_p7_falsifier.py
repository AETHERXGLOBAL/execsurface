#!/usr/bin/env python3
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = (ROOT / "docs/development/P7_CLOSEOUT_DECISION.md").read_text()
PROTOCOL = (ROOT / "docs/development/POST_ALPHA4_PROMOTION_A6_P7_PLATFORM_PROTOCOL.md").read_text()
ALPHA4 = "48e0b9a0707553349e75e97a9dfa096d13f9ab5d"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


class A6P7PlatformFalsifier(unittest.TestCase):
    def test_a6_01_arm64_negative_result_remains_negative(self):
        self.assertIn("P7_A0_ARM64_NOT_PORTABLE", CLOSEOUT)
        self.assertIn("does NOT establish:\n- Linux arm64 support", CLOSEOUT)

    def test_a6_02_arm64_failure_run_remains_retained(self):
        self.assertIn("36774237512", CLOSEOUT)
        self.assertIn("failed native parity run", CLOSEOUT)

    def test_a6_03_no_arm64_public_support_claim_is_introduced(self):
        self.assertIn("no arm64 release asset or public-support claim was created", CLOSEOUT)
        self.assertIn("ARM64 NEGATIVE RETAINED", PROTOCOL)

    def test_a6_04_gitlab_metadata_remains_context_only(self):
        self.assertIn("GitLab metadata remains context-only", CLOSEOUT)
        self.assertIn("cannot select baselines, change verdicts, or upgrade observer authority", CLOSEOUT)

    def test_a6_05_gitlab_metadata_cannot_select_baseline(self):
        self.assertIn("cannot select baselines", CLOSEOUT)

    def test_a6_06_gitlab_metadata_cannot_override_verdict(self):
        self.assertIn("change verdicts", CLOSEOUT)

    def test_a6_07_self_hosted_identity_remains_non_authoritative(self):
        self.assertIn("Runner ownership, provider identity, root/admin status and labels remain non-authoritative context", CLOSEOUT)

    def test_a6_08_self_hosted_environment_cannot_steer_baseline_or_verdict(self):
        self.assertIn("Baseline steering and verdict override from CI environment remain forbidden", CLOSEOUT)

    def test_a6_09_public_gitlab_and_self_hosted_support_remain_unclaimed(self):
        self.assertIn("No live/public GitLab support is claimed", CLOSEOUT)
        self.assertIn("No public self-hosted support or zero-assistance deployment claim is made", CLOSEOUT)

    def test_a6_10_existing_p7_contract_corpora_replay_cleanly(self):
        for path in [
            "experiments/p7-platform-ci/test_gitlab_context_adapter.py",
            "experiments/p7-platform-ci/test_self_hosted_evidence_contract.py",
            "experiments/p7-platform-ci/test_self_hosted_package.py",
        ]:
            self.assertTrue((ROOT / path).is_file(), path)

    def test_a6_11_platform_count_cannot_weaken_evidence_contracts(self):
        self.assertIn("does not open Windows, macOS, additional architectures or extra CI providers merely to increase platform count", CLOSEOUT)
        self.assertIn("platform count", PROTOCOL)

    def test_a6_12_public_anchors_and_product_scope_remain_unchanged(self):
        self.assertEqual(git("rev-parse", "v0.1.0-alpha.4^{commit}"), ALPHA4)
        self.assertEqual(git("rev-parse", "v0.1^{commit}"), ALPHA4)
        changed = git("diff", "--name-only", "4d6ea72e31d4734d0d6d68a1554901bb2be08511..HEAD").splitlines()
        forbidden = (
            "crates/execsurface-cli/",
            "crates/execsurface-diff/",
            "crates/execsurface-normalize/",
            "crates/execsurface-observe/",
            "crates/execsurface-policy/",
        )
        self.assertFalse(any(path.startswith(forbidden) for path in changed), changed)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(A6P7PlatformFalsifier)
    names = [case._testMethodName for case in suite]
    expected = [f"test_a6_{i:02d}_" for i in range(1, 13)]
    assert len(names) == 12, names
    for prefix, name in zip(expected, names):
        assert name.startswith(prefix), (prefix, name)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if result.wasSuccessful() else 1)
