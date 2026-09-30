import unittest

from gitlab_context_adapter import AUTHORITY, ContextError, bind_gitlab_context


class GitLabContextAdapterTests(unittest.TestCase):
    def valid_env(self):
        return {
            "CI_PROJECT_PATH": "aether-x/execsurface",
            "CI_COMMIT_SHA": "A" * 40,
            "CI_PIPELINE_ID": "12345",
            "CI_JOB_ID": "67890",
            "CI_JOB_NAME": "runtime verification",
            "CI_PIPELINE_SOURCE": "push",
        }

    def test_01_complete_valid_context_maps_successfully(self):
        bound = bind_gitlab_context(self.valid_env())
        out = bound.output()
        self.assertEqual(out["schema"], "execsurface.gitlab-ci-context.v1")
        self.assertEqual(out["provider"], "gitlab-ci")
        self.assertEqual(out["authority"], "context_only")
        self.assertEqual(out["commit_sha"], "a" * 40)

    def test_02_repeated_mapping_is_byte_deterministic(self):
        first = bind_gitlab_context(self.valid_env())
        second = bind_gitlab_context(dict(reversed(list(self.valid_env().items()))))
        self.assertEqual(first.canonical_json, second.canonical_json)
        self.assertEqual(first.context_digest, second.context_digest)

    def test_03_commit_substitution_changes_digest(self):
        first = bind_gitlab_context(self.valid_env())
        changed = self.valid_env()
        changed["CI_COMMIT_SHA"] = "b" * 40
        second = bind_gitlab_context(changed)
        self.assertNotEqual(first.context_digest, second.context_digest)

    def test_04_pipeline_or_job_replay_changes_digest(self):
        first = bind_gitlab_context(self.valid_env())
        pipeline = self.valid_env()
        pipeline["CI_PIPELINE_ID"] = "12346"
        job = self.valid_env()
        job["CI_JOB_ID"] = "67891"
        self.assertNotEqual(first.context_digest, bind_gitlab_context(pipeline).context_digest)
        self.assertNotEqual(first.context_digest, bind_gitlab_context(job).context_digest)

    def test_05_missing_required_field_fails_closed(self):
        for name in tuple(self.valid_env()):
            env = self.valid_env()
            del env[name]
            with self.assertRaises(ContextError, msg=name):
                bind_gitlab_context(env)

    def test_06_malformed_commit_identity_fails_closed(self):
        for value in ("abc", "g" * 40, "a" * 39, "a" * 41):
            env = self.valid_env()
            env["CI_COMMIT_SHA"] = value
            with self.assertRaises(ContextError):
                bind_gitlab_context(env)

    def test_07_newline_or_control_injection_fails_closed(self):
        for field in ("CI_PROJECT_PATH", "CI_JOB_NAME", "CI_PIPELINE_SOURCE"):
            env = self.valid_env()
            env[field] = env[field] + "\nINJECTED=1"
            with self.assertRaises(ContextError, msg=field):
                bind_gitlab_context(env)

    def test_08_malformed_numeric_ids_fail_closed(self):
        for field in ("CI_PIPELINE_ID", "CI_JOB_ID"):
            for value in ("0", "-1", "1.5", "12x", "1\n2"):
                env = self.valid_env()
                env[field] = value
                with self.assertRaises(ContextError, msg=f"{field}:{value!r}"):
                    bind_gitlab_context(env)

    def test_09_project_path_injection_or_traversal_fails_closed(self):
        for value in ("../execsurface", "aether-x/../execsurface", "/root/project", "aether-x//execsurface", "single"):
            env = self.valid_env()
            env["CI_PROJECT_PATH"] = value
            with self.assertRaises(ContextError, msg=value):
                bind_gitlab_context(env)

    def test_10_unknown_environment_variables_do_not_change_output(self):
        base = bind_gitlab_context(self.valid_env())
        env = self.valid_env()
        env.update({"UNRELATED": "value", "CI_SECRET_TOKEN": "do-not-bind"})
        changed = bind_gitlab_context(env)
        self.assertEqual(base.canonical_json, changed.canonical_json)
        self.assertEqual(base.context_digest, changed.context_digest)

    def test_11_ci_variables_cannot_select_or_set_baseline(self):
        base = bind_gitlab_context(self.valid_env())
        env = self.valid_env()
        env.update({"EXECSURFACE_BASELINE": "/tmp/evil.json", "BASELINE_DIGEST": "deadbeef"})
        changed = bind_gitlab_context(env)
        self.assertEqual(base.context_digest, changed.context_digest)
        self.assertFalse(changed.output()["may_select_baseline"])
        self.assertNotIn("/tmp/evil.json", changed.canonical_json)
        self.assertNotIn("deadbeef", changed.canonical_json)

    def test_12_ci_variables_cannot_supply_or_override_verdict(self):
        base = bind_gitlab_context(self.valid_env())
        env = self.valid_env()
        env.update({"EXECSURFACE_VERDICT": "PASS", "VERDICT": "PASS"})
        changed = bind_gitlab_context(env)
        self.assertEqual(base.context_digest, changed.context_digest)
        self.assertFalse(changed.output()["may_change_verdict"])
        self.assertNotIn('"verdict"', changed.canonical_json)

    def test_13_provider_or_backend_name_cannot_upgrade_authority(self):
        env = self.valid_env()
        env.update({"CI_PROVIDER": "trusted-kernel", "BACKEND": "authoritative", "AUTHORITY": "direct"})
        bound = bind_gitlab_context(env)
        self.assertEqual(bound.output()["authority"], AUTHORITY)
        self.assertFalse(bound.output()["may_upgrade_observer_authority"])

    def test_14_incomplete_context_cannot_be_complete_authoritative_binding(self):
        env = self.valid_env()
        env["CI_JOB_NAME"] = ""
        with self.assertRaises(ContextError):
            bind_gitlab_context(env)


if __name__ == "__main__":
    unittest.main()
