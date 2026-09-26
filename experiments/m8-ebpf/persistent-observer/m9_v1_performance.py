#!/usr/bin/env python3
import argparse
import datetime as dt
import hashlib
import json
import math
import os
import platform
import statistics
import subprocess
import tempfile
import time
from pathlib import Path

MODES = ["direct", "ptrace", "libbpf_per_invocation", "persistent_two_session"]
ROTATIONS = [MODES[i:] + MODES[:i] for i in range(len(MODES))]
HARD_EBPF_INCOMPLETE = {
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_decode",
    "incomplete_collector",
    "incomplete_lifecycle",
}


def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--cli", required=True)
    p.add_argument("--collector", required=True)
    p.add_argument("--persistent", required=True)
    p.add_argument("--launcher", required=True)
    p.add_argument("--workdir", required=True)
    p.add_argument("--script", required=True)
    p.add_argument("--workload-id", required=True)
    p.add_argument("--upstream", required=True)
    p.add_argument("--upstream-sha", required=True)
    p.add_argument("--commit-sha", required=True)
    p.add_argument("--output", required=True)
    p.add_argument("--warmups", type=int, default=3)
    p.add_argument("--samples", type=int, default=15)
    args = p.parse_args()
    if os.geteuid() != 0:
        p.error("M9-V1 harness must run as root so all timed modes share one privilege context")
    if args.warmups != 3 or args.samples != 15:
        p.error("M9-V1 frozen protocol requires exactly 3 warmups and 15 measured samples")
    for value in (args.cli, args.collector, args.persistent, args.launcher):
        if not Path(value).is_file():
            p.error(f"required executable missing: {value}")
    if not Path(args.workdir).is_dir():
        p.error(f"workdir missing: {args.workdir}")
    return args


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def os_release():
    result = {}
    try:
        for line in Path("/etc/os-release").read_text().splitlines():
            if "=" in line:
                k, v = line.split("=", 1)
                result[k] = v.strip().strip('"')
    except OSError:
        pass
    return result


def p90(values):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(0.9 * len(ordered)) - 1)]


def summarize(values):
    median = statistics.median(values)
    return {
        "count": len(values),
        "median_ms": median,
        "min_ms": min(values),
        "max_ms": max(values),
        "mad_ms": statistics.median(abs(x - median) for x in values),
        "p90_ms": p90(values),
    }


def run_process(command, cwd=None, env=None):
    started = time.perf_counter_ns()
    proc = subprocess.run(command, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stopped = time.perf_counter_ns()
    return proc, (stopped - started) / 1_000_000.0


def base_sample(mode, proc, wall_ms):
    return {
        "mode": mode,
        "wall_ms": wall_ms,
        "returncode": proc.returncode,
        "stdout_bytes": len(proc.stdout),
        "stderr_bytes": len(proc.stderr),
        "clean": proc.returncode == 0,
    }


def invoke(mode, args):
    with tempfile.TemporaryDirectory(prefix="m9-v1-") as td:
        report_path = Path(td) / "report.json"
        cwd = args.workdir
        if mode == "direct":
            command = ["/bin/bash", "-lc", args.script]
            proc, wall = run_process(command, cwd=cwd)
            sample = base_sample(mode, proc, wall)
            return sample

        if mode == "ptrace":
            command = [args.cli, "observe", "--", "/bin/bash", "-lc", args.script]
            proc, wall = run_process(command, cwd=cwd)
            sample = base_sample(mode, proc, wall)
            try:
                report = json.loads(proc.stdout)
            except json.JSONDecodeError as exc:
                sample["clean"] = False
                sample["error"] = f"ptrace stdout invalid JSON: {exc}"
                return sample
            outcome = report.get("outcome", {})
            sample.update({
                "ptrace_complete": report.get("complete"),
                "ptrace_event_count": len(report.get("events", [])),
                "outcome": outcome,
            })
            sample["clean"] = (
                sample["clean"]
                and report.get("complete") is True
                and outcome.get("exit_code") == 0
                and outcome.get("signal") is None
            )
            return sample

        if mode == "libbpf_per_invocation":
            command = [args.collector, "--report", str(report_path), "--", "/bin/bash", "-lc", args.script]
            proc, wall = run_process(command, cwd=cwd)
            sample = base_sample(mode, proc, wall)
            if not report_path.is_file():
                sample["clean"] = False
                sample["error"] = "libbpf report missing"
                return sample
            report = json.loads(report_path.read_text())
            outcome = report.get("outcome", {})
            sample.update({
                "completeness": report.get("completeness"),
                "dropped_events": report.get("dropped_events"),
                "lifecycle_drain_complete": report.get("lifecycle_drain_complete"),
                "collector_failure": report.get("collector_failure"),
                "event_count": len(report.get("events", [])),
                "outcome": outcome,
            })
            sample["clean"] = (
                sample["clean"]
                and report.get("completeness") not in HARD_EBPF_INCOMPLETE
                and report.get("dropped_events") == 0
                and report.get("lifecycle_drain_complete") is True
                and report.get("collector_failure") is None
                and outcome.get("exit_code") == 0
                and outcome.get("signal") is None
            )
            return sample

        if mode == "persistent_two_session":
            env = os.environ.copy()
            env["M9_V1_WORKDIR"] = args.workdir
            env["M9_V1_SCRIPT"] = args.script
            command = [args.persistent, "--report", str(report_path), "--fixture", args.launcher]
            proc, wall = run_process(command, env=env)
            sample = base_sample(mode, proc, wall)
            if not report_path.is_file():
                sample["clean"] = False
                sample["error"] = "persistent report missing"
                return sample
            report = json.loads(report_path.read_text())
            sessions = report.get("sessions", [])
            authority = report.get("authority", {})
            session_health = []
            for session in sessions:
                outcome = session.get("outcome", {})
                healthy = (
                    session.get("clean_for_m8_7a") is True
                    and session.get("lifecycle_drain_complete") is True
                    and session.get("active_remaining") == 0
                    and session.get("stale_epoch_events") == 0
                    and session.get("integrity_errors") == 0
                    and session.get("decode_error_delta") == 0
                    and session.get("producer_drop_delta") == 0
                    and session.get("routing_error_delta") == 0
                    and session.get("event_limit_hit") is False
                    and session.get("membership_map_empty") is True
                    and session.get("pending_mechanism_map_empty") is True
                    and session.get("exec_count", 0) >= 1
                    and session.get("exit_count", 0) >= 1
                    and outcome.get("exit_code") == 0
                    and outcome.get("signal") is None
                )
                session_health.append(healthy)
            sample.update({
                "session_count": len(sessions),
                "amortized_wall_per_session_ms": wall / 2.0,
                "internal_session_ms": [s.get("elapsed_ms") for s in sessions],
                "session_health": session_health,
                "session_event_counts": [s.get("event_count") for s in sessions],
                "session_spawn_counts": [s.get("spawn_count") for s in sessions],
                "session_exec_counts": [s.get("exec_count") for s in sessions],
                "session_exit_counts": [s.get("exit_count") for s in sessions],
                "one_time_lifecycle": report.get("one_time_lifecycle"),
                "unexpected_without_session": report.get("unexpected_without_session"),
                "decode_errors_total": report.get("decode_errors_total"),
                "final_membership_map_empty": report.get("final_membership_map_empty"),
                "final_pending_mechanism_map_empty": report.get("final_pending_mechanism_map_empty"),
                "authority": authority,
            })
            sample["clean"] = (
                sample["clean"]
                and len(sessions) == 2
                and all(session_health)
                and report.get("unexpected_without_session") == 0
                and report.get("decode_errors_total") == 0
                and report.get("final_membership_map_empty") is True
                and report.get("final_pending_mechanism_map_empty") is True
                and authority.get("ptrace_correctness_reference") is True
                and authority.get("full_surface_comparable") is False
                and authority.get("ebpf_pass_authorized") is False
                and authority.get("product_integration_authorized") is False
            )
            return sample

        raise ValueError(mode)


def main():
    args = parse_args()
    evidence = {
        "schema_version": 1,
        "experiment": "M9-V1 same-workload eBPF value experiment",
        "claim_scope": "research-only exact-host same-workload performance; no product or authority change",
        "workload": {
            "id": args.workload_id,
            "upstream": args.upstream,
            "upstream_sha": args.upstream_sha,
            "workdir": args.workdir,
            "script": args.script,
        },
        "host": {
            "utc_timestamp": dt.datetime.now(dt.timezone.utc).isoformat(),
            "repository_commit_sha": args.commit_sha,
            "runner_name": os.environ.get("RUNNER_NAME"),
            "runner_os": os.environ.get("RUNNER_OS"),
            "runner_arch": os.environ.get("RUNNER_ARCH"),
            "os_release": os_release(),
            "kernel_release": platform.release(),
            "machine": platform.machine(),
            "logical_cpu_count": os.cpu_count(),
            "effective_uid": os.geteuid(),
            "btf_vmlinux_readable": os.access("/sys/kernel/btf/vmlinux", os.R_OK),
            "binary_sha256": {
                "execsurface": sha256_file(args.cli),
                "per_invocation_collector": sha256_file(args.collector),
                "persistent_observer": sha256_file(args.persistent),
                "barrier_launcher": sha256_file(args.launcher),
            },
        },
        "protocol": {
            "warmups_per_mode": 3,
            "timed_samples_per_mode": 15,
            "persistent_sessions_per_invocation": 2,
            "timed_mode_order": "rotating four-order schedule",
            "outlier_policy": "none removed",
            "privilege_context": "all timed modes run from one root harness",
            "value_gate": "eligible eBPF end-to-end median lower than same-job ptrace median",
        },
        "authority": {
            "ptrace_correctness_reference": True,
            "full_surface_comparable": False,
            "ebpf_pass_authorized": False,
            "product_integration_authorized": False,
        },
        "warmups": [],
        "samples": {m: [] for m in MODES},
    }

    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)

    for mode in MODES:
        for i in range(3):
            sample = invoke(mode, args)
            sample["warmup"] = i
            evidence["warmups"].append(sample)
            if not sample["clean"]:
                output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
                raise RuntimeError(f"warmup failed mode={mode}: {sample}")

    for i in range(15):
        for mode in ROTATIONS[i % len(ROTATIONS)]:
            sample = invoke(mode, args)
            sample["repetition"] = i
            evidence["samples"][mode].append(sample)

    for mode, rows in evidence["samples"].items():
        if len(rows) != 15 or not all(r["clean"] for r in rows):
            output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
            raise RuntimeError(f"timed health gate failed mode={mode}")

    summary = {
        "direct": summarize([r["wall_ms"] for r in evidence["samples"]["direct"]]),
        "ptrace": summarize([r["wall_ms"] for r in evidence["samples"]["ptrace"]]),
        "libbpf_per_invocation": summarize([r["wall_ms"] for r in evidence["samples"]["libbpf_per_invocation"]]),
        "persistent_two_session_amortized": summarize([r["amortized_wall_per_session_ms"] for r in evidence["samples"]["persistent_two_session"]]),
        "persistent_internal_session": summarize([
            ms
            for r in evidence["samples"]["persistent_two_session"]
            for ms in r["internal_session_ms"]
        ]),
    }
    evidence["summary"] = summary
    ptrace = summary["ptrace"]["median_ms"]
    per = summary["libbpf_per_invocation"]["median_ms"]
    persistent = summary["persistent_two_session_amortized"]["median_ms"]
    evidence["ratios"] = {
        "libbpf_per_invocation_over_ptrace": per / ptrace,
        "persistent_amortized_over_ptrace": persistent / ptrace,
    }
    evidence["value_signal"] = {
        "libbpf_per_invocation_faster_than_ptrace": per < ptrace,
        "persistent_amortized_faster_than_ptrace": persistent < ptrace,
        "any_ebpf_end_to_end_faster_than_ptrace": min(per, persistent) < ptrace,
    }
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")

    print("M9_V1_VALUE_EVIDENCE_PASS")
    for name, row in summary.items():
        print(name, "median_ms=", round(row["median_ms"], 6), "mad_ms=", round(row["mad_ms"], 6), "p90_ms=", round(row["p90_ms"], 6))
    print("ratios=", {k: round(v, 6) for k, v in evidence["ratios"].items()})
    print("value_signal=", evidence["value_signal"])


if __name__ == "__main__":
    main()
