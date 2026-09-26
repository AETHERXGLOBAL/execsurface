#!/usr/bin/env python3
"""Run the canonical post-M9.0 zero-contact case for casey/just.

The script deliberately records failures before exiting non-zero. It never turns
an observer/runtime failure into a clean M9.1 result merely because an artifact
was produced.
"""

from __future__ import annotations

import hashlib
import json
import os
import platform
import statistics
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

TARGET_REPOSITORY = "https://github.com/casey/just"
TARGET_SHA = "5d5742cbcc50f19c99c356bc7e085acaa5f4665d"
TOOLCHAIN = "1.90.0"
EXECSURFACE_VERSION = "0.1.0-alpha.2"
EXECSURFACE_SOURCE_SHA = "c6cabf1b4d1399898b9643a7fce011664aac0918"
TIMEOUT_SECONDS = 180.0
PRIVACY_SENTINEL = "M9_PRIVATE_SENTINEL_8d6c173f_DO_NOT_PERSIST"


class CaseFailure(RuntimeError):
    pass


def dump(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def run_quiet(argv: list[str], *, cwd: Path, env: dict[str, str], timeout: float) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        argv,
        cwd=cwd,
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        timeout=timeout,
        check=False,
    )


def host_facts() -> dict[str, Any]:
    cpu = "unknown"
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8", errors="replace").splitlines():
            if line.lower().startswith("model name") and ":" in line:
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass

    os_name = platform.system()
    try:
        os_release = {}
        for line in Path("/etc/os-release").read_text(encoding="utf-8", errors="replace").splitlines():
            if "=" in line:
                key, value = line.split("=", 1)
                os_release[key] = value.strip().strip('"')
        os_name = f"{os_release.get('NAME', os_name)} {os_release.get('VERSION_ID', '')}".strip()
    except OSError:
        pass

    return {
        "os": os_name,
        "kernel": platform.release(),
        "architecture": platform.machine(),
        "cpu_description": cpu,
        "runner_image": os.environ.get("ImageOS") or os.environ.get("RUNNER_IMAGE"),
    }


def main() -> int:
    repo_root = Path(os.environ.get("GITHUB_WORKSPACE", ".")).resolve()
    target_root = repo_root / "external" / "just"
    out = repo_root / "m9-evidence" / "just-post-freeze"
    raw = out / "raw"
    out.mkdir(parents=True, exist_ok=True)
    raw.mkdir(parents=True, exist_ok=True)

    host = host_facts()
    dump(
        out / "run-metadata.json",
        {
            "evidence_id": "M9-ZC-JUST-POSTFREEZE-001",
            "target_repository": TARGET_REPOSITORY,
            "target_sha": TARGET_SHA,
            "toolchain": TOOLCHAIN,
            "execsurface_version": EXECSURFACE_VERSION,
            "execsurface_source_sha": EXECSURFACE_SOURCE_SHA,
            "timeout_seconds": TIMEOUT_SECONDS,
            "sample_plan": {"warmups_per_mode": 3, "measured_per_mode": 15},
            "measured_pair_order": "direct,ptrace then ptrace,direct alternating by pair",
            "host": host,
        },
    )

    env = os.environ.copy()
    env["M9_PRIVATE_SENTINEL"] = PRIVACY_SENTINEL
    env["CARGO_TERM_COLOR"] = "never"

    install_root = Path(os.environ.get("RUNNER_TEMP", str(repo_root / ".m9-tmp"))) / "execsurface-m9-just"
    exe = install_root / "bin" / "execsurface"

    outcome = {
        "installation": "NOT_RUN",
        "doctor": "NOT_RUN",
        "baseline": "NOT_RUN",
        "rerun_check": "NOT_RUN",
        "drift_case": "NOT_RUN",
        "verdict": None,
        "comparable": None,
        "complete": None,
        "operational_error": None,
        "false_positive_reported": False,
        "suspected_false_negative": False,
        "usability_friction": None,
    }
    failures: list[dict[str, Any]] = []
    performance: dict[str, Any] | None = None
    failure_message: str | None = None

    def fail(kind: str, message: str) -> None:
        nonlocal failure_message
        failures.append({"kind": kind, "description": message, "retained": True})
        if failure_message is None:
            failure_message = message
        outcome["operational_error"] = failure_message
        outcome["comparable"] = False
        outcome["complete"] = False

    # Public installation path. Preserve failure; do not fall back to repository build.
    try:
        cp = run_quiet(
            [
                "cargo",
                f"+{TOOLCHAIN}",
                "install",
                "execsurface",
                "--version",
                f"={EXECSURFACE_VERSION}",
                "--locked",
                "--root",
                str(install_root),
            ],
            cwd=repo_root,
            env=env,
            timeout=900.0,
        )
    except subprocess.TimeoutExpired:
        cp = None
        outcome["installation"] = "FAIL"
        fail("installation_timeout", "public crates.io installation exceeded frozen 900s setup timeout")
    else:
        if cp.returncode == 0 and exe.is_file():
            outcome["installation"] = "PASS"
        else:
            outcome["installation"] = "FAIL"
            fail("installation_failure", f"public crates.io installation failed rc={cp.returncode}")

    if outcome["installation"] == "PASS":
        version_cp = subprocess.run([str(exe), "--version"], cwd=repo_root, env=env, text=True, capture_output=True, check=False)
        version_text = (version_cp.stdout + version_cp.stderr).strip()
        if version_cp.returncode != 0 or EXECSURFACE_VERSION not in version_text:
            outcome["installation"] = "FAIL"
            fail("version_mismatch", f"installed binary version check failed: {version_text[:300]}")

    if outcome["installation"] == "PASS":
        doctor_cp = subprocess.run([str(exe), "doctor"], cwd=repo_root, env=env, text=True, capture_output=True, check=False)
        dump(out / "doctor-summary.json", {"returncode": doctor_cp.returncode, "passed": doctor_cp.returncode == 0})
        if doctor_cp.returncode == 0:
            outcome["doctor"] = "PASS"
        else:
            outcome["doctor"] = "FAIL"
            fail("doctor_failure", f"execsurface doctor failed rc={doctor_cp.returncode}")

    target = [
        "/bin/bash",
        "-lc",
        f"cd {target_root} && cargo +{TOOLCHAIN} test --all >/dev/null 2>&1",
    ]

    # Direct upstream reproduction is a prerequisite and also warms build caches.
    if failure_message is None:
        started = time.perf_counter_ns()
        try:
            upstream_cp = run_quiet(target, cwd=repo_root, env=env, timeout=TIMEOUT_SECONDS)
            upstream_ms = (time.perf_counter_ns() - started) / 1_000_000
            dump(out / "upstream-reproduction.json", {"returncode": upstream_cp.returncode, "elapsed_ms": upstream_ms})
            if upstream_cp.returncode != 0:
                fail("upstream_failure", f"pinned upstream workload failed directly rc={upstream_cp.returncode}")
        except subprocess.TimeoutExpired:
            upstream_ms = (time.perf_counter_ns() - started) / 1_000_000
            dump(out / "upstream-reproduction.json", {"timeout": True, "elapsed_ms": upstream_ms})
            fail("upstream_timeout", f"pinned upstream workload exceeded frozen {TIMEOUT_SECONDS}s timeout")

    warm_direct: list[float] = []
    warm_ptrace: list[float] = []
    measured_direct: list[float] = []
    measured_ptrace: list[float] = []
    measurement_order: list[str] = []

    def timed_direct(label: str) -> float:
        started = time.perf_counter_ns()
        try:
            cp = run_quiet(target, cwd=repo_root, env=env, timeout=TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired as exc:
            raise CaseFailure(f"direct {label} exceeded frozen {TIMEOUT_SECONDS}s timeout") from exc
        elapsed = (time.perf_counter_ns() - started) / 1_000_000
        if cp.returncode != 0:
            raise CaseFailure(f"direct {label} failed rc={cp.returncode}")
        return elapsed

    observe_index = 0

    def timed_observe(label: str) -> float:
        nonlocal observe_index
        observe_index += 1
        report = raw / f"observe-{observe_index:02d}-{label}.json"
        stderr_path = raw / f"observe-{observe_index:02d}-{label}.stderr.txt"
        started = time.perf_counter_ns()
        try:
            with report.open("w", encoding="utf-8") as stdout_fh, stderr_path.open("w", encoding="utf-8") as stderr_fh:
                cp = subprocess.run(
                    [str(exe), "observe", "--", *target],
                    cwd=repo_root,
                    env=env,
                    stdout=stdout_fh,
                    stderr=stderr_fh,
                    timeout=TIMEOUT_SECONDS,
                    check=False,
                    text=True,
                )
        except subprocess.TimeoutExpired as exc:
            elapsed = (time.perf_counter_ns() - started) / 1_000_000
            raise CaseFailure(f"ptrace observe {label} exceeded frozen {TIMEOUT_SECONDS}s timeout after {elapsed:.3f}ms") from exc
        elapsed = (time.perf_counter_ns() - started) / 1_000_000
        if cp.returncode != 0:
            err = stderr_path.read_text(encoding="utf-8", errors="replace").strip()
            raise CaseFailure(f"ptrace observe {label} failed rc={cp.returncode}: {err[:500]}")
        try:
            data = json.loads(report.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            raise CaseFailure(f"ptrace observe {label} returned invalid JSON") from exc
        if data.get("complete") is not True:
            raise CaseFailure(f"ptrace observe {label} incomplete: {data.get('warnings')}")
        target_outcome = data.get("outcome", {})
        if target_outcome.get("exit_code") != 0 or target_outcome.get("signal") is not None:
            raise CaseFailure(f"ptrace observe {label} target outcome not clean: {target_outcome}")
        return elapsed

    if failure_message is None:
        try:
            for i in range(3):
                warm_direct.append(timed_direct(f"warmup-{i + 1}"))
            for i in range(3):
                warm_ptrace.append(timed_observe(f"warmup-{i + 1}"))

            for pair in range(15):
                if pair % 2 == 0:
                    measured_direct.append(timed_direct(f"measured-{len(measured_direct) + 1}"))
                    measurement_order.append("direct")
                    measured_ptrace.append(timed_observe(f"measured-{len(measured_ptrace) + 1}"))
                    measurement_order.append("ptrace")
                else:
                    measured_ptrace.append(timed_observe(f"measured-{len(measured_ptrace) + 1}"))
                    measurement_order.append("ptrace")
                    measured_direct.append(timed_direct(f"measured-{len(measured_direct) + 1}"))
                    measurement_order.append("direct")
        except CaseFailure as exc:
            fail("observer_or_measurement_failure", str(exc))

    dump(
        out / "performance-raw.json",
        {
            "warmup_direct_ms": warm_direct,
            "warmup_ptrace_ms": warm_ptrace,
            "measured_direct_ms": measured_direct,
            "measured_ptrace_ms": measured_ptrace,
            "measurement_order": measurement_order,
            "timeout_seconds": TIMEOUT_SECONDS,
            "complete_3x15": (
                len(warm_direct) == 3
                and len(warm_ptrace) == 3
                and len(measured_direct) == 15
                and len(measured_ptrace) == 15
            ),
        },
    )

    if failure_message is None:
        d = float(statistics.median(measured_direct))
        p = float(statistics.median(measured_ptrace))
        overhead = p - d
        ratio = p / d
        trigger = (d >= 100.0 and ratio > 2.0) or overhead > 500.0
        performance = {
            "warmup_direct_ms": warm_direct,
            "warmup_ptrace_ms": warm_ptrace,
            "measured_direct_ms": measured_direct,
            "measured_ptrace_ms": measured_ptrace,
            "median_direct_ms": d,
            "median_ptrace_ms": p,
            "median_absolute_overhead_ms": overhead,
            "slowdown_ratio": ratio,
            "timeout_seconds": TIMEOUT_SECONDS,
            "m65_trigger_crossed": trigger,
            "excluded_samples": [],
        }

    baseline = out / "just.lock.json"
    if failure_message is None:
        learn_cp = run_quiet(
            [str(exe), "learn", "--output", str(baseline), "--", *target],
            cwd=repo_root,
            env=env,
            timeout=TIMEOUT_SECONDS,
        )
        if learn_cp.returncode == 0 and baseline.is_file():
            outcome["baseline"] = "PASS"
        else:
            outcome["baseline"] = "FAIL"
            fail("baseline_failure", f"learn failed rc={learn_cp.returncode}")

    if failure_message is None:
        no_drift_ok = True
        for n in (1, 2):
            report = out / f"no-drift-{n}.json"
            markdown = out / f"no-drift-{n}.md"
            cp = run_quiet(
                [
                    str(exe),
                    "check",
                    "--baseline",
                    str(baseline),
                    "--json-output",
                    str(report),
                    "--markdown-output",
                    str(markdown),
                    "--",
                    *target,
                ],
                cwd=repo_root,
                env=env,
                timeout=TIMEOUT_SECONDS,
            )
            if cp.returncode != 0 or not report.is_file():
                no_drift_ok = False
                fail("rerun_check_failure", f"no-drift check {n} failed rc={cp.returncode}")
                break
            data = json.loads(report.read_text(encoding="utf-8"))
            if data.get("verdict") != "pass" or data.get("findings") != []:
                no_drift_ok = False
                fail("rerun_check_semantic_failure", f"no-drift check {n} was not clean PASS")
                break
        outcome["rerun_check"] = "PASS" if no_drift_ok else "FAIL"

    if failure_message is None:
        drift_report = out / "controlled-drift.json"
        drift_md = out / "controlled-drift.md"
        drift_target = [
            "/bin/bash",
            "-lc",
            f"cd {target_root} && cargo +{TOOLCHAIN} test --all >/dev/null 2>&1; /usr/bin/sha256sum Cargo.toml >/dev/null",
        ]
        try:
            cp = subprocess.run(
                [
                    str(exe),
                    "check",
                    "--baseline",
                    str(baseline),
                    "--json-output",
                    str(drift_report),
                    "--markdown-output",
                    str(drift_md),
                    "--",
                    *drift_target,
                ],
                cwd=repo_root,
                env=env,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                timeout=TIMEOUT_SECONDS,
                check=False,
            )
        except subprocess.TimeoutExpired:
            outcome["drift_case"] = "FAIL"
            fail("drift_timeout", f"controlled drift case exceeded frozen {TIMEOUT_SECONDS}s timeout")
        else:
            drift_ok = cp.returncode == 10 and drift_report.is_file()
            if drift_ok:
                data = json.loads(drift_report.read_text(encoding="utf-8"))
                hits = []
                for finding in data.get("findings", []):
                    if finding.get("change") != "added" or finding.get("effect_kind") != "process_exec":
                        continue
                    executable = finding.get("evidence", {}).get("effect", {}).get("executable", {})
                    if executable.get("family") == "sha256sum":
                        hits.append(finding)
                drift_ok = data.get("verdict") == "review" and bool(hits)
            if drift_ok:
                outcome["drift_case"] = "PASS"
            else:
                outcome["drift_case"] = "FAIL"
                fail("drift_semantic_failure", f"controlled drift case did not produce expected REVIEW/sha256sum evidence rc={cp.returncode}")

    # Privacy sentinel is intentionally a harmless canary, not a secret.
    privacy_leak_files: list[str] = []
    for path in sorted(out.rglob("*")):
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if PRIVACY_SENTINEL in text:
            privacy_leak_files.append(str(path.relative_to(out)))
    if privacy_leak_files:
        fail("privacy_sentinel_leak", f"privacy sentinel persisted in: {privacy_leak_files}")

    if failure_message is None:
        outcome["verdict"] = "pass"
        outcome["comparable"] = True
        outcome["complete"] = True

    # Digest raw evidence before creating the canonical record, avoiding a digest cycle.
    artifacts = []
    for path in sorted(out.rglob("*")):
        if not path.is_file() or path.name in {"evidence-record.json", "SHA256SUMS"}:
            continue
        artifacts.append(
            {
                "name": path.name,
                "sha256": sha256(path),
                "location": str(path.relative_to(repo_root)),
            }
        )

    record = {
        "schema_version": "m9-evidence-v1",
        "evidence_id": "M9-ZC-JUST-POSTFREEZE-001",
        "timestamp_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "independence_class": "ZERO_CONTACT_EXTERNAL_REPRO",
        "claim_status": "COMPUTATIONAL_EVIDENCE" if failure_message is None else "PARTIAL",
        "external_reference": TARGET_REPOSITORY,
        "provenance": {
            "initiated_by": "AETHER_X",
            "executed_by": "AETHER_X",
            "aether_x_material_involvement": True,
            "third_party_attestation": None,
        },
        "project": {
            "name": "casey/just",
            "repository": TARGET_REPOSITORY,
            "revision": TARGET_SHA,
            "modified_by_aether_x": False,
        },
        "execsurface": {
            "version": EXECSURFACE_VERSION,
            "source_sha": EXECSURFACE_SOURCE_SHA,
            "install_path": "cargo install execsurface --version =0.1.0-alpha.2 --locked",
            "backend": "ptrace",
        },
        "host": host,
        "workflow": {
            "command": f"cargo +{TOOLCHAIN} test --all",
            "baseline_attempted": outcome["baseline"] != "NOT_RUN",
            "rerun_check_attempted": outcome["rerun_check"] != "NOT_RUN",
            "drift_case_attempted": outcome["drift_case"] != "NOT_RUN",
            "notes": "Post-M9.0 canonical run; 3 warmups/mode, 15 measured/mode, alternating direct/ptrace pair order, fixed 180s timeout.",
        },
        "outcome": outcome,
        "privacy": {
            "metadata_only_confirmed": not privacy_leak_files,
            "prohibited_sensitive_payload_collected": bool(privacy_leak_files),
            "notes": None if not privacy_leak_files else f"sentinel persisted in {privacy_leak_files}",
        },
        "performance": performance,
        "artifacts": artifacts,
        "failures_exclusions": failures,
        "reproduction": {
            "instructions": (
                f"Checkout casey/just at {TARGET_SHA}; install Rust {TOOLCHAIN}; install ExecSurface "
                f"{EXECSURFACE_VERSION} from crates.io with --locked; run this repository's M9 just workflow."
            ),
            "expected_scope": "Exact pinned casey/just workload on the recorded Linux x86_64 GitHub-hosted runner class only.",
        },
    }
    dump(out / "evidence-record.json", record)

    sums = []
    for path in sorted(out.rglob("*")):
        if path.is_file() and path.name != "SHA256SUMS":
            sums.append(f"{sha256(path)}  {path.relative_to(out)}")
    (out / "SHA256SUMS").write_text("\n".join(sums) + "\n", encoding="utf-8")

    if failure_message is not None:
        print(f"M9_JUST_CASE_PARTIAL: {failure_message}", file=sys.stderr)
        return 1

    print("M9_JUST_CASE_COMPUTATIONAL_EVIDENCE_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
