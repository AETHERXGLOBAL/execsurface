"""Adversarial, stdlib-only privacy tests for the P8-A3.5 external trial harness.

Synthetic content only: do not insert real credentials or private paths.
"""
from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parents[1]
WRAPPER = """
import sys
sys.path.insert(0, sys.argv[1])
from p8_a3_external_trial_capture import run_privacy_bounded
result = run_privacy_bounded([sys.executable, "-c", sys.argv[2]], cwd=__import__("pathlib").Path(sys.argv[3]))
if len(sys.argv) > 4 and sys.argv[4] == "safe-status":
    print(f"CHILD_STATUS={result.returncode}")
    raise SystemExit(0)
raise SystemExit(result.returncode)
"""


class PrivacyBoundaryTests(unittest.TestCase):
    def run_wrapped(self, child_code: str, safe_status: bool = False) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as workdir:
            return subprocess.run(
                [sys.executable, "-c", WRAPPER, str(SCRIPT_DIR), child_code, workdir, "safe-status" if safe_status else "exit"],
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )

    def assert_no_spill(self, result: subprocess.CompletedProcess[str]) -> None:
        # Never put the child output in assertion messages (CI may be public).
        self.assertFalse(result.stdout, "child stdout escaped public trial harness")
        self.assertFalse(result.stderr, "child stderr escaped public trial harness")

    def test_success_suppresses_both_streams(self) -> None:
        result = self.run_wrapped(
            'import sys; print("SYNTHETIC_STDOUT_SECRET"); '
            'sys.stderr.write("SYNTHETIC_STDERR_SECRET\\n")'
        )
        self.assertEqual(result.returncode, 0)
        self.assert_no_spill(result)

    def test_failure_keeps_exact_nonzero_status_without_streams(self) -> None:
        result = self.run_wrapped(
            'import sys; print("SYNTHETIC_STDOUT_SECRET"); '
            'sys.stderr.write("SYNTHETIC_STDERR_SECRET\\n"); '
            'sys.exit(19)'
        )
        self.assertEqual(result.returncode, 19)
        self.assert_no_spill(result)

    def test_large_child_output_does_not_block_or_escape(self) -> None:
        result = self.run_wrapped(
            'import os; os.write(1, b"S" * 2000000); '
            'os.write(2, b"E" * 2000000)'
        )
        self.assertEqual(result.returncode, 0)
        self.assert_no_spill(result)

    def test_signal_termination_is_not_normalized(self) -> None:
        if sys.platform == "win32":
            self.skipTest("POSIX signal return semantics")
        result = self.run_wrapped('import os, signal; os.kill(os.getpid(), signal.SIGTERM)', safe_status=True)
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout.strip(), "CHILD_STATUS=-15", "POSIX signal exit must stay negative")
        self.assertFalse(result.stderr, "signal-path error output escaped")


if __name__ == "__main__":
    unittest.main()
