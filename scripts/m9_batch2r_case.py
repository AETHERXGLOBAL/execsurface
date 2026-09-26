#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time

TIMEOUT = 120.0
CASES = {
    "ZC-04R-JUST-CLI": {
        "name": "just",
        "repo": "casey/just",
        "revision": "5d5742cbcc50f19c99c356bc7e085acaa5f4665d",
        "toolchain": "1.90.0",
        "build": ["cargo", "+1.90.0", "build", "--locked", "--release"],
        "baseline": "./target/release/just --list >/dev/null",
        "drift": "./target/release/just --list >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null",
    },
    "ZC-06R-RIPGREP-CLI": {
        "name": "ripgrep",
        "repo": "BurntSushi/ripgrep",
        "revision": "3fce3b5bb0236da2df6d99672afb8a719642eca7",
        "toolchain": "1.96.0",
        "build": ["cargo", "+1.96.0", "build", "--locked", "--release"],
        "baseline": "./target/release/rg --files . >/dev/null",
        "drift": "./target/release/rg --files . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null",
    },
    "ZC-07R-FD-CLI": {
        "name": "fd",
        "repo": "sharkdp/fd",
        "revision": "ce97e473ebaec49697c07daa50a7bc2b32f713d2",
        "toolchain": "1.90.0",
        "build": ["cargo", "+1.90.0", "build", "--locked", "--release", "--all-features"],
        "baseline": "./target/release/fd --hidden --type f --exclude target . >/dev/null",
        "drift": "./target/release/fd --hidden --type f --exclude target . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null",
    },
}


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def run(args, cwd: Path, stdout: Path | None = None, stderr: Path | None = None, timeout: float = TIMEOUT):
    out = stdout.open("wb") if stdout else subprocess.DEVNULL
    err = stderr.open("wb") if stderr else subprocess.DEVNULL
    try:
        return subprocess.run(args, cwd=cwd, stdout=out, stderr=err, timeout=timeout, check=False)
    finally:
        if stdout:
            out.close()
        if stderr:
            err.close()


def complete_observation(path: Path) -> bool:
    try:
        data = json.loads(path.read_text())
    except Exception:
        return False
    outcome = data.get("outcome", {})
    return data.get("complete") is True and outcome.get("exit_code") == 0 and outcome.get("signal") is None


def process_exec_sha256sum(data: dict) -> bool:
    for finding in data.get("findings", []):
        if finding.get("change") != "added" or finding.get("effect_kind") != "process_exec":
            continue
        family = finding.get("evidence", {}).get("effect", {}).get("executable", {}).get("family")
        if family == "sha256sum":
            return True
    return False


def main() -> int:
    if len(sys.argv) != 2 or sys.argv[1] not in CASES:
        raise SystemExit(f"usage: {sys.argv[0]} <{'|'.join(CASES)}>")
    case_id = sys.argv[1]
    case = CASES[case_id]
    repo_root = Path(os.environ["GITHUB_WORKSPACE"])
    source = repo_root / "external/project"
    work = Path(os.environ["RUNNER_TEMP"]) / f"m9-b2r-{case['name']}"
    evidence = repo_root / "m9-b2r-evidence" / case_id
    exe = repo_root / "target/release/execsurface"
    evidence.mkdir(parents=True, exist_ok=True)

    revision = subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip()
    if revision != case["revision"]:
        raise SystemExit(f"revision mismatch {revision} != {case['revision']}")
    if subprocess.check_output(["git", "-C", str(source), "status", "--porcelain"], text=True).strip():
        raise SystemExit("external checkout not clean before copy")

    if work.exists():
        shutil.rmtree(work)
    shutil.copytree(source, work, symlinks=True)
    if subprocess.check_output(["git", "-C", str(work), "rev-parse", "HEAD"], text=True).strip() != case["revision"]:
        raise SystemExit("copied checkout revision mismatch")

    state = {
        "setup": "NOT_RUN", "direct": "NOT_RUN", "doctor": "NOT_RUN", "baseline": "NOT_RUN",
        "checks": [], "stable": False, "drift": {"attempted": False}, "performance": "NOT_RUN",
        "operational_error": None,
    }
    errors: list[str] = []

    build = run(case["build"], work, evidence / "build.stdout", evidence / "build.stderr", 300.0)
    tracked = subprocess.check_output(["git", "-C", str(work), "status", "--porcelain", "--untracked-files=no"], text=True)
    (evidence / "tracked-status-after-build.txt").write_text(tracked)
    if build.returncode == 0 and not tracked.strip():
        state["setup"] = "PASS"
    else:
        state["setup"] = "FAIL"
        errors.append(f"external setup failed rc={build.returncode} tracked_dirty={bool(tracked.strip())}")

    baseline_script = case["baseline"]
    drift_script = case["drift"]
    baseline_cmd = ["/bin/bash", "-lc", baseline_script]
    drift_cmd = ["/bin/bash", "-lc", drift_script]

    if state["setup"] == "PASS":
        direct = run(baseline_cmd, work, evidence / "direct.stdout", evidence / "direct.stderr")
        state["direct"] = "PASS" if direct.returncode == 0 else "FAIL"
        if direct.returncode != 0:
            errors.append(f"direct baseline failed rc={direct.returncode}")

        doctor = run([str(exe), "doctor"], work, evidence / "doctor.stdout", evidence / "doctor.stderr")
        state["doctor"] = "PASS" if doctor.returncode == 0 else "FAIL"
        if doctor.returncode != 0:
            errors.append(f"doctor failed rc={doctor.returncode}")

    baseline = evidence / "baseline.lock.json"
    if state["direct"] == "PASS" and state["doctor"] == "PASS":
        learned = run([str(exe), "learn", "--output", str(baseline), "--"] + baseline_cmd, work,
                      evidence / "learn.stdout", evidence / "learn.stderr")
        state["baseline"] = "PASS" if learned.returncode == 0 else "FAIL"
        if learned.returncode != 0:
            errors.append(f"learn failed rc={learned.returncode}")

    if state["baseline"] == "PASS":
        for n in (1, 2):
            jout = evidence / f"check-{n}.json"
            md = evidence / f"check-{n}.md"
            proc = run([str(exe), "check", "--baseline", str(baseline), "--json-output", str(jout),
                        "--markdown-output", str(md), "--"] + baseline_cmd, work,
                       evidence / f"check-{n}.stdout", evidence / f"check-{n}.stderr")
            data = json.loads(jout.read_text()) if jout.exists() else {}
            state["checks"].append({"rc": proc.returncode, "verdict": data.get("verdict"), "findings": len(data.get("findings", []))})
        state["stable"] = all(x["rc"] == 0 and x["verdict"] == "pass" and x["findings"] == 0 for x in state["checks"])
        if not state["stable"]:
            errors.append("unchanged checks were not two zero-finding PASS results")

    if state["stable"]:
        jout = evidence / "drift.json"
        md = evidence / "drift.md"
        proc = run([str(exe), "check", "--baseline", str(baseline), "--json-output", str(jout),
                    "--markdown-output", str(md), "--"] + drift_cmd, work,
                   evidence / "drift.stdout", evidence / "drift.stderr")
        data = json.loads(jout.read_text()) if jout.exists() else {}
        visible = process_exec_sha256sum(data)
        state["drift"] = {"attempted": True, "rc": proc.returncode, "verdict": data.get("verdict"),
                          "findings": len(data.get("findings", [])), "sha256sum_visible": visible}
        if proc.returncode == 0 or data.get("verdict") not in {"review", "block"} or not visible:
            errors.append("controlled drift did not produce REVIEW/BLOCK with added sha256sum process_exec")

    preperf = evidence / "preperf-observe.json"
    if state["stable"] and state["drift"].get("sha256sum_visible") is True:
        proc = run([str(exe), "observe", "--"] + baseline_cmd, work, preperf, evidence / "preperf-observe.stderr")
        if proc.returncode != 0 or not complete_observation(preperf):
            errors.append("pre-performance observation incomplete or failed")
        else:
            def direct_ms() -> float:
                t0 = time.perf_counter_ns()
                p = run(baseline_cmd, work)
                ms = (time.perf_counter_ns() - t0) / 1_000_000
                if p.returncode != 0:
                    raise RuntimeError(f"direct timing rc={p.returncode}")
                return ms

            seq = 0
            def ptrace_ms() -> float:
                nonlocal seq
                seq += 1
                jout = evidence / f"perf-observe-{seq}.json"
                t0 = time.perf_counter_ns()
                p = run([str(exe), "observe", "--"] + baseline_cmd, work, jout, evidence / f"perf-observe-{seq}.stderr")
                ms = (time.perf_counter_ns() - t0) / 1_000_000
                if p.returncode != 0 or not complete_observation(jout):
                    raise RuntimeError("timed ptrace observation failed/incomplete")
                return ms

            try:
                wd = [direct_ms() for _ in range(3)]
                wp = [ptrace_ms() for _ in range(3)]
                d: list[float] = []
                p: list[float] = []
                for i in range(15):
                    if i % 2 == 0:
                        d.append(direct_ms()); p.append(ptrace_ms())
                    else:
                        p.append(ptrace_ms()); d.append(direct_ms())
                mdirect = float(statistics.median(d)); mptrace = float(statistics.median(p))
                overhead = mptrace - mdirect; ratio = mptrace / mdirect if mdirect > 0 else float("inf")
                state.update({
                    "performance": "PASS", "warmup_direct_ms": wd, "warmup_ptrace_ms": wp,
                    "measured_direct_ms": d, "measured_ptrace_ms": p, "median_direct_ms": mdirect,
                    "median_ptrace_ms": mptrace, "median_absolute_overhead_ms": overhead,
                    "slowdown_ratio": ratio,
                    "m65_trigger_crossed": ((mdirect >= 100.0 and ratio > 2.0) or overhead > 500.0),
                })
            except Exception as exc:
                state["performance"] = "FAIL"
                errors.append(str(exc))

    state["operational_error"] = "; ".join(errors) or None
    (evidence / "state.json").write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")

    raw_manifest = evidence / "RAW_SHA256SUMS"
    rows = []
    for path in sorted(p for p in evidence.iterdir() if p.is_file() and p.name not in {"RAW_SHA256SUMS", "evidence.json", "evidence.sha256", "validator.txt"}):
        rows.append(f"{sha256(path)}  {path.name}")
    raw_manifest.write_text("\n".join(rows) + "\n")

    accepted = (state["setup"] == "PASS" and state["direct"] == "PASS" and state["doctor"] == "PASS"
                and state["baseline"] == "PASS" and state["stable"] and state["drift"].get("sha256sum_visible") is True
                and state["performance"] == "PASS")
    perf = None
    if state["performance"] == "PASS":
        perf = {
            "warmup_direct_ms": state["warmup_direct_ms"], "warmup_ptrace_ms": state["warmup_ptrace_ms"],
            "measured_direct_ms": state["measured_direct_ms"], "measured_ptrace_ms": state["measured_ptrace_ms"],
            "median_direct_ms": state["median_direct_ms"], "median_ptrace_ms": state["median_ptrace_ms"],
            "median_absolute_overhead_ms": state["median_absolute_overhead_ms"], "slowdown_ratio": state["slowdown_ratio"],
            "timeout_seconds": TIMEOUT, "m65_trigger_crossed": state["m65_trigger_crossed"], "excluded_samples": [],
        }
    version = subprocess.check_output([str(exe), "--version"], text=True).strip()
    run_url = f"https://github.com/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
    failures = [] if accepted else [{"kind": "runtime_or_measurement_failure", "description": state["operational_error"] or "acceptance gate incomplete", "retained": True}]
    record = {
        "schema_version": "m9-evidence-v1", "evidence_id": f"M9-{case_id}-001",
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "independence_class": "ZERO_CONTACT_EXTERNAL_REPRO",
        "claim_status": "COMPUTATIONAL_EVIDENCE" if accepted else "PARTIAL",
        "external_reference": f"https://github.com/{case['repo']}/commit/{case['revision']}",
        "provenance": {"initiated_by": "AETHER_X", "executed_by": "AETHER_X", "aether_x_material_involvement": True, "third_party_attestation": None},
        "project": {"name": case["name"], "repository": f"https://github.com/{case['repo']}", "revision": case["revision"], "modified_by_aether_x": False},
        "execsurface": {"version": version, "source_sha": os.environ["GITHUB_SHA"], "install_path": "exact branch source build", "backend": "ptrace"},
        "host": {"os": "Ubuntu 24.04 GitHub-hosted runner", "kernel": subprocess.check_output(["uname", "-srmo"], text=True).strip(), "architecture": subprocess.check_output(["uname", "-m"], text=True).strip(), "cpu_description": subprocess.check_output("lscpu | sed -n 's/^Model name:[[:space:]]*//p' | head -1", shell=True, text=True).strip(), "runner_image": "ubuntu-24.04"},
        "workflow": {"command": f"/bin/bash -lc {baseline_script!r}", "baseline_attempted": state["baseline"] != "NOT_RUN", "rerun_check_attempted": bool(state["checks"]), "drift_case_attempted": bool(state["drift"].get("attempted")), "notes": "Batch 2R frozen harness repair. External checkout copied unchanged to RUNNER_TEMP; build is outside observation; root command remains /bin/bash -lc across baseline/check/drift/performance."},
        "outcome": {"installation": "PASS", "doctor": state["doctor"], "baseline": state["baseline"], "rerun_check": "PASS" if state["stable"] else "FAIL", "drift_case": "PASS" if state["drift"].get("sha256sum_visible") is True else ("NOT_RUN" if not state["drift"].get("attempted") else "FAIL"), "verdict": "PASS" if accepted else None, "comparable": accepted, "complete": accepted, "operational_error": state["operational_error"], "false_positive_reported": False, "suspected_false_negative": state["drift"].get("attempted") is True and state["drift"].get("sha256sum_visible") is not True, "usability_friction": None},
        "privacy": {"metadata_only_confirmed": True, "prohibited_sensitive_payload_collected": False, "notes": "No file contents, environment values, stdin contents, network payloads, secrets, credentials, or unrestricted argv collected into accepted evidence."},
        "performance": perf,
        "artifacts": [{"name": "raw evidence SHA-256 manifest", "sha256": sha256(raw_manifest), "location": run_url}],
        "failures_exclusions": failures,
        "reproduction": {"instructions": f"Checkout {case['repo']}@{case['revision']}; copy unchanged checkout outside ExecSurface workspace; build with frozen toolchain; use /bin/bash -lc root for baseline/check/drift/performance; run two unchanged checks, controlled sha256sum drift, and exact 3+15 timing protocol.", "expected_scope": "Exact pinned GitHub-hosted zero-contact compatibility/performance evidence; not independent adoption and not a universal Linux claim."},
    }
    (evidence / "evidence.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    (evidence / "evidence.sha256").write_text(f"{sha256(evidence / 'evidence.json')}  evidence.json\n")
    print(json.dumps({"case": case_id, "accepted": accepted, "state": state}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
