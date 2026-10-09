#!/usr/bin/env python3
"""Privacy-minimized harness for the P8-A3.5 external typed-evidence trial.

The harness deliberately does not record the evaluator's command, target
stdout/stderr, environment values, or file contents. It preserves only bounded
execution metadata, hashes, and independently validated evidence semantics.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import platform
import subprocess
import sys
from pathlib import Path
from typing import Any

PACKET_ID = "P8-A3.5-EXTERNAL-TYPED-EVIDENCE-TRIAL-v1"
FROZEN_TARGET_SHA = "cea214244d7efe05fea2108a5f5d8ed6287d9a3f"
SUMMARY_NAME = "trial-summary.json"
EVIDENCE_NAME = "typed-evidence.json"
CONSUMER_REPORT_NAME = "consumer-report.json"


class TrialError(RuntimeError):
    def __init__(self, code: str, message: str):
        super().__init__(message)
        self.code = code


def run_text(command: list[str], cwd: Path | None = None) -> tuple[int, str]:
    proc = subprocess.run(
        command,
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    return proc.returncode, proc.stdout.strip()


def run_privacy_bounded(command: list[str], cwd: Path) -> subprocess.CompletedProcess[bytes]:
    """Run an untrusted trial stage without copying child output into public CI logs.

    Never retain stdout/stderr or command text in the trial's shared artifacts.
    Exit codes remain available for stage-level, non-sensitive diagnostics.
    DEVNULL avoids unbounded PIPE memory and unbounded artifact creation.
    """
    return subprocess.run(
        command,
        cwd=cwd,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )


def git_head(path: Path) -> str:
    rc, out = run_text(["git", "rev-parse", "HEAD"], cwd=path)
    if rc != 0 or len(out) != 40:
        raise TrialError("git-head-unavailable", f"cannot resolve git HEAD for {path}")
    return out


def tracked_clean(path: Path) -> bool:
    rc, out = run_text(
        ["git", "status", "--porcelain", "--untracked-files=no"],
        cwd=path,
    )
    if rc != 0:
        raise TrialError("git-status-unavailable", f"cannot read git status for {path}")
    return out == ""


def inside(candidate: Path, root: Path) -> bool:
    return candidate == root or root in candidate.parents


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def tool_version(command: list[str]) -> str | None:
    rc, out = run_text(command)
    return out.splitlines()[0] if rc == 0 and out else None


def base_summary(packet_revision: str | None, rehearsal: bool) -> dict[str, Any]:
    return {
        "packet_id": PACKET_ID,
        "packet_revision": packet_revision,
        "product_target_sha": FROZEN_TARGET_SHA,
        "product_surface": "observe --evidence-output",
        "product_stability": "experimental-unreleased",
        "qualification_state": (
            "DISQUALIFIED_SELF_EVIDENCE"
            if rehearsal
            else "UNQUALIFIED_PENDING_EXTERNAL_EVIDENCE_REVIEW"
        ),
        "external_evidence_claimed": False,
        "p8_a1_claimed": False,
        "p8_a4_claimed": False,
        "p9_5_claimed": False,
        "workload_origin_verified_by_harness": False,
        "assistance_before_initial_result_recorded_by_harness": False,
        "privacy_capture_policy": {
            "command_recorded": False,
            "target_stdout_recorded": False,
            "target_stderr_recorded": False,
            "environment_values_recorded": False,
            "file_contents_recorded": False,
            "typed_evidence_may_contain_paths": True,
        },
        "environment": {
            "os": platform.system(),
            "kernel_release": platform.release(),
            "architecture": platform.machine(),
            "python": platform.python_version(),
            "rustc": tool_version(["rustc", "--version"]),
            "cargo": tool_version(["cargo", "--version"]),
        },
        "stages": {
            "preflight": "pending",
            "build": "pending",
            "doctor": "pending",
            "observe": "pending",
            "consumer": "pending",
        },
        "build_exit_code": None,
        "doctor_exit_code": None,
        "observe_exit_code": None,
        "consumer_exit_code": None,
        "target_outcome": None,
        "collection_health": None,
        "typed_effect_count": None,
        "evidence_sha256": None,
        "status": "IN_PROGRESS",
        "failure_code": None,
    }


def load_manifest(packet_root: Path) -> dict[str, Any]:
    path = packet_root / "docs/external/P8_A3_TYPED_EVIDENCE_TRIAL_MANIFEST.json"
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise TrialError("manifest-unavailable", str(error)) from error
    if value.get("packet_id") != PACKET_ID:
        raise TrialError("manifest-packet-id-mismatch", "trial manifest packet_id mismatch")
    if value.get("product_target", {}).get("source_sha") != FROZEN_TARGET_SHA:
        raise TrialError("manifest-target-mismatch", "trial manifest target SHA mismatch")
    return value


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expected-packet-sha", required=True)
    parser.add_argument("--target-source", required=True, type=Path)
    parser.add_argument("--workdir", required=True, type=Path)
    parser.add_argument("--output-dir", required=True, type=Path)
    parser.add_argument("--schema", type=Path)
    parser.add_argument("--consumer", type=Path)
    parser.add_argument("--rehearsal-self-evidence", action="store_true")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()

    command = list(args.command)
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        print("trial harness: missing evaluator-supplied command", file=sys.stderr)
        raise SystemExit(2)

    output_dir = args.output_dir.expanduser().resolve()
    if output_dir.exists():
        print("trial harness: output directory already exists; preserve the initial result", file=sys.stderr)
        raise SystemExit(2)
    output_dir.mkdir(parents=True, exist_ok=False)
    summary_path = output_dir / SUMMARY_NAME

    packet_root = Path(__file__).resolve().parent.parent
    target_source = args.target_source.expanduser().resolve()
    workdir = args.workdir.expanduser().resolve()
    schema = (
        args.schema.expanduser().resolve()
        if args.schema
        else packet_root / "schemas/execsurface-typed-evidence-experimental-v1.schema.json"
    )
    consumer = (
        args.consumer.expanduser().resolve()
        if args.consumer
        else packet_root / "scripts/p8_a3_typed_evidence_reference_consumer.py"
    )

    packet_revision: str | None = None
    summary = base_summary(packet_revision, args.rehearsal_self_evidence)
    write_json(summary_path, summary)

    try:
        if not target_source.is_dir():
            raise TrialError("target-source-missing", "target source directory does not exist")
        if not workdir.is_dir():
            raise TrialError("workdir-missing", "workload directory does not exist")
        if not schema.is_file():
            raise TrialError("schema-missing", "typed-evidence schema is missing")
        if not consumer.is_file():
            raise TrialError("consumer-missing", "reference consumer is missing")

        load_manifest(packet_root)

        packet_revision = git_head(packet_root)
        summary["packet_revision"] = packet_revision
        if packet_revision != args.expected_packet_sha:
            raise TrialError(
                "packet-revision-mismatch",
                "packet checkout does not match --expected-packet-sha",
            )
        if not tracked_clean(packet_root):
            raise TrialError("packet-worktree-dirty", "packet tracked files are modified")

        target_head = git_head(target_source)
        if target_head != FROZEN_TARGET_SHA:
            raise TrialError(
                "wrong-target-source",
                f"target source must be exact frozen commit {FROZEN_TARGET_SHA}",
            )
        if not tracked_clean(target_source):
            raise TrialError("target-worktree-dirty", "target tracked files are modified")

        if inside(workdir, packet_root) or inside(workdir, target_source):
            raise TrialError(
                "workload-directory-is-internal",
                "workload must be outside the packet and frozen ExecSurface source trees",
            )
        if inside(output_dir, packet_root) or inside(output_dir, target_source) or inside(output_dir, workdir):
            raise TrialError(
                "output-directory-overlap",
                "trial output directory must be outside packet, target source, and workload trees",
            )

        summary["stages"]["preflight"] = "pass"
        write_json(summary_path, summary)

        build = run_privacy_bounded(
            ["cargo", "build", "--locked", "-p", "execsurface"],
            cwd=target_source,
        )
        summary["build_exit_code"] = build.returncode
        if build.returncode != 0:
            summary["stages"]["build"] = "fail"
            raise TrialError(
                "build-failed",
                f"frozen target source did not build (exit code {build.returncode}); inspect privately",
            )
        if not tracked_clean(target_source):
            raise TrialError(
                "target-worktree-mutated-by-build",
                "build changed tracked files in frozen target source",
            )
        summary["stages"]["build"] = "pass"
        write_json(summary_path, summary)

        binary = target_source / "target/debug/execsurface"
        if not binary.is_file():
            raise TrialError("binary-missing-after-build", "execsurface binary not found")

        doctor = run_privacy_bounded([str(binary), "doctor"], cwd=workdir)
        summary["doctor_exit_code"] = doctor.returncode
        summary["stages"]["doctor"] = "pass" if doctor.returncode == 0 else "fail"
        write_json(summary_path, summary)
        if doctor.returncode != 0:
            raise TrialError(
                "doctor-failed",
                f"execsurface doctor did not pass (exit code {doctor.returncode}); inspect privately",
            )

        evidence = output_dir / EVIDENCE_NAME
        observed = run_privacy_bounded(
            [
                str(binary),
                "observe",
                "--evidence-output",
                str(evidence),
                "--",
                *command,
            ],
            cwd=workdir,
        )
        summary["observe_exit_code"] = observed.returncode
        summary["stages"]["observe"] = "pass" if observed.returncode == 0 else "fail"
        if evidence.is_file():
            summary["evidence_sha256"] = "sha256:" + sha256_file(evidence)
        write_json(summary_path, summary)
        if observed.returncode != 0:
            raise TrialError(
                "observe-failed",
                f"experimental observe/evidence generation returned exit code {observed.returncode}; preserve this initial result",
            )
        if not evidence.is_file():
            raise TrialError("evidence-missing", "observe returned success without evidence file")

        checked = subprocess.run(
            [sys.executable, str(consumer), str(evidence), "--schema", str(schema)],
            cwd=packet_root,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        summary["consumer_exit_code"] = checked.returncode
        summary["stages"]["consumer"] = "pass" if checked.returncode == 0 else "fail"
        write_json(summary_path, summary)
        if checked.returncode != 0:
            raise TrialError(
                "consumer-rejected",
                "independent reference consumer rejected the evidence; preserve this initial result",
            )

        try:
            consumer_report = json.loads(checked.stdout)
            evidence_value = json.loads(evidence.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            raise TrialError("validated-json-unreadable", str(error)) from error

        write_json(output_dir / CONSUMER_REPORT_NAME, consumer_report)
        summary["target_outcome"] = evidence_value["raw_observation"]["outcome"]
        summary["collection_health"] = evidence_value["collection_health"]["state"]
        summary["typed_effect_count"] = len(evidence_value["effects"])
        summary["status"] = "TRIAL_CAPTURE_COMPLETE_UNQUALIFIED"
        write_json(summary_path, summary)

        print(f"trial summary: {summary_path}")
        print(f"typed evidence: {evidence}")
        print(f"consumer report: {output_dir / CONSUMER_REPORT_NAME}")
        print("qualification: external evidence is NOT claimed by the harness")
    except TrialError as error:
        summary["status"] = "TRIAL_CAPTURE_FAILED_PRESERVE_INITIAL_RESULT"
        summary["failure_code"] = error.code
        write_json(summary_path, summary)
        print(f"trial harness: {error.code}: {error}", file=sys.stderr)
        print(f"preserved summary: {summary_path}", file=sys.stderr)
        raise SystemExit(2) from error


if __name__ == "__main__":
    main()
