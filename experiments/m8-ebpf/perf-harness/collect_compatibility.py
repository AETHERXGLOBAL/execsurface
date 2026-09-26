#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import platform
import subprocess
import tempfile
from pathlib import Path

ATTACHMENT_POINTS = [
    "sched/sched_process_exec",
    "sched/sched_process_exit",
    "syscalls/sys_exit_fork",
    "syscalls/sys_exit_vfork",
    "syscalls/sys_enter_clone",
    "syscalls/sys_exit_clone",
    "syscalls/sys_exit_clone3",
    "syscalls/sys_exit_openat",
]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--collector", required=True)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--commit-sha", required=True)
    return parser.parse_args()


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json_if_present(path):
    try:
        return json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def report_summary(report):
    if not isinstance(report, dict):
        return None
    failure = report.get("collector_failure")
    return {
        "backend_id": report.get("backend", {}).get("id"),
        "kernel_release": report.get("backend", {}).get("kernel_release"),
        "kernel_btf_readable": report.get("backend", {}).get("kernel_btf_readable"),
        "completeness": report.get("completeness"),
        "observation_complete": report.get("observation_complete"),
        "dropped_events": report.get("dropped_events"),
        "lifecycle_drain_complete": report.get("lifecycle_drain_complete"),
        "collector_failure_stage": failure.get("stage") if isinstance(failure, dict) else None,
        "target_started": failure.get("target_started") if isinstance(failure, dict) else True,
        "event_count": len(report.get("events", [])),
        "warning_codes": sorted({w.get("code") for w in report.get("warnings", []) if isinstance(w, dict)}),
    }


def run_probe(command, report_path):
    proc = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    report = read_json_if_present(report_path)
    return {
        "exit_code": proc.returncode,
        "stdout_sha256": hashlib.sha256(proc.stdout).hexdigest(),
        "stderr_tail": proc.stderr.decode("utf-8", errors="replace")[-2000:],
        "report": report_summary(report),
    }


def tracepoint_state():
    roots = [Path("/sys/kernel/tracing/events"), Path("/sys/kernel/debug/tracing/events")]
    result = {}
    for point in ATTACHMENT_POINTS:
        found = False
        matched = None
        for root in roots:
            candidate = root / point
            if candidate.is_dir():
                found = True
                matched = str(candidate)
                break
        result[point] = {"present": found, "path": matched}
    return result


def main():
    args = parse_args()
    collector = str(Path(args.collector).resolve())
    fixture = str(Path(args.fixture).resolve())
    btf = Path("/sys/kernel/btf/vmlinux")

    with tempfile.TemporaryDirectory(prefix="execsurface-m86-compat-") as tempdir:
        unpriv_report = str(Path(tempdir) / "unpriv.json")
        priv_report = str(Path(tempdir) / "priv.json")

        unprivileged = run_probe(
            [collector, "--report", unpriv_report, "--", fixture, "short_exec"],
            unpriv_report,
        )
        privileged = run_probe(
            ["sudo", collector, "--report", priv_report, "--", fixture, "short_exec"],
            priv_report,
        )

    evidence = {
        "schema_version": 1,
        "claim_scope": "compatibility evidence for this exact tested host only",
        "repository_commit_sha": args.commit_sha,
        "host": {
            "kernel_release": platform.release(),
            "machine": platform.machine(),
            "effective_uid": os.geteuid(),
            "btf_vmlinux_readable": os.access(btf, os.R_OK),
            "btf_vmlinux_sha256": sha256_file(btf) if btf.is_file() and os.access(btf, os.R_OK) else None,
        },
        "attachment_points": tracepoint_state(),
        "collector_sha256": sha256_file(collector),
        "fixture_sha256": sha256_file(fixture),
        "unprivileged_probe": unprivileged,
        "privileged_probe": privileged,
        "known_semantic_boundary": {
            "full_surface_comparable": False,
            "ebpf_pass_authorized": False,
            "clone3_universal_parity": False,
        },
    }

    Path(args.output).write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    priv = privileged.get("report") or {}
    if privileged["exit_code"] != 0:
        raise SystemExit("privileged libbpf compatibility probe failed")
    if priv.get("completeness") not in {"incomplete_capability"}:
        raise SystemExit(f"unexpected privileged completeness: {priv.get('completeness')}")
    if priv.get("dropped_events") != 0 or priv.get("lifecycle_drain_complete") is not True:
        raise SystemExit("privileged compatibility probe is not clean")
    print("M8_6_COMPATIBILITY_PROBE_PASS")


if __name__ == "__main__":
    main()
