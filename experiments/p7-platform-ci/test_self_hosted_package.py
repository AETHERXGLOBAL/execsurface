import hashlib
import json
import os
import tempfile
import unittest
from pathlib import Path

from self_hosted_package import (
    APPROVAL_ORIGIN,
    CONTEXT_AUTHORITY,
    PACKAGE_SHA256,
    PACKAGE_VERSION,
    ContractError,
    bind_environment,
    run_check,
    validate_config,
    verify_baseline,
)


class SelfHostedPackageTests(unittest.TestCase):
    def valid_config(self, baseline_sha="a" * 64):
        return {
            "baseline_path": "approved/execsurface.lock.json",
            "baseline_sha256": baseline_sha,
            "approval_origin": APPROVAL_ORIGIN,
            "package_version": PACKAGE_VERSION,
            "package_sha256": PACKAGE_SHA256,
        }

    def linux_env(self):
        return {
            "RUNNER_OS": "Linux",
            "RUNNER_ARCH": "X64",
            "CI_PROVIDER": "self-hosted-test",
            "RUNNER_NAME": "runner-01",
            "RUNNER_LABELS": "self-hosted,linux,x64,trusted,root",
        }

    def test_01_valid_configuration_maps_successfully(self):
        cfg = validate_config(self.valid_config())
        self.assertEqual(cfg.package_version, PACKAGE_VERSION)
        self.assertEqual(cfg.package_sha256, PACKAGE_SHA256)

    def test_02_canonical_configuration_serialization_is_deterministic(self):
        first = validate_config(self.valid_config())
        second = validate_config(dict(reversed(list(self.valid_config().items()))))
        self.assertEqual(first.canonical_json, second.canonical_json)
        self.assertEqual(first.config_digest, second.config_digest)

    def test_03_wrong_package_version_fails_closed(self):
        data = self.valid_config()
        data["package_version"] = "v9.9.9"
        with self.assertRaises(ContractError):
            validate_config(data)

    def test_04_wrong_package_sha256_fails_closed(self):
        data = self.valid_config()
        data["package_sha256"] = "b" * 64
        with self.assertRaises(ContractError):
            validate_config(data)

    def test_05_absolute_baseline_path_fails_closed(self):
        data = self.valid_config()
        data["baseline_path"] = "/tmp/evil.json"
        with self.assertRaises(ContractError):
            validate_config(data)

    def test_06_path_traversal_baseline_path_fails_closed(self):
        for path in ("../evil.json", "approved/../evil.json"):
            data = self.valid_config()
            data["baseline_path"] = path
            with self.assertRaises(ContractError, msg=path):
                validate_config(data)

    def test_07_malformed_baseline_sha256_fails_closed(self):
        for digest in ("abc", "G" * 64, "a" * 63, "a" * 65):
            data = self.valid_config(digest.lower())
            with self.assertRaises(ContractError, msg=digest):
                validate_config(data)

    def test_08_non_external_preapproved_origin_fails_closed(self):
        for origin in ("same_job_learn", "ci_env", "trusted_runner", ""):
            data = self.valid_config()
            data["approval_origin"] = origin
            with self.assertRaises(ContractError, msg=origin):
                validate_config(data)

    def test_09_unsupported_os_fails_closed(self):
        env = self.linux_env()
        env["RUNNER_OS"] = "Windows"
        with self.assertRaises(ContractError):
            bind_environment(env, kernel="test")

    def test_10_unsupported_architecture_fails_closed(self):
        env = self.linux_env()
        env["RUNNER_ARCH"] = "ARM64"
        with self.assertRaises(ContractError):
            bind_environment(env, kernel="test")

    def test_11_ci_env_cannot_replace_baseline_path(self):
        cfg = validate_config(self.valid_config())
        env = self.linux_env()
        env["EXECSURFACE_BASELINE"] = "/tmp/evil.json"
        ctx = bind_environment(env, kernel="test")
        self.assertEqual(cfg.baseline_path, "approved/execsurface.lock.json")
        self.assertNotIn("/tmp/evil.json", ctx.canonical_json)
        self.assertFalse(ctx.output()["may_select_baseline"])

    def test_12_ci_env_cannot_replace_baseline_digest(self):
        cfg = validate_config(self.valid_config())
        env = self.linux_env()
        env["BASELINE_SHA256"] = "deadbeef"
        ctx = bind_environment(env, kernel="test")
        self.assertEqual(cfg.baseline_sha256, "a" * 64)
        self.assertNotIn("deadbeef", ctx.canonical_json)

    def test_13_ci_env_cannot_override_verdict(self):
        env = self.linux_env()
        env.update({"VERDICT": "PASS", "EXECSURFACE_VERDICT": "PASS"})
        ctx = bind_environment(env, kernel="test")
        self.assertFalse(ctx.output()["may_change_verdict"])
        self.assertNotIn('"verdict"', ctx.canonical_json)

    def test_14_self_hosted_trusted_root_labels_cannot_upgrade_authority(self):
        ctx = bind_environment(self.linux_env(), kernel="test")
        self.assertEqual(ctx.output()["authority"], CONTEXT_AUTHORITY)
        self.assertFalse(ctx.output()["may_upgrade_observer_authority"])

    def test_15_missing_baseline_file_fails_closed(self):
        cfg = validate_config(self.valid_config())
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises((ContractError, FileNotFoundError)):
                verify_baseline(Path(tmp), cfg)

    def test_16_substituted_baseline_bytes_fail_digest_verification(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "approved" / "execsurface.lock.json"
            path.parent.mkdir()
            path.write_bytes(b"approved")
            approved_digest = hashlib.sha256(path.read_bytes()).hexdigest()
            cfg = validate_config(self.valid_config(approved_digest))
            path.write_bytes(b"substituted")
            with self.assertRaises(ContractError):
                verify_baseline(root, cfg)

    def test_17_matching_baseline_bytes_pass_digest_verification(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "approved" / "execsurface.lock.json"
            path.parent.mkdir()
            path.write_bytes(b"approved")
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            cfg = validate_config(self.valid_config(digest))
            self.assertEqual(verify_baseline(root, cfg), path.resolve())

    def test_18_wrapper_preserves_native_execsurface_exit_code_unchanged(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            baseline = root / "approved" / "execsurface.lock.json"
            baseline.parent.mkdir()
            baseline.write_bytes(b"approved")
            digest = hashlib.sha256(baseline.read_bytes()).hexdigest()
            cfg = validate_config(self.valid_config(digest))

            fake = root / "fake-execsurface"
            fake.write_text("#!/bin/sh\nexit 10\n", encoding="utf-8")
            fake.chmod(0o755)

            rc = run_check(
                binary=fake,
                workspace=root,
                config=cfg,
                reports_dir=root / "reports",
                command=["/bin/true"],
                env=self.linux_env(),
                kernel="test",
            )
            self.assertEqual(rc, 10)
            context = json.loads((root / "reports" / "environment-context.json").read_text())
            self.assertEqual(context["authority"], CONTEXT_AUTHORITY)
            self.assertFalse(context["may_change_verdict"])


if __name__ == "__main__":
    unittest.main()
