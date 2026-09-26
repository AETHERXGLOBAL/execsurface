#!/usr/bin/env python3
"""Run one frozen M9.1 post-hardening zero-contact case.

This runner preserves failures as evidence. It does not relax M9 acceptance:
compatibility still requires install+doctor+learn+two unchanged PASS checks, and
performance still uses exactly 3 warmups and 15 measured samples per mode.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time

TIMEOUT_RC = 124
RUST_TOOLCHAIN = "1.90.0"
SAMPLE_TIMEOUT = 120.0

CASES = {
    "ZC-01-JUST": {
        "project": "just",
        "repository": "casey/just",
        "revision": "5d5742cbcc50f19c99c356bc7e085acaa5f4665d",
        "argv": ["cargo", f"+{RUST_TOOLCHAIN}", "test", "--all"],
    },
    "ZC-02-RIPGREP": {
        "project": "ripgrep",
        "repository": "BurntSushi/ripgrep",
        "revision": "3fce3b5bb0236da2df6d99672afb8a719642eca7",
        "argv": ["cargo", f"+{RUST_TOOLCHAIN}", "test", "--workspace", "--all-targets"],
    },
    "ZC-03-FZF": {
        "project": "fzf",
        "repository": "junegunn/fzf",
        "revision": "b1be3a8be1b833ce5b92fbbac11637643d60a046",
        "argv": ["go", "test", "./..."],
    },
}


def run_capture(argv: list[str], cwd: Path, stdout: Path, stderr: Path, timeout: float = SAMPLE_TIMEOUT) -> int:
    try:
        with stdout.open("wb") as out, stderr.open("wb") as err:
            proc = subprocess.run(argv, cwd=cwd, stdout=out, stderr=err, timeout=timeout, check=False)
        return proc.returncode
    except subprocess.TimeoutExpired:
        stderr.write_text(f"TIMEOUT after {timeout:.0f}s\n")
        return TIMEOUT_RC


def run_quiet(argv: list[str], cwd: Path, timeout: float = SAMPLE_TIMEOUT) -> int:
    try:
        proc = subprocess.run(argv, cwd=cwd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=timeout, check=False)
        return proc.returncode
    except subprocess.TimeoutExpired:
        return TIMEOUT_RC


def checked_json(path: Path) -> dict | None:
    try:
        data = json.loads(path.read_text())
    except Exception:
        return None
    outcome = data.get("outcome", {})
    if data.get("complete") is not True or outcome.get("exit_code") != 0 or outcome.get("signal") is not None:
        return None
    return data


def observe_argv(exe: Path, command: str) -> list[str]:
    # Child output is redirected only to keep the observer JSON channel parseable.
    wrapped = f"cd {sh_quote(str(WORK_ROOT))} && {command} >/dev/null 2>&1"
    return [str(exe), "observe", "--", "/bin/bash", "-lc", wrapped]


def wrapped_command(command: str) -> str:
    return f"cd {sh_quote(str(WORK_ROOT))} && {command} >/dev/null 2>&1"


def sh_quote(value: str) -> str:
    return "'" + value.replace("'", "'\"'\"'") + "'"


def measure_direct(command: str, target: Path) -> bool:
    start = time.perf_counter_ns()
    rc = run_quiet(["/bin/bash", "-lc", wrapped_command(command)], WORK_ROOT)
    elapsed = (time.perf_counter_ns() - start) / 1_000_000
    with target.open("a") as fh:
        fh.write(f"{elapsed:.6f}\n")
    return rc == 0


def measure_ptrace(exe: Path, command: str, timings: Path, seq: str, evidence: Path) -> bool:
    jout = evidence / f"perf-observe-{seq}.json"
    jerr = evidence / f"perf-observe-{seq}.stderr"
    start = time.perf_counter_ns()
    rc = run_capture(observe_argv(exe, command), WORK_ROOT, jout, jerr)
    elapsed = (time.perf_counter_ns() - start) / 1_000_000
    with timings.open("a") as fh:
        fh.write(f"{elapsed:.6f}\n")
    return rc == 0 and checked_json(jout) is not None


def values(path: Path) -> list[float]:
    return [float(line) for line in path.read_text().splitlines() if line.strip()]


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_raw_manifest(evidence: Path) -> str:
    manifest = evidence / "RAW_SHA256SUMS"
    rows = []
    for path in sorted(p for p in evidence.iterdir() if p.is_file() and p.name not in {"RAW_SHA256SUMS", "evidence.json", "evidence.sha256", "validator.txt"}):
        rows.append(f"{sha256_file(path)}  {path.name}")
    manifest.write_text("\n".join(rows) + "\n")
    return sha256_file(manifest)


def command_text(argv: list[str]) -> str:
    return " ".join(sh_quote(x) if any(c.isspace() for c in x) else x for x in argv)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("case_id", choices=sorted(CASES))
    parser.add_argument("--source", default="external/project")
    parser.add_argument("--install-root", default="/tmp/execsurface-m9-install")
    parser.add_argument("--work-root", default="/tmp/execsurface-m9-external")
    parser.add_argument("--evidence-root", default="m9-evidence")
    args = parser.parse_args()

    global WORK_ROOT
    WORK_ROOT = Path(args.work_root)
    source = Path(args.source).resolve()
    install_root = Path(args.install_root).resolve()
    evidence = (Path(args.evidence_root) / args.case_id).resolve()
    case = CASES[args.case_id]
    cmd_argv = case["argv"]
    command = command_text(cmd_argv)
    exe = install_root / "bin" / "execsurface"

    evidence.mkdir(parents=True, exist_ok=True)
    if WORK_ROOT.exists():
        shutil.rmtree(WORK_ROOT)
    shutil.copytree(source, WORK_ROOT, symlinks=True)

    revision = subprocess.check_output(["git", "-C", str(WORK_ROOT), "rev-parse", "HEAD"], text=True).strip()
    if revision != case["revision"]:
        raise SystemExit(f"pinned revision mismatch: {revision} != {case['revision']}")
    if subprocess.check_output(["git", "-C", str(WORK_ROOT), "status", "--porcelain"], text=True).strip():
        raise SystemExit("external tracked tree is not clean before execution")

    state = {
        "install_status": "PASS" if exe.exists() else "FAIL",
        "direct_status": "NOT_RUN",
        "doctor_status": "NOT_RUN",
        "baseline_status": "NOT_RUN",
        "check_status": "NOT_RUN",
        "check1_verdict": None,
        "check2_verdict": None,
        "performance_status": "NOT_RUN",
        "operational_error": None,
    }
    errors: list[str] = []

    # Direct compatibility run: capture upstream output instead of discarding it.
    if state["install_status"] == "PASS":
        direct_rc = run_capture(cmd_argv, WORK_ROOT, evidence / "direct.stdout", evidence / "direct.stderr")
        (evidence / "direct.status").write_text(f"direct_rc={direct_rc}\n")
        state["direct_status"] = "PASS" if direct_rc == 0 else "FAIL"
        if direct_rc != 0:
            errors.append(f"direct compatibility command failed rc={direct_rc}")

        doctor_rc = run_capture([str(exe), "doctor"], WORK_ROOT, evidence / "doctor.stdout", evidence / "doctor.stderr")
        (evidence / "doctor.status").write_text(f"doctor_rc={doctor_rc}\n")
        state["doctor_status"] = "PASS" if doctor_rc == 0 else "FAIL"
        if doctor_rc != 0:
            errors.append(f"doctor failed rc={doctor_rc}")

        if direct_rc == 0:
            observed = wrapped_command(command)
            baseline = evidence / "baseline.lock.json"
            learn_rc = run_capture(
                [str(exe), "learn", "--output", str(baseline), "--", "/bin/bash", "-lc", observed],
                WORK_ROOT,
                evidence / "learn.stdout",
                evidence / "learn.stderr",
            )
            (evidence / "learn.status").write_text(f"learn_rc={learn_rc}\n")
            state["baseline_status"] = "PASS" if learn_rc == 0 else "FAIL"
            if learn_rc != 0:
                errors.append(f"baseline creation failed rc={learn_rc}")
            else:
                verdicts = []
                check_ok = True
                for n in (1, 2):
                    jout = evidence / f"check-{n}.json"
                    rc = run_capture(
                        [str(exe), "check", "--baseline", str(baseline), "--json-output", str(jout), "--markdown-output", str(evidence / f"check-{n}.md"), "--", "/bin/bash", "-lc", observed],
                        WORK_ROOT,
                        evidence / f"check-{n}.stdout",
                        evidence / f"check-{n}.stderr",
                    )
                    data = json.loads(jout.read_text()) if jout.exists() else {}
                    verdict = data.get("verdict")
                    findings = len(data.get("findings", []))
                    verdicts.append((n, rc, verdict, findings))
                    state[f"check{n}_verdict"] = verdict
                    if rc != 0 or verdict != "pass" or findings != 0:
                        check_ok = False
                (evidence / "check-verdicts.txt").write_text("".join(f"check{n}={v} findings={f} rc={rc}\n" for n, rc, v, f in verdicts))
                state["check_status"] = "PASS" if check_ok else "FAIL"
                if not check_ok:
                    errors.append("unchanged rerun/check did not produce two zero-finding PASS results")

            # Performance is attempted after direct success and one complete observed run,
            # even if the unchanged compatibility checks report drift.
            preperf = evidence / "preperf-observe.json"
            preperf_rc = run_capture(observe_argv(exe, command), WORK_ROOT, preperf, evidence / "preperf-observe.stderr")
            (evidence / "preperf.status").write_text(f"preperf_rc={preperf_rc}\n")
            if preperf_rc == 0 and checked_json(preperf) is not None:
                wd = evidence / "warmup-direct-ms.txt"
                wp = evidence / "warmup-ptrace-ms.txt"
                md = evidence / "measured-direct-ms.txt"
                mp = evidence / "measured-ptrace-ms.txt"
                for p in (wd, wp, md, mp):
                    p.write_text("")
                perf_ok = True
                for i in range(1, 4):
                    perf_ok = measure_direct(command, wd) and perf_ok
                for i in range(1, 4):
                    perf_ok = measure_ptrace(exe, command, wp, f"warmup-{i}", evidence) and perf_ok
                if perf_ok:
                    for i in range(1, 16):
                        if i % 2:
                            perf_ok = measure_direct(command, md) and perf_ok
                            perf_ok = measure_ptrace(exe, command, mp, f"measured-{i}-a", evidence) and perf_ok
                        else:
                            perf_ok = measure_ptrace(exe, command, mp, f"measured-{i}-a", evidence) and perf_ok
                            perf_ok = measure_direct(command, md) and perf_ok
                        if not perf_ok:
                            break
                state["performance_status"] = "PASS" if perf_ok and len(values(md)) == 15 and len(values(mp)) == 15 else "FAIL"
                if state["performance_status"] != "PASS":
                    errors.append("performance series failed or incomplete")
            else:
                state["performance_status"] = "FAIL"
                errors.append("pre-performance observed command was incomplete or failed")
    else:
        errors.append("ExecSurface source installation failed")

    state["operational_error"] = "; ".join(errors) or None
    (evidence / "state.json").write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")

    env = {
        "run_id": os.getenv("GITHUB_RUN_ID", "unknown"),
        "run_attempt": os.getenv("GITHUB_RUN_ATTEMPT", "unknown"),
        "execsurface_source_sha": os.getenv("GITHUB_SHA", "unknown"),
        "project": case["repository"],
        "revision": case["revision"],
        "kernel": subprocess.check_output(["uname", "-srmo"], text=True).strip(),
        "architecture": subprocess.check_output(["uname", "-m"], text=True).strip(),
        "command": command,
    }
    try:
        env["cpu"] = subprocess.check_output("lscpu | sed -n 's/^Model name:[[:space:]]*//p' | head -1", shell=True, text=True).strip()
    except Exception:
        env["cpu"] = "unavailable"
    (evidence / "environment.json").write_text(json.dumps(env, indent=2, sort_keys=True) + "\n")

    manifest_sha = write_raw_manifest(evidence)
    try:
        version = subprocess.check_output([str(exe), "--version"], text=True, stderr=subprocess.STDOUT).strip()
    except Exception:
        version = "unavailable"

    performance = None
    if state["performance_status"] == "PASS":
        wd = values(evidence / "warmup-direct-ms.txt")
        wp = values(evidence / "warmup-ptrace-ms.txt")
        direct = values(evidence / "measured-direct-ms.txt")
        ptrace = values(evidence / "measured-ptrace-ms.txt")
        median_direct = float(statistics.median(direct))
        median_ptrace = float(statistics.median(ptrace))
        overhead = median_ptrace - median_direct
        ratio = median_ptrace / median_direct if median_direct > 0 else float("inf")
        performance = {
            "warmup_direct_ms": wd,
            "warmup_ptrace_ms": wp,
            "measured_direct_ms": direct,
            "measured_ptrace_ms": ptrace,
            "median_direct_ms": median_direct,
            "median_ptrace_ms": median_ptrace,
            "median_absolute_overhead_ms": overhead,
            "slowdown_ratio": ratio,
            "timeout_seconds": SAMPLE_TIMEOUT,
            "m65_trigger_crossed": (median_direct >= 100.0 and ratio > 2.0) or overhead > 500.0,
            "excluded_samples": [],
        }

    compatibility_ok = all(state[k] == "PASS" for k in ("install_status", "doctor_status", "baseline_status", "check_status"))
    claim_status = "COMPUTATIONAL_EVIDENCE" if compatibility_ok and state["performance_status"] == "PASS" else "PARTIAL"
    run_url = f"https://github.com/{os.getenv('GITHUB_REPOSITORY', 'AETHERXGLOBAL/execsurface')}/actions/runs/{os.getenv('GITHUB_RUN_ID', 'unknown')}"
    failures = []
    if state["operational_error"]:
        failures.append({"kind": "runtime_or_measurement_failure", "description": state["operational_error"], "retained": True})

    record = {
        "schema_version": "m9-evidence-v1",
        "evidence_id": f"M9-{args.case_id}-POSTHARDENING-002",
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "independence_class": "ZERO_CONTACT_EXTERNAL_REPRO",
        "claim_status": claim_status,
        "external_reference": f"https://github.com/{case['repository']}/commit/{case['revision']}",
        "provenance": {"initiated_by": "AETHER_X", "executed_by": "AETHER_X", "aether_x_material_involvement": True, "third_party_attestation": None},
        "project": {"name": case["project"], "repository": f"https://github.com/{case['repository']}", "revision": case["revision"], "modified_by_aether_x": False},
        "execsurface": {"version": version, "source_sha": os.getenv("GITHUB_SHA", "unknown"), "install_path": "cargo install --locked --path crates/execsurface-cli from main-derived M9.1 branch", "backend": "ptrace"},
        "host": {"os": "Ubuntu 24.04 GitHub-hosted runner", "kernel": env["kernel"], "architecture": env["architecture"], "cpu_description": env["cpu"], "runner_image": "ubuntu-24.04"},
        "workflow": {"command": command, "baseline_attempted": state["baseline_status"] != "NOT_RUN", "rerun_check_attempted": state["check_status"] != "NOT_RUN", "drift_case_attempted": False, "notes": "M9.1B recovery run. Same frozen workload/revision and 3+15 protocol; state is JSON-safe. Child output is redirected only during observed runs to preserve the observer JSON channel."},
        "outcome": {"installation": state["install_status"], "doctor": state["doctor_status"], "baseline": state["baseline_status"], "rerun_check": state["check_status"], "drift_case": "NOT_RUN", "verdict": "PASS" if state["check_status"] == "PASS" else None, "comparable": compatibility_ok, "complete": compatibility_ok, "operational_error": state["operational_error"], "false_positive_reported": False, "suspected_false_negative": False, "usability_friction": None},
        "privacy": {"metadata_only_confirmed": True, "prohibited_sensitive_payload_collected": False, "notes": "No file contents, environment values, stdin contents, network payloads, secrets, credentials, or unrestricted argv were collected into accepted evidence."},
        "performance": performance,
        "artifacts": [{"name": "raw evidence SHA-256 manifest", "sha256": manifest_sha, "location": run_url}],
        "failures_exclusions": failures,
        "reproduction": {"instructions": f"Checkout {case['repository']}@{case['revision']}; use ExecSurface source {os.getenv('GITHUB_SHA', 'unknown')}; prime once; run doctor, learn, two unchanged checks, then frozen 3+15 timing protocol with 120s/sample timeout.", "expected_scope": "Exact GitHub-hosted Ubuntu 24.04 zero-contact external reproduction; not independent adoption and not a universal Linux performance claim."},
    }
    (evidence / "evidence.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    (evidence / "evidence.sha256").write_text(f"{sha256_file(evidence / 'evidence.json')}  evidence.json\n")

    print(json.dumps({"case": args.case_id, "state": state, "claim_status": claim_status}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
