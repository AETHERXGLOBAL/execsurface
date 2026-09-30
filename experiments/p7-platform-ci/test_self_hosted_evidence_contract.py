import unittest

from self_hosted_evidence_contract import (
    ALPHA4_SOURCE,
    CONTEXT_AUTHORITY,
    ContractError,
    bind_verification_evidence,
    build_package_contract,
)


class SelfHostedEvidenceContractTests(unittest.TestCase):
    def valid_runner(self):
        return {
            "provider": "github-actions-self-hosted",
            "runner_id": "runner-01",
            "os": "linux",
            "arch": "x86_64",
            "labels": ["self-hosted", "linux", "x64"],
        }

    def bind(self, package=None, **overrides):
        package = package or build_package_contract(self.valid_runner())
        args = {
            "verdict": "REVIEW",
            "evidence_digest": "b" * 64,
            "observer_authority": "direct",
            "observer_completeness": "complete",
        }
        args.update(overrides)
        return bind_verification_evidence(package, **args)

    def test_01_valid_linux_x86_64_is_eligible_and_deterministic(self):
        first = build_package_contract(self.valid_runner())
        second = build_package_contract(self.valid_runner())
        self.assertTrue(first.output()["package_eligible"])
        self.assertIsNone(first.output()["ineligible_reason"])
        self.assertEqual(first.output()["execsurface_source"], ALPHA4_SOURCE)
        self.assertEqual(first.package_digest, second.package_digest)
        self.assertEqual(first.canonical_json, second.canonical_json)

    def test_02_label_order_and_duplicates_canonicalize(self):
        a = self.valid_runner()
        b = self.valid_runner()
        a["labels"] = ["linux", "self-hosted", "x64", "linux"]
        b["labels"] = ["x64", "linux", "self-hosted"]
        first = build_package_contract(a)
        second = build_package_contract(b)
        self.assertEqual(first.package_digest, second.package_digest)
        self.assertEqual(first.output()["runner"]["labels"], ["linux", "self-hosted", "x64"])

    def test_03_runner_id_substitution_changes_package_digest(self):
        first = build_package_contract(self.valid_runner())
        env = self.valid_runner()
        env["runner_id"] = "runner-02"
        second = build_package_contract(env)
        self.assertNotEqual(first.package_digest, second.package_digest)

    def test_04_label_substitution_changes_provenance_not_authority(self):
        first = build_package_contract(self.valid_runner())
        env = self.valid_runner()
        env["labels"] = ["self-hosted", "linux", "x64", "gpu"]
        second = build_package_contract(env)
        self.assertNotEqual(first.package_digest, second.package_digest)
        self.assertEqual(first.output()["runner"]["authority"], CONTEXT_AUTHORITY)
        self.assertEqual(second.output()["runner"]["authority"], CONTEXT_AUTHORITY)

    def test_05_trusted_prod_root_labels_cannot_upgrade_authority(self):
        env = self.valid_runner()
        env["labels"] = ["self-hosted", "trusted", "prod", "root", "authoritative"]
        package = build_package_contract(env)
        self.assertEqual(package.output()["runner"]["authority"], "context_only")
        self.assertFalse(package.output()["may_upgrade_observer_authority"])
        envelope = self.bind(package, observer_authority="attempt_only")
        self.assertEqual(envelope.output()["observer_authority"], "attempt_only")
        self.assertFalse(envelope.output()["authority_upgraded_by_runner"])

    def test_06_missing_required_runner_context_fails_closed(self):
        for field in ("provider", "runner_id", "os", "arch", "labels"):
            env = self.valid_runner()
            del env[field]
            with self.assertRaises(ContractError, msg=field):
                build_package_contract(env)

    def test_07_control_injection_in_runner_metadata_fails_closed(self):
        for field in ("provider", "runner_id", "os", "arch"):
            env = self.valid_runner()
            env[field] = str(env[field]) + "\nINJECTED=1"
            with self.assertRaises(ContractError, msg=field):
                build_package_contract(env)
        env = self.valid_runner()
        env["labels"] = ["self-hosted", "trusted\nroot"]
        with self.assertRaises(ContractError):
            build_package_contract(env)

    def test_08_arm64_is_explicitly_ineligible(self):
        env = self.valid_runner()
        env["arch"] = "arm64"
        package = build_package_contract(env)
        self.assertFalse(package.output()["package_eligible"])
        self.assertEqual(package.output()["ineligible_reason"], "public_alpha4_platform_not_supported")
        self.assertEqual(package.output()["runner"]["authority"], "context_only")

    def test_09_non_linux_is_explicitly_ineligible(self):
        env = self.valid_runner()
        env["os"] = "windows"
        package = build_package_contract(env)
        self.assertFalse(package.output()["package_eligible"])
        self.assertEqual(package.output()["ineligible_reason"], "public_alpha4_platform_not_supported")

    def test_10_environment_baseline_variables_cannot_select_baseline(self):
        base = build_package_contract(self.valid_runner())
        env = self.valid_runner()
        env.update({"EXECSURFACE_BASELINE": "/tmp/evil.json", "BASELINE_DIGEST": "c" * 64})
        injected = build_package_contract(env)
        self.assertEqual(base.package_digest, injected.package_digest)
        self.assertIsNone(injected.output()["baseline_reference"])
        self.assertFalse(injected.output()["may_infer_baseline"])

    def test_11_environment_verdict_variables_cannot_override_verdict(self):
        env = self.valid_runner()
        env.update({"EXECSURFACE_VERDICT": "PASS", "VERDICT": "PASS"})
        package = build_package_contract(env)
        envelope = self.bind(package, verdict="BLOCK")
        self.assertEqual(envelope.output()["verdict"], "BLOCK")
        self.assertFalse(package.output()["may_change_verdict"])

    def test_12_explicit_baseline_reference_must_be_null_or_sha256(self):
        null_ref = build_package_contract(self.valid_runner())
        self.assertIsNone(null_ref.output()["baseline_reference"])
        valid = build_package_contract(self.valid_runner(), baseline_reference="D" * 64)
        self.assertEqual(valid.output()["baseline_reference"], "d" * 64)
        for value in ("", "abc", "g" * 64, "a" * 63, "/tmp/baseline.json"):
            with self.assertRaises(ContractError, msg=value):
                build_package_contract(self.valid_runner(), baseline_reference=value)

    def test_13_verification_values_bind_exactly_without_runner_upgrade(self):
        env = self.valid_runner()
        env["labels"] = ["trusted", "prod", "root", "direct-authority"]
        package = build_package_contract(env)
        envelope = self.bind(
            package,
            verdict="REVIEW",
            evidence_digest="E" * 64,
            observer_authority="derived",
            observer_completeness="complete",
        )
        out = envelope.output()
        self.assertEqual(out["verdict"], "REVIEW")
        self.assertEqual(out["evidence_digest"], "e" * 64)
        self.assertEqual(out["observer_authority"], "derived")
        self.assertEqual(out["observer_completeness"], "complete")
        self.assertEqual(out["runner_authority"], "context_only")
        self.assertFalse(out["authority_upgraded_by_runner"])

    def test_14_incomplete_or_lost_cannot_be_laundered_into_pass(self):
        env = self.valid_runner()
        env["labels"] = ["trusted", "prod", "complete", "pass"]
        package = build_package_contract(env)
        lost = self.bind(package, verdict="ERROR", observer_completeness="lost")
        self.assertEqual(lost.output()["observer_completeness"], "lost")
        self.assertFalse(lost.output()["completeness_upgraded_by_runner"])
        for state in ("incomplete", "lost", "ambiguous", "unsupported"):
            with self.assertRaises(ContractError, msg=state):
                self.bind(package, verdict="PASS", observer_completeness=state)

    def test_15_runner_context_replay_rebinds_evidence_digest(self):
        first_package = build_package_contract(self.valid_runner())
        changed = self.valid_runner()
        changed["runner_id"] = "runner-replayed"
        second_package = build_package_contract(changed)
        first = self.bind(first_package)
        second = self.bind(second_package)
        self.assertNotEqual(first_package.package_digest, second_package.package_digest)
        self.assertNotEqual(first.envelope_digest, second.envelope_digest)

    def test_16_unknown_environment_variables_do_not_change_outputs(self):
        base_env = self.valid_runner()
        extra_env = self.valid_runner()
        extra_env.update({"SECRET": "do-not-bind", "RUNNER_TRUST": "ultimate", "OWNER": "root"})
        base = build_package_contract(base_env)
        extra = build_package_contract(extra_env)
        self.assertEqual(base.package_digest, extra.package_digest)
        self.assertEqual(self.bind(base).envelope_digest, self.bind(extra).envelope_digest)


if __name__ == "__main__":
    unittest.main()
